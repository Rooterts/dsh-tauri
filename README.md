# DSH Desktop (Tauri 2)

Prototipo de aplicación de escritorio para abrir DeepSeek Harness Web sin tener que escribir `dsh web` en una terminal.

## Qué hace

1. Arranca `dsh --profile web --no-open --port 0` como proceso hijo.
2. Espera la URL autenticada que DSH anuncia cuando el servidor está listo.
3. Abre esa URL dentro de una ventana Tauri 2/WebView.
4. Cierra el proceso DSH al salir de la aplicación.

El servidor sigue siendo el Web oficial de DSH: esta primera iteración es un launcher nativo, no una reimplementación de la interfaz ni del runtime de agentes. Esto mantiene compatibilidad con sesiones, plugins, credenciales y actualizaciones de DSH.

## Requisitos Linux / CachyOS

- `dsh` instalado y funcionando desde una terminal (`dsh --version`).
- Rust, Cargo y `cargo-tauri` para compilar.
- Dependencias WebKitGTK de Tauri 2 (en CachyOS/Arch suelen estar disponibles mediante `webkit2gtk-4.1`, `libappindicator-gtk3`, `librsvg` y herramientas de desarrollo).

## Desarrollo

Desde esta carpeta:

```bash
pnpm install
pnpm dev              # release optimizado
pnpm run dev:debug    # desarrollo con hot reload
```

En Linux/Wayland el launcher selecciona Wayland cuando está disponible y no fuerza renderizado por software. Tauri usa WebKitGTK; en Linux su composición del WebView se basa en EGL/OpenGL, aunque la GPU Vulkan del sistema siga disponible para otras aplicaciones.

## Paquetes Linux

```bash
pnpm build
```

Genera AppImage, `.deb` y `.rpm` en `src-tauri/target/release/bundle/` según las herramientas instaladas.

## Siguientes iteraciones

- Detectar instalaciones de `dsh` fuera de `PATH` y permitir elegir el binario.
- Preferencias de puerto, perfil y directorio de trabajo.
- Icono, autostart y notificación de bandeja para CachyOS/Hyprland.
- Reutilizar el core/runtime de DSH como librería o proceso supervisado cuando exista una API pública estable.
- Empaquetado reproducible para Linux y después Windows/macOS/Android/iOS donde Tauri lo permita.
