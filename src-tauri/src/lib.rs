use std::net::{TcpStream, ToSocketAddrs};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;
use tauri::{webview::PageLoadEvent, Listener, Manager, WebviewUrl, WebviewWindowBuilder};

const MAIN_HOST: &str = "sky.shiiyu.moe";
const MAIN_SCHEME: &str = "https";
const OFFLINE_RETRY_SECS: u64 = 3;

fn is_allowed_host(host: &str) -> bool {
    host == MAIN_HOST || host.ends_with(".shiiyu.moe")
}

fn is_online() -> bool {
    let addrs = match (MAIN_HOST, 443).to_socket_addrs() {
        Ok(addrs) => addrs.collect::<Vec<_>>(),
        Err(_) => return false,
    };
    for addr in addrs {
        if TcpStream::connect_timeout(&addr, Duration::from_secs(2)).is_ok() {
            return true;
        }
    }
    false
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _: Result<(), _> = window.set_focus();
            }
        }))
        .setup(|app| {
            let app_handle = app.handle().clone();
            let is_hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                || std::env::var("XDG_CURRENT_DESKTOP")
                    .map(|v| v.to_lowercase().contains("hyprland"))
                    .unwrap_or(false);

            let _splash = WebviewWindowBuilder::new(
                app,
                "splash",
                WebviewUrl::App("index.html".into()),
            )
            .title("SkyCrypt Desktop")
            .inner_size(420.0, 260.0)
            .resizable(false)
            .decorations(false)
            .center()
            .build()
            .expect("failed to create splash window");

            let main_created = Arc::new(AtomicBool::new(false));
            let create_main_window = {
                let main_created = Arc::clone(&main_created);
                move |app_handle: &tauri::AppHandle| {
                    if main_created.swap(true, Ordering::SeqCst) {
                        return;
                    }
                WebviewWindowBuilder::new(
                    app_handle,
                    "main",
                    WebviewUrl::External(format!("{MAIN_SCHEME}://{MAIN_HOST}").parse().unwrap()),
                )
                .title("SkyCrypt Desktop")
                .inner_size(1400.0, 900.0)
                .center()
                .resizable(true)
                .decorations(!is_hyprland)
                .zoom_hotkeys_enabled(false)
                .visible(false)
                .on_page_load({
                    let app_handle = app_handle.clone();
                    move |window, payload| {
                        if payload.event() == PageLoadEvent::Finished {
                            if let Some(splash) = app_handle.get_webview_window("splash") {
                                let _ = splash.close();
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .on_navigation(|url| {
                    let allowed = url.scheme() == MAIN_SCHEME
                        && url.host_str().map(is_allowed_host).unwrap_or(false);
                    if allowed {
                        true
                    } else {
                        let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                        false
                    }
                })
                .on_new_window(|url, _features| {
                    let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                    tauri::webview::NewWindowResponse::Deny
                })
                .build()
                .expect("failed to create window");
                }
            };

            tauri::async_runtime::spawn_blocking({
                let app_handle = app_handle.clone();
                let main_created = Arc::clone(&main_created);
                move || {
                    if is_online() {
                        create_main_window(&app_handle);
                        return;
                    }

                    if let Some(splash) = app_handle.get_webview_window("splash") {
                        let _ = splash.close();
                    }

                    let _offline = WebviewWindowBuilder::new(
                        &app_handle,
                        "offline",
                        WebviewUrl::App("offline".into()),
                    )
                    .title("SkyCrypt Desktop")
                    .inner_size(520.0, 320.0)
                    .resizable(false)
                    .decorations(false)
                    .center()
                    .build()
                    .expect("failed to create offline window");

                    loop {
                        std::thread::sleep(Duration::from_secs(OFFLINE_RETRY_SECS));
                        if is_online() {
                            if let Some(offline) = app_handle.get_webview_window("offline") {
                                let _ = offline.close();
                            }
                            if !main_created.load(Ordering::SeqCst) {
                                create_main_window(&app_handle);
                            }
                            break;
                        }
                    }
                }
            });

            app_handle.listen("offline-retry", {
                let app_handle = app_handle.clone();
                let main_created = Arc::clone(&main_created);
                move |_| {
                    if is_online() && !main_created.load(Ordering::SeqCst) {
                        if let Some(offline) = app_handle.get_webview_window("offline") {
                            let _ = offline.close();
                        }
                        create_main_window(&app_handle);
                    }
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
