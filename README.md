# DSH Desktop

A desktop app for **DeepSeek Harness**: it opens the DSH Web UI in a native window, so you never have to run `dsh web` in a terminal.

## Features

- **No terminal.** Starts the DSH web server for you and opens it in a native window.
- **First-run installer.** If DSH is not installed yet, the app offers to install it for you, with an optional list of prerelease versions.
- **Respects your system install.** If `dsh` is already available, it is used as-is and never modified.
- **Clean shutdown.** The DSH process is stopped when you close the window.
- **Linux packaging.** AppImage, DEB and RPM builds are produced automatically for every release.

## Install

Download the latest package from the [releases page](https://github.com/Rooterts/dsh-tauri/releases), or build it yourself.

### AppImage (any distribution)

```bash
chmod +x DSH_*_amd64.AppImage
./DSH_*_amd64.AppImage
```

### Debian / Ubuntu

```bash
sudo apt install ./DSH_*_amd64.deb
```

### Fedora / openSUSE

```bash
sudo dnf install ./DSH-*.x86_64.rpm
```

### Arch Linux / CachyOS

Use the AppImage, or build a package with the provided [PKGBUILD](packaging/arch/PKGBUILD):

```bash
cd packaging/arch
makepkg -si
```

The PKGBUILD downloads the AppImage from the GitHub release, so keep `pkgver` in sync with the released tag.

## First run

On startup the app checks whether DSH is available. If it is, the DSH Web UI opens right away. Otherwise you get an installer window where you can:

- See whether DSH and Node.js/npm were detected.
- Pick a DSH version from the published list.
- Enable **Show prerelease versions (rc, beta, alpha)** to see non-stable builds.
- Install, then open DSH.

DSH is installed into your user data directory and is only used by this app:

```text
~/.local/share/dsh-desktop/runtime
```

`XDG_DATA_HOME` is respected. You can also point the app at an existing binary with the `DSH_BIN` environment variable.

> Note: at the time of writing, `@deepseek-ai/dsh` only publishes prerelease versions. When no stable version exists, the installer says so and lists the prereleases instead.

## Requirements

- Linux with GTK3 / WebKitGTK 4.1 (already present on most desktops).
- `dsh` in `PATH`, **or** `node` and `npm` so the app can install it for you. On Arch/CachyOS:

  ```bash
  sudo pacman -S nodejs npm
  ```

## Build from source

```bash
pnpm install
pnpm dev            # release profile, uses the local dsh
pnpm run dev:debug  # development profile with hot reload
pnpm build          # AppImage, DEB and RPM in src-tauri/target/release/bundle/
```

## How it works

The app is a native launcher, not a reimplementation of DSH. It runs the real DSH web server and renders the official web UI in a WebView, which keeps sessions, plugins and credentials fully compatible.

1. Detect an existing `dsh`, or use the managed install.
2. Run `dsh --profile web --no-open --port 0` as a child process.
3. Wait for the URL the server prints when it is ready.
4. Open that URL in a Tauri 2 / WebKitGTK window, and close the splash screen.

During startup the app also probes for an accelerated graphics backend (Vulkan or OpenGL) in a short-lived helper process and logs the result. Your system GPU is used through the normal GTK/WebKitGTK stack; the probe is informational.

## Maintainers

Releases are built by GitHub Actions. Push a semantic tag, or run the *Release Linux bundles* workflow manually:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow creates a **draft** release with the AppImage, DEB and RPM attached. Review it and publish when ready.

## License

[MIT](LICENSE)
