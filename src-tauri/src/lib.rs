#[cfg(not(mobile))]
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(not(mobile))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
    Mutex,
};
#[cfg(not(mobile))]
use std::time::Duration;
use tauri::{webview::PageLoadEvent, Url, WebviewUrl, WebviewWindowBuilder};
#[cfg(not(mobile))]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
#[cfg(not(mobile))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
#[cfg(not(mobile))]
use tauri::{Listener, Manager, WindowEvent};

const MAIN_HOST: &str = "sky.shiiyu.moe";
const MAIN_SCHEME: &str = "https";
#[cfg(not(mobile))]
const OFFLINE_RETRY_SECS: u64 = 3;

mod injector;

#[cfg(not(mobile))]
#[derive(Default)]
struct QuitFlag(AtomicBool);

#[cfg(not(mobile))]
struct TrayState(tauri::tray::TrayIcon);

#[cfg(not(mobile))]
struct LastProfile(Mutex<Option<String>>);

#[cfg(not(mobile))]
struct TrayMenuItems {
    show: MenuItem<tauri::Wry>,
    hide: MenuItem<tauri::Wry>,
    last_profile: MenuItem<tauri::Wry>,
    reload: MenuItem<tauri::Wry>,
    quit: MenuItem<tauri::Wry>,
}

// Injection toggles: keep the scripts available but disabled by default.
// Uncomment these to re-enable.
const INTERACTION_OVERRIDE_SCRIPT: &str = "injections/interaction_overrides.js";
// const ENHANCED_SCRIPT: &str = "injections/skycrypt_enhanced.js";
// const DEFAULT_THEME: &str = "default.json";

fn inject_interaction_overrides(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let _ = injector::inject_from_resource(app, window, INTERACTION_OVERRIDE_SCRIPT);
}

// fn inject_skycrypt_scripts(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
//     let _ = injector::inject_from_resource(app, window, ENHANCED_SCRIPT);
// }

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

#[cfg(not(mobile))]
fn show_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
        update_tray_menu(app);
        return;
    }
    if let Some(window) = app.get_webview_window("offline") {
        let _ = window.show();
        let _ = window.set_focus();
        update_tray_menu(app);
    }
}

#[cfg(not(mobile))]
fn hide_main_window(app: &tauri::AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
    if let Some(window) = app.get_webview_window("offline") {
        let _ = window.hide();
    }
    update_tray_menu(app);
}

#[cfg(not(mobile))]
fn is_main_focused(app: &tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(true) = window.is_focused() {
            return true;
        }
    }
    if let Some(window) = app.get_webview_window("offline") {
        if let Ok(true) = window.is_focused() {
            return true;
        }
    }
    false
}

#[cfg(not(mobile))]
fn is_main_visible(app: &tauri::AppHandle) -> bool {
    if let Some(window) = app.get_webview_window("main") {
        if let Ok(true) = window.is_visible() {
            return true;
        }
    }
    if let Some(window) = app.get_webview_window("offline") {
        if let Ok(true) = window.is_visible() {
            return true;
        }
    }
    false
}

