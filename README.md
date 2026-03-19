# SkyCrypt Desktop

A lightweight desktop wrapper for SkyCrypt built with Rust and Tauri.

This project turns the SkyCrypt web app into a native desktop experience without the overhead of Electron. It uses a minimal WebView approach while allowing custom enhancements like UI injection and additional controls.

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
# SkyCrypt-desktop
