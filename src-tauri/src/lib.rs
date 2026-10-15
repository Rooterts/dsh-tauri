use std::{
    io::{BufRead, BufReader},
    path::PathBuf,
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
};

use serde::Serialize;
use tauri::{webview::WebviewWindowBuilder, AppHandle, Manager, RunEvent, WebviewUrl};

const DSH_PACKAGE: &str = "@deepseek-ai/dsh";

#[derive(Default)]
struct DshProcess(Mutex<Option<Child>>);

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DshStatus {
    available: bool,
    command: Option<String>,
    managed: bool,
    node_available: bool,
}

fn data_root() -> Result<PathBuf, String> {
    if let Some(path) = std::env::var_os("XDG_DATA_HOME") {
        return Ok(PathBuf::from(path).join("dsh-desktop"));
    }
    std::env::var_os("HOME")
        .map(|home| PathBuf::from(home).join(".local/share/dsh-desktop"))
        .ok_or_else(|| "No se pudo localizar el directorio HOME".into())
}

fn managed_dsh() -> Result<PathBuf, String> {
    Ok(data_root()?.join("runtime/node_modules/.bin/dsh"))
}

fn find_dsh() -> Option<(String, bool)> {
    if let Ok(path) = std::env::var("DSH_BIN") {
        if !path.is_empty() && PathBuf::from(&path).exists() {
            return Some((path, true));
        }
    }
    if let Ok(output) = Command::new("dsh").arg("--version").output() {
        if output.status.success() {
            return Some(("dsh".into(), false));
        }
    }
    managed_dsh()
        .ok()
        .filter(|path| path.exists())
        .map(|path| (path.to_string_lossy().into_owned(), true))
}

fn node_available() -> bool {
    Command::new("node")
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[tauri::command]
fn dsh_status() -> DshStatus {
    let command = find_dsh();
    DshStatus {
        available: command.is_some(),
        managed: command
            .as_ref()
            .map(|(_, managed)| *managed)
            .unwrap_or(false),
        command: command.map(|(path, _)| path),
        node_available: node_available(),
    }
}

#[tauri::command]
fn list_dsh_versions(include_prerelease: bool) -> Result<Vec<String>, String> {
    let output = Command::new("npm")
        .args(["view", DSH_PACKAGE, "versions", "--json"])
        .output()
        .map_err(|error| format!("No se pudo ejecutar npm: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).trim().to_string());
    }
    let mut versions: Vec<String> = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("Respuesta de npm no válida: {error}"))?;
    if !include_prerelease {
        versions.retain(|version| !version.contains('-'));
    }
    versions.reverse();
    Ok(versions)
}

#[tauri::command]
fn install_dsh(version: String) -> Result<(), String> {
    if version.is_empty()
        || version.len() > 80
        || version
            .chars()
            .any(|character| !(character.is_ascii_alphanumeric() || ".-_+".contains(character)))
    {
        return Err("Versión de DSH no válida".into());
    }
    if !node_available() {
        return Err("Para instalar DSH se necesita Node.js y npm disponibles en PATH".into());
    }
    let root = data_root()?;
    std::fs::create_dir_all(&root)
        .map_err(|error| format!("No se pudo crear el directorio de DSH: {error}"))?;
    let package = format!("{DSH_PACKAGE}@{version}");
    let output = Command::new("npm")
        .args(["install", "--prefix"])
        .arg(&root)
        .args(["--no-save", "--no-package-lock"])
        .arg(package)
        .output()
        .map_err(|error| format!("No se pudo ejecutar npm install: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}

fn launch_web(app: AppHandle) {
    thread::spawn(move || {
        let Some((binary, _)) = find_dsh() else {
            show_error(
                &app,
                "No se encontró DSH. Instálalo desde esta ventana para continuar.".into(),
            );
            return;
        };
        let mut child = match Command::new(&binary)
            .args(["--profile", "web", "--no-open", "--port", "0"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("DSH_DESKTOP", "tauri")
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                show_error(&app, format!("No se pudo ejecutar DSH ({binary}): {error}"));
                return;
            }
        };
        let stdout = child.stdout.take();
        if let Some(state) = app.try_state::<DshProcess>() {
            *state.0.lock().unwrap() = Some(child);
        }
        let Some(stdout) = stdout else {
            show_error(&app, "DSH no proporcionó salida de inicio".into());
            return;
        };
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if let Some(url) = line
                .strip_prefix("dsh web: ")
                .and_then(|value| value.split_whitespace().next())
            {
                let url = url.to_string();
                let handle = app.clone();
                let _ = app.run_on_main_thread(move || {
                    if let Ok(window) = WebviewWindowBuilder::new(
                        &handle,
                        "dsh",
                        WebviewUrl::External(url.parse().unwrap()),
                    )
                    .title("DSH — DeepSeek Harness")
                    .inner_size(1280.0, 820.0)
                    .min_inner_size(760.0, 560.0)
                    .build()
                    {
                        if let Some(splash) = handle.get_webview_window("splash") {
                            let _ = splash.close();
                        }
                        let _ = window.set_focus();
                    }
                });
                return;
            }
        }
        show_error(
            &app,
            "DSH terminó antes de anunciar su URL. Revisa la instalación.".into(),
        );
    });
}

#[tauri::command]
fn start_dsh(app: AppHandle) -> Result<(), String> {
    if find_dsh().is_none() {
        return Err("DSH todavía no está instalado".into());
    }
    launch_web(app);
    Ok(())
}

fn show_error(app: &AppHandle, message: String) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Some(window) = handle.get_webview_window("splash") {
            let script = format!(
                "document.querySelector('#error').textContent = {}",
                serde_json::to_string(&message).unwrap()
            );
            let _ = window.eval(&script);
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DshProcess::default())
        .invoke_handler(tauri::generate_handler![
            dsh_status,
            list_dsh_versions,
            install_dsh,
            start_dsh
        ])
        .setup(|app| {
            if dsh_status().available {
                launch_web(app.handle().clone());
            }
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building DSH Tauri application")
        .run(|app, event| {
            if matches!(event, RunEvent::Exit) {
                if let Some(state) = app.try_state::<DshProcess>() {
                    if let Some(mut child) = state.0.lock().unwrap().take() {
                        let _ = child.kill();
                    }
                }
            }
        });
}