#[cfg(not(mobile))]
fn update_tray_menu(app: &tauri::AppHandle) {
    let visible = is_main_visible(app);
    let last_exists = app
        .state::<LastProfile>()
        .0
        .lock()
        .ok()
        .and_then(|guard| guard.clone())
        .is_some();

    let items = app.state::<TrayMenuItems>();
    let _ = items.show.set_enabled(!visible);
    let _ = items.hide.set_enabled(visible);
    let _ = items.reload.set_enabled(visible);
    let _ = items.last_profile.set_enabled(visible && last_exists);
    let _ = items.quit.set_enabled(true);
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
                app.manage(QuitFlag::default());
                app.manage(LastProfile(Mutex::new(None)));

                let menu = Menu::new(app)?;
                let tray_show = MenuItem::with_id(app, "tray_show", "Show", true, None::<&str>)?;
                let tray_hide = MenuItem::with_id(app, "tray_hide", "Hide", true, None::<&str>)?;
                let tray_last_profile = MenuItem::with_id(
                    app,
                    "tray_last_profile",
                    "Open Last Profile",
                    true,
                    None::<&str>,
                )?;
                let tray_reload =
                    MenuItem::with_id(app, "tray_reload", "Reload", true, None::<&str>)?;
                let tray_quit = MenuItem::with_id(app, "tray_quit", "Quit", true, None::<&str>)?;
                let tray_sep = PredefinedMenuItem::separator(app)?;

                menu.append(&tray_show)?;
                menu.append(&tray_hide)?;
                menu.append(&tray_last_profile)?;
                menu.append(&tray_reload)?;
                menu.append(&tray_sep)?;
                menu.append(&tray_quit)?;

                let mut tray_builder = TrayIconBuilder::new()
                    .menu(&menu)
                    .tooltip("SkyCrypt Desktop")
                    .show_menu_on_left_click(false)
                    .on_menu_event(|app, event| match event.id().as_ref() {
                        "tray_show" => {
                            show_main_window(app);
                        }
                        "tray_hide" => {
                            hide_main_window(app);
                        }
                        "tray_last_profile" => {
                            let last_url = {
                                let last = app.state::<LastProfile>();
                                last.0.lock().ok().and_then(|guard| guard.clone())
                            };
                            if let Some(url) = last_url.as_ref() {
                                if let Some(window) = app.get_webview_window("main") {
                                    if let Ok(url) = Url::parse(url) {
                                        let _ = window.navigate(url);
                                    }
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                } else {
                                    show_main_window(app);
                                }
                            } else {
                                show_main_window(app);
                            }
                        }
                        "tray_reload" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.reload();
                            }
                        }
                        "tray_quit" => {
                            let state = app.state::<QuitFlag>();
                            state.0.store(true, Ordering::SeqCst);
                            app.exit(0);
                        }
                        _ => {}
                    })
                    .on_tray_icon_event(|tray, event| {
                        if let TrayIconEvent::Click {
                            button: MouseButton::Left,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            show_main_window(app);
                        }
                        if let TrayIconEvent::Click {
                            button: MouseButton::Right,
                            button_state: MouseButtonState::Up,
                            ..
                        } = event
                        {
                            let app = tray.app_handle();
                            let focused = is_main_focused(app);
                            let visible = is_main_visible(app);
                            if !focused || !visible {
                                show_main_window(app);
                            }
                        }
                    });

                if let Some(icon) = app.default_window_icon().cloned() {
                    tray_builder = tray_builder.icon(icon);
                }

                let tray = tray_builder.build(app)?;
                app.manage(TrayState(tray));
                app.manage(TrayMenuItems {
                    show: tray_show,
                    hide: tray_hide,
                    last_profile: tray_last_profile,
                    reload: tray_reload,
                    quit: tray_quit,
                });
                update_tray_menu(app.handle());

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
                        let window = WebviewWindowBuilder::new(
                            app_handle,
                            "main",
                            WebviewUrl::External(
                                format!("{MAIN_SCHEME}://{MAIN_HOST}").parse().unwrap(),
                            ),
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
                                    // inject_skycrypt_scripts(&app_handle, &window);
                                if let Some(splash) = app_handle.get_webview_window("splash")
                                {
                                    let _ = splash.close();
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                                update_tray_menu(&app_handle);
                            }
                        }
                    })
                        .on_navigation({
                            let app_handle = app_handle.clone();
                            move |url| {
                                if is_blank_url(url) {
                                    return false;
                                }
                                let allowed = url.scheme() == MAIN_SCHEME
                                    && url.host_str().map(is_allowed_host).unwrap_or(false);
                                if allowed {
                                    if url.path().starts_with("/stats/") {
                                        if let Ok(mut last) =
                                            app_handle.state::<LastProfile>().0.lock()
                                        {
                                            *last = Some(url.as_str().to_string());
                                            update_tray_menu(&app_handle);
                                        }
                                    }
                                    true
                                } else {
                                    let _ =
                                        tauri_plugin_opener::open_url(url.as_str(), None::<&str>);
                                    false
                                }
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

                        let window_for_event = window.clone();
                        let app_handle_for_event = app_handle.clone();
                        window.on_window_event(move |event| {
                            if let WindowEvent::CloseRequested { api, .. } = event {
                                let should_quit = app_handle_for_event
                                    .state::<QuitFlag>()
                                    .0
                                    .load(Ordering::SeqCst);
                                if !should_quit {
                                    api.prevent_close();
                                    let _ = window_for_event.hide();
                                    update_tray_menu(&app_handle_for_event);
                                }
                            }
                        });
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

                app_handle.listen("skycrypt-url", {
                    let app_handle = app_handle.clone();
                    move |event| {
                        let payload = event.payload();
                        if let Ok(url) = Url::parse(payload) {
                            if url.scheme() == MAIN_SCHEME
                                && url.host_str().map(is_allowed_host).unwrap_or(false)
                                && url.path().starts_with("/stats/")
                            {
                                if let Ok(mut last) = app_handle.state::<LastProfile>().0.lock() {
                                    *last = Some(url.as_str().to_string());
                                    update_tray_menu(&app_handle);
                                }
                            }
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
                        // inject_skycrypt_scripts(&app_handle, window);
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
