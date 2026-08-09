# aVimStickies

A markdown sticky-note app for macOS with built-in vim keybindings. Write notes in markdown using `hjkl`, `yy`, `dd`, and friends — just like in your editor — on little floating sticky notes.

## Features

- **Vim editing** in every note (CodeMirror + vim: normal/insert/visual modes, `hjkl`, `yy`, `dd`, `gg`, `/`, etc.)
- **Markdown preview** toggle — `Cmd+P`
- **Floating sticky windows** — drag from the header, resize from the edges; position and size are remembered
- **Menu bar (tray) icon** — list all notes, open them, create new ones, open settings, search, quit
- **Note colors** — 6-color palette per note (click the color dot or `Cmd+Shift+C` when enabled)
- **Customizable** — font family, font size, dark theme, window opacity, line numbers, status bar, and which buttons to show
- **Auto-save** — every edit is saved to a plain `.md` file, debounced
- **Live reload** — edit a note file externally and the window refreshes itself
- **Cross-note search** — `Cmd+Shift+F`
- **Pin notes on top** — `Cmd+Shift+P`
- **Global shortcuts** — `Cmd+N` creates a note from anywhere

## Install

1. Download `aVimStickies_<version>_aarch64.dmg` from the [Releases](../../releases) page.
2. Open the DMG and drag **aVimStickies** into your Applications folder.
3. **First launch (unsigned build):** macOS Gatekeeper may warn that the app is from an unidentified developer. Right-click the app in Applications → **Open** → **Open** again. It only needs to be done once.

## Keybindings

| Shortcut | Action |
| --- | --- |
| `Cmd+N` | New note (global) |
| `Cmd+P` | Toggle markdown preview |
| `Cmd+Shift+P` | Pin / unpin note on top |
| `Cmd+Delete` | Delete note |
| `Cmd+Shift+C` | Cycle note color (opt-in in settings) |
| `Cmd+=` / `Cmd+-` | Increase / decrease font size |
| `Cmd+,` | Open settings |
| `Cmd+Shift+F` | Search all notes |
| `Cmd+W` | Hide note window to the tray |
| `Cmd+Q` | Quit |

Inside the editor, all normal vim motions and commands work (`i` insert, `Esc`, `:w` not needed — auto-save, visual mode, etc.).

## Notes data

Notes are stored as plain markdown files in `~/Stickies/`:

- `<id>.md` — one file per note
- `meta.json` — note metadata (window position, size, color, pin)
- `settings.json` — app preferences

You can edit the `.md` files directly with any editor; open notes pick up the changes automatically.

## Build from source

Prerequisites: [Rust](https://rustup.rs/), [Node.js](https://nodejs.org/), [pnpm](https://pnpm.io/).

```bash
pnpm install
pnpm tauri dev     # run in development
pnpm tauri build   # produce a release .app and .dmg
```

## Release process

Push a tag to trigger the release workflow (`.github/workflows/release.yml`):

```bash
git tag v0.1.0
git push origin v0.1.0
```

GitHub Actions builds the app and publishes a release with the DMG attached.

## License

MIT
