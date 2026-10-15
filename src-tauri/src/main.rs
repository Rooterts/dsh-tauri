#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gpu;

fn main() {
    if std::env::args().any(|arg| arg == "--dsh-gpu-probe") {
        gpu::probe_best_backend();
        return;
    }

    // En Wayland dejamos que WebKitGTK use la composición acelerada del sistema.
    // Tauri/WebKitGTK no ofrece un backend Vulkan directo: la GPU Vulkan sigue
    // disponible para el sistema, pero el WebView renderiza mediante EGL/OpenGL.
    if std::env::var("XDG_SESSION_TYPE").as_deref() == Ok("wayland")
        && std::env::var_os("WAYLAND_DISPLAY").is_some()
    {
        std::env::set_var("GDK_BACKEND", "wayland");
    }
    // WebKitGTK 4.1 can produce corrupted surfaces (a half white, half green
    // window) when the DMA-BUF renderer is used with some Mesa/driver
    // combinations. Disable that path unless it was set explicitly or the user
    // opts back in with DSH_DESKTOP_ALLOW_DMABUF=1.
    if std::env::var_os("WEBKIT_DISABLE_DMABUF_RENDERER").is_none()
        && std::env::var_os("DSH_DESKTOP_ALLOW_DMABUF").is_none()
    {
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    gpu::detect_best_backend();
    dsh_tauri_lib::run();
}
