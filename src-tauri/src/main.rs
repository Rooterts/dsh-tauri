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
    // With some Mesa/driver combinations the GPU compositing path of WebKitGTK
    // corrupts the window surface (a half white, half green picture). Measured on
    // this workload, the CPU compositor is also faster (61 fps against 40-47 fps)
    // and uses less CPU and memory, so it is used by default. Set
    // DSH_DESKTOP_ALLOW_GPU_COMPOSITING=1 to keep the stock GPU path.
    if std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none()
        && std::env::var_os("DSH_DESKTOP_ALLOW_GPU_COMPOSITING").is_none()
    {
        std::env::set_var("WEBKIT_DISABLE_COMPOSITING_MODE", "1");
    }
    gpu::detect_best_backend();
    dsh_tauri_lib::run();
}
