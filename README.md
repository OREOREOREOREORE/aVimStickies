<div align="center">

<img width="256" height="256" alt="aVimStickies logo" src="src-tauri/icons/vim.png" />

# aVimStickies

*A markdown sticky-note app for macOS — written in Rust, made for vim users.*

[![Release](https://img.shields.io/github/v/release/OREOREOREOREORE/aVimStickies?style=flat-square)](https://github.com/OREOREOREOREORE/aVimStickies/releases)
[![License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](#)
![Platform](https://img.shields.io/badge/platform-macOS-lightgrey.svg?style=flat-square)
![Language](https://img.shields.io/badge/language-Rust-orange.svg?style=flat-square)

This app is only for vim users — if you live in `hjkl`, `yy`, and `dd`, you'll feel at home. Written in Rust.

[Install](#install) • [Features](#features) • [Keybindings](#keybindings)

</div>

## Install
Download the .dmg from [Releases](../../releases), drag aVimStickies to Applications, then right-click → Open on first launch (unsigned build).

## Features
- Vim editing + markdown preview (`Cmd+P`) in every note
- Floating windows with remembered position/size, 6-color palette, and pin-on-top
- Auto-save to plain `.md`, live reload on external edits, and cross-note search (`Cmd+Shift+F`)
- Menu-bar tray with note list; customizable font, size, theme, opacity, and line numbers
- In-app auto-update — a banner appears when a new release is out; one click to install

## Keybindings

| Shortcut | Action |
| --- | --- |
| `Cmd+N` | New note (global) |
| `Cmd+P` | Toggle markdown preview |
| `Cmd+Shift+P` | Pin / unpin note on top |
| `Cmd+Delete` | Delete note |
| `Cmd+Shift+C` | Cycle note color (opt-in) |
| `Cmd+` / `Cmd-` | Increase / decrease font size |
| `Cmd+,` | Open settings |
| `Cmd+Shift+F` | Search all notes |
| `Cmd+W` | Hide note window |
| `Cmd+Q` | Quit |

## Notes data
Plain markdown files in `~/Stickies/` — edit them with any editor and open notes refresh automatically.
