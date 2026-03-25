// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

/// Entry point for the SkyCrypt desktop application.
/// Accepts an optional player name as a command-line argument.
fn main() {
    let args: Vec<String> = std::env::args().collect();
    let player_name = args.get(1).cloned();
    skycrypt_desktop_lib::run(player_name);
}
