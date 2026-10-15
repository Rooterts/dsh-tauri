#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod gpu;

fn main() {
    // En Wayland dejamos que WebKitGTK use la composición acelerada del sistema.
    // Tauri/WebKitGTK no ofrece un backend Vulkan directo: la GPU Vulkan sigue
    // disponible para el sistema, pero el WebView renderiza mediante EGL/OpenGL.
    if std::env::var("XDG_SESSION_TYPE").as_deref() == Ok("wayland")
        && std::env::var_os("WAYLAND_DISPLAY").is_some()
    {
        std::env::set_var("GDK_BACKEND", "wayland");
    }
    std::env::remove_var("LIBGL_ALWAYS_SOFTWARE");
    std::env::remove_var("WEBKIT_DISABLE_COMPOSITING_MODE");
    gpu::probe_vulkan();
    dsh_tauri_lib::run();
}
