use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::Mutex,
    thread,
};
use tauri::{webview::WebviewWindowBuilder, AppHandle, Manager, RunEvent, WebviewUrl};

#[derive(Default)]
struct DshProcess(Mutex<Option<Child>>);

fn launch_web(app: AppHandle) {
    thread::spawn(move || {
        let mut child = match Command::new("dsh")
            .args(["--profile", "web", "--no-open", "--port", "0"])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .env("DSH_DESKTOP", "tauri")
            .spawn()
        {
            Ok(child) => child,
            Err(error) => {
                show_error(&app, format!("No se pudo ejecutar dsh: {error}"));
                return;
            }
        };
        let stdout = child.stdout.take();
        if let Some(state) = app.try_state::<DshProcess>() {
            *state.0.lock().unwrap() = Some(child);
        }
        let Some(stdout) = stdout else {
            show_error(&app, "dsh no proporcionó salida de inicio".into());
            return;
        };
        for line in BufReader::new(stdout).lines().map_while(Result::ok) {
            if let Some(url) = line
                .strip_prefix("dsh web: ")
                .and_then(|v| v.split_whitespace().next())
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
            "dsh terminó antes de anunciar su URL. Revisa la instalación de la interfaz web."
                .into(),
        );
    });
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
        .setup(|app| {
            launch_web(app.handle().clone());
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
