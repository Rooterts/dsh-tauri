# DSH Desktop (Tauri 2)

Prototipo de aplicación de escritorio para abrir DeepSeek Harness Web sin tener que escribir `dsh web` en una terminal.

## Qué hace

1. Detecta un DSH ya instalado en el sistema (`dsh` en `PATH`).
2. Si no lo encuentra, ofrece instalarlo desde la propia ventana, eligiendo versión estable o preliminar.
3. Arranca `dsh --profile web --no-open --port 0` como proceso hijo.
4. Espera la URL autenticada que DSH anuncia cuando el servidor está listo.
5. Abre esa URL dentro de una ventana Tauri 2/WebView.
6. Cierra el proceso DSH al salir de la aplicación.

## Primer arranque

La primera vez que se abre la aplicación se comprueba si DSH está disponible. Si no lo está, aparece un instalador gráfico con:

- Estado de detección de DSH y de Node.js/npm.
- Selector de versiones publicadas del paquete `@deepseek-ai/dsh`.
- Casilla **Mostrar también versiones preliminares (rc, beta, alpha)**.
- Botón para instalar y botón para abrir DSH al terminar.

Si no existe ninguna versión estable publicada, el instalador avisa y muestra automáticamente las preliminares.

DSH se instala en el espacio de usuario de la aplicación:

```text
~/.local/share/dsh-desktop/runtime
```

Se respetan `XDG_DATA_HOME` y la variable `DSH_BIN` para usar un binario propio. Los DSH instalados en el sistema nunca se modifican.

El servidor sigue siendo el Web oficial de DSH: esta primera iteración es un launcher nativo, no una reimplementación de la interfaz ni del runtime de agentes. Esto mantiene compatibilidad con sesiones, plugins, credenciales y actualizaciones de DSH.

## Requisitos Linux / CachyOS

- `dsh` instalado en `PATH` **o** Node.js/npm para instalarlo desde la aplicación en el primer arranque.
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

La CI de GitHub genera automáticamente tres formatos al publicar un tag `vX.Y.Z`:

- **AppImage**: opción recomendada para Arch/CachyOS y distribuciones no basadas en Debian.
- **DEB**: Debian, Ubuntu, Linux Mint y derivados.
- **RPM**: Fedora, openSUSE y derivados.

### Instalar desde una release

```bash
# AppImage
chmod +x DSH_*_amd64.AppImage
./DSH_*_amd64.AppImage

# Debian/Ubuntu
sudo apt install ./DSH_*_amd64.deb

# Fedora/openSUSE
sudo dnf install ./DSH-*.x86_64.rpm
```

### Arch Linux / CachyOS

La forma más sencilla es usar AppImage. También se incluye un PKGBUILD en `packaging/arch/`; se debe ajustar `pkgver` al tag publicado y comprobar que `url` apunta al repositorio correcto:

```bash
cd packaging/arch
makepkg -si
```

### Publicar una release

La workflow de GitHub Actions se activa al subir un tag semántico, o manualmente desde la pestaña **Actions** con `workflow_dispatch`:

```bash
git tag v0.1.0
git push origin v0.1.0
```

La release se crea como borrador y contiene AppImage, DEB y RPM.

> **Importante:** los paquetes contienen el launcher y su instalador, pero no incluyen Node.js. Si el usuario no tiene `dsh` en `PATH`, la instalación desde la aplicación necesita `node` y `npm`; en Arch/CachyOS: `sudo pacman -S nodejs npm`.

Para generar los paquetes localmente:

```bash
pnpm build
```

Los artefactos aparecen en `src-tauri/target/release/bundle/`.

## Rama experimental wgpu/Vulkan

La rama Git `experiment/wgpu-vulkan-shell` añade una sonda wgpu que selecciona exclusivamente Vulkan y registra el adaptador físico detectado. Sirve para validar la GPU nativa del sistema antes de construir un renderer propio.

Importante: esta sonda no sustituye todavía el WebView de DSH. WebKitGTK sigue siendo el motor que pinta la web y utiliza EGL/OpenGL en Linux. Para reemplazarlo completamente habría que cambiar a Chromium/CEF o implementar un compositor híbrido, con un coste mayor de RAM y mantenimiento.

Para probar la rama:

~~~bash
git switch experiment/wgpu-vulkan-shell
pnpm install
pnpm dev
~~~

## Siguientes iteraciones

- Detectar instalaciones de `dsh` fuera de `PATH` y permitir elegir el binario.
- Preferencias de puerto, perfil y directorio de trabajo.
- Icono, autostart y notificación de bandeja para CachyOS/Hyprland.
- Reutilizar el core/runtime de DSH como librería o proceso supervisado cuando exista una API pública estable.
- Empaquetado reproducible para Linux y después Windows/macOS/Android/iOS donde Tauri lo permita.
