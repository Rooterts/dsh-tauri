# Rendering benchmark

Small harness used to compare how fast a DSH-style, token-streaming chat list renders in different engines and WebKitGTK configurations on the same machine. It is what produced the numbers quoted in the main README and in the source comments.

## How to run

Start the report server first (it also serves the page):

```bash
node bench/server.cjs
```

Then open <http://127.0.0.1:8799/bench.html> in the engine you want to measure. The page runs a six second streaming workload (200 message rows, three tokens appended per frame, forced layout) and posts a JSON report back to the server, which prints it to stdout.

For WebKitGTK, which has no browser binary, a minimal shell is enough:

```bash
WEBKIT_DISABLE_COMPOSITING_MODE=1 python3 - <<'PY'
import gi
gi.require_version('Gtk', '3.0')
gi.require_version('WebKit2', '4.1')
from gi.repository import Gtk, WebKit2, GLib
win = Gtk.Window(); view = WebKit2.WebView(); win.add(view)
win.set_default_size(1280, 820); win.show_all()
view.load_uri('http://127.0.0.1:8799/bench.html')
GLib.timeout_add_seconds(14, Gtk.main_quit)
Gtk.main()
PY
```

## Reported metrics

- `fps`: frames per second over the measured window.
- `p50ms`, `p95ms`, `maxms`: frame time percentiles, in milliseconds.

## Results measured on the development machine

Intel Iris Xe (TGL GT2), Mesa 26.2.4, Wayland, 1280x820 window.

| Configuration | fps | p50 | p95 | CPU | RAM |
|---|---|---|---|---|---|
| WebKitGTK, CPU compositor (app default) | 61.2 | 17 ms | 20 ms | 19.7% | 178 MB |
| WebKitGTK, stock GPU compositing | 58.3 / 58.7 | 17 ms | 21 ms | 27.8% | 185 MB |
| WebKitGTK, DMA-BUF renderer disabled | 47.3 / 40.3 | 21-25 ms | 33-37 ms | 27.0% | 202 MB |
| Chromium 154 | 83.4 | 11.1 ms | 22.2 ms | not measured | not measured |

Run to run variation is noticeable, so treat single runs as indicative. The stock GPU configuration also corrupted the window surface on this machine, which is why it is not the default.
