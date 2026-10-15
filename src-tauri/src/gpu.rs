//! Experimental graphics backend detector.
//!
//! The probe runs in a short-lived helper process so wgpu/Vulkan libraries do
//! not remain mapped in the main WebKitGTK process.

use std::process::Command;

pub fn probe_best_backend() {
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::VULKAN | wgpu::Backends::GL,
        flags: wgpu::InstanceFlags::default(),
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
            println!(
                "dsh gpu: selected backend={:?} adapter={} vendor=0x{:04x} device=0x{:04x}",
                info.backend, info.name, info.vendor, info.device
            );
        }
        None => println!("dsh gpu: no accelerated adapter; WebView fallback remains active"),
    }
}

pub fn detect_best_backend() {
    let Ok(executable) = std::env::current_exe() else {
        eprintln!("dsh gpu: could not locate helper executable");
        return;
    };

    match Command::new(executable).arg("--dsh-gpu-probe").output() {
        Ok(output) if output.status.success() => {
            let result = String::from_utf8_lossy(&output.stdout);
            eprintln!("{}", result.trim());
        }
        Ok(output) => eprintln!(
            "dsh gpu: probe exited with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ),
        Err(error) => eprintln!("dsh gpu: probe unavailable: {error}"),
    }
}
