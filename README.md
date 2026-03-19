# SkyCrypt Desktop

SkyCrypt Desktop is a lightweight desktop app for viewing Hypixel SkyBlock statistics via SkyCrypt (`https://sky.shiiyu.moe/`). Built with Rust and Tauri, it delivers a fast native window without Electron overhead.

This project turns the SkyCrypt web app into a native desktop experience without the overhead of Electron. It uses a minimal WebView approach while allowing custom enhancements like UI injection and additional controls.

## Overview

SkyCrypt Desktop is a Hypixel SkyBlock desktop app that wraps the SkyCrypt website in a native window. If you are searching for a SkyCrypt desktop app, Hypixel SkyBlock stats desktop app, or a lightweight SkyBlock profile viewer for PC, this project is that desktop client.

## Use Cases

* View SkyCrypt profiles on desktop
* Open Hypixel SkyBlock stats quickly in a native window
* Keep a dedicated SkyCrypt desktop window while gaming

## Features

* Fast and lightweight (Rust + Tauri)
* Native desktop window for SkyCrypt
* Custom JavaScript injection support
* No Chromium bloat or heavy runtime

## Tech Stack

* Rust
* Tauri (WebView-based desktop framework)

## Notes

This is essentially a thin wrapper around the SkyCrypt website. Functionality depends on the upstream site, and UI injections may break if the site structure changes.

## Installation

### Build from source

```bash
pnpm install
pnpm run tauri dev
```

To build a release binary:

```bash
pnpm run tauri build
```

### Arch Linux (AUR)

Package names:

* `skycrypt` (builds from source)
* `skycrypt-bin` (prebuilt binary)

Install:

```bash
yay -S skycrypt
# or
yay -S skycrypt-bin
```

This repo includes the PKGBUILDs in `AUR/` and `AUR-bin/`.

# Related Searches

People also find this project when looking for a SkyCrypt desktop app, SkyCrypt desktop client, SkyCrypt for PC, Hypixel SkyBlock stats app, SkyBlock profile viewer, or a lightweight SkyBlock stats desktop client.

# SkyCrypt-desktop
