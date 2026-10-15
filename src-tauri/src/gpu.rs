//! Experimental Vulkan capability probe.
//!
//! This deliberately does not replace the Tauri WebView yet: WebKitGTK owns
//! the web surface and still composites through EGL/OpenGL on Linux.

pub fn probe_vulkan() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN,
        dx12_shader_compiler: Default::default(),
        gles_minor_version: wgpu::Gles3MinorVersion::Automatic,
    });

    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
    }));

    match adapter {
        Some(adapter) => {
            let info = adapter.get_info();
            eprintln!(
                "dsh gpu: Vulkan adapter={} vendor=0x{:04x} device=0x{:04x} backend={:?}",
                info.name, info.vendor, info.device, info.backend
            );
        }
        None => eprintln!("dsh gpu: no Vulkan adapter available; WebView fallback remains active"),
    }
}
