# SkyCrypt Desktop

[![GitHub release (latest)](https://img.shields.io/github/v/release/GrishMahat/SkyCrypt-desktop?include_prereleases&label=latest)](https://github.com/GrishMahat/SkyCrypt-desktop/releases/latest)
[![GitHub all releases](https://img.shields.io/github/downloads/GrishMahat/SkyCrypt-desktop/total?label=total)](https://github.com/GrishMahat/SkyCrypt-desktop/releases)
[![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-blue)](https://github.com/GrishMahat/SkyCrypt-desktop/releases)
[![License](https://img.shields.io/github/license/GrishMahat/SkyCrypt-desktop)](https://github.com/GrishMahat/SkyCrypt-desktop/blob/master/LICENSE)

SkyCrypt Desktop is a lightweight desktop app for viewing Hypixel SkyBlock statistics via SkyCrypt (`https://sky.shiiyu.moe/`). Built with Rust and Tauri, it delivers a fast native window without Electron overhead.

---

## Downloads

### Latest Release (v0.1.6)
| Platform | File |
|----------|------|
| Windows | [SkyCrypt.Desktop_0.1.6_x64-setup.exe](https://github.com/GrishMahat/SkyCrypt-desktop/releases/download/v0.1.6/SkyCrypt.Desktop_0.1.6_x64-setup.exe) |
| macOS (Apple Silicon) | [SkyCrypt.Desktop_0.1.6_aarch64.dmg](https://github.com/GrishMahat/SkyCrypt-desktop/releases/download/v0.1.6/SkyCrypt.Desktop_0.1.6_aarch64.dmg) |
| Linux (Debian) | [SkyCrypt.Desktop_0.1.6_amd64.deb](https://github.com/GrishMahat/SkyCrypt-desktop/releases/download/v0.1.6/SkyCrypt.Desktop_0.1.6_amd64.deb) |
| Linux (RPM) | [SkyCrypt.Desktop-0.1.6-1.x86_64.rpm](https://github.com/GrishMahat/SkyCrypt-desktop/releases/download/v0.1.6/SkyCrypt.Desktop-0.1.6-1.x86_64.rpm) |

### Previous Releases
| Version | Release Page |
|---------|--------------|
| [v0.1.5](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.5) |
| [v0.1.4](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.4) |
| [v0.1.3](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.3) |
| [v0.1.2](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.2) |
| [v0.1.1](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.1) |
| [v0.1.0](https://github.com/GrishMahat/SkyCrypt-desktop/releases/v0.1.0) |

---

## Overview

This project turns the SkyCrypt web app into a native desktop experience without the overhead of Electron. It uses a minimal WebView approach while allowing custom enhancements like UI injection and additional controls.

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
