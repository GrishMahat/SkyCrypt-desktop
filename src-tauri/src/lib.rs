#[cfg(not(mobile))]
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(not(mobile))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
#[cfg(not(mobile))]
use std::time::Duration;
use tauri::{webview::PageLoadEvent, Url, WebviewUrl, WebviewWindowBuilder};
#[cfg(not(mobile))]
use tauri::{Listener, Manager};

const MAIN_HOST: &str = "sky.shiiyu.moe";
const MAIN_SCHEME: &str = "https";
#[cfg(not(mobile))]
const OFFLINE_RETRY_SECS: u64 = 3;

mod injector;

const INTERACTION_OVERRIDE_SCRIPT: &str = "injections/interaction_overrides.js";

fn inject_interaction_overrides(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let _ = injector::inject_from_resource(app, window, INTERACTION_OVERRIDE_SCRIPT);
}

fn is_allowed_host(host: &str) -> bool {
    host == MAIN_HOST || host.ends_with(".shiiyu.moe")
}

#[cfg(not(mobile))]
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

fn is_blank_url(url: &Url) -> bool {
    url.as_str() == "about:blank" || url.scheme() == "about"
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default().plugin(tauri_plugin_opener::init());
    #[cfg(not(mobile))]
    let builder = builder.plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
        if let Some(window) = app.get_webview_window("main") {
            let _: Result<(), _> = window.set_focus();
        }
    }));

    builder
        .setup(|app| {
            #[cfg(not(mobile))]
            {
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
            let create_main_window: Arc<dyn Fn(&tauri::AppHandle) + Send + Sync> = {
                let main_created = Arc::clone(&main_created);
                Arc::new(move |app_handle: &tauri::AppHandle| {
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
                            inject_interaction_overrides(&app_handle, &window);
                            if let Some(splash) = app_handle.get_webview_window("splash") {
                                let _ = splash.close();
                            }
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .on_navigation(|url| {
                    if is_blank_url(url) {
                        return false;
                    }
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
                    if is_blank_url(&url) {
                        return tauri::webview::NewWindowResponse::Deny;
                    }
                    let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                    tauri::webview::NewWindowResponse::Deny
                })
                .build()
                .expect("failed to create window");
                })
            };

            tauri::async_runtime::spawn_blocking({
                let app_handle = app_handle.clone();
                let main_created = Arc::clone(&main_created);
                let create_main_window = Arc::clone(&create_main_window);
                move || {
                    if is_online() {
                        (create_main_window)(&app_handle);
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
                                (create_main_window)(&app_handle);
                            }
                            break;
                        }
                    }
                }
            });

            app_handle.listen("offline-retry", {
                let app_handle = app_handle.clone();
                let main_created = Arc::clone(&main_created);
                let create_main_window = Arc::clone(&create_main_window);
                move |_| {
                    if is_online() && !main_created.load(Ordering::SeqCst) {
                        if let Some(offline) = app_handle.get_webview_window("offline") {
                            let _ = offline.close();
                        }
                        (create_main_window)(&app_handle);
                    }
                }
            });
            }

            #[cfg(mobile)]
            {
                let app_handle = app.handle().clone();
                WebviewWindowBuilder::new(
                    app,
                    "main",
                    WebviewUrl::External(format!("{MAIN_SCHEME}://{MAIN_HOST}").parse().unwrap()),
                )
                .on_page_load(move |window, payload| {
                    if payload.event() == PageLoadEvent::Finished {
                        inject_interaction_overrides(&app_handle, window);
                    }
                })
                .on_navigation(|url| {
                    if is_blank_url(url) {
                        return false;
                    }
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
                    if is_blank_url(&url) {
                        return tauri::webview::NewWindowResponse::Deny;
                    }
                    let _ = tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                    tauri::webview::NewWindowResponse::Deny
                })
                .build()
                .expect("failed to create mobile webview");
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
