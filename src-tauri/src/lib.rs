#[cfg(not(mobile))]
use std::fs::{self, OpenOptions};
#[cfg(not(mobile))]
use std::io::Write;
#[cfg(not(mobile))]
use std::net::{TcpStream, ToSocketAddrs};
#[cfg(not(mobile))]
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
#[cfg(not(mobile))]
use std::time::Duration;
#[cfg(not(mobile))]
use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
#[cfg(not(mobile))]
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};
use tauri::{webview::PageLoadEvent, Url, WebviewUrl, WebviewWindowBuilder};
#[cfg(not(mobile))]
use tauri::{Listener, Manager, WindowEvent};

const MAIN_HOST: &str = "sky.shiiyu.moe";
const DEV_HOST: &str = "cupcake.shiiyu.moe";
const MAIN_SCHEME: &str = "https";

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct Config {
    website_version: String,
    auto_refresh: String,
}

fn get_current_host(app: &tauri::AppHandle) -> String {
    let config = load_config(app);
    if config.website_version == "dev" {
        DEV_HOST.to_string()
    } else {
        MAIN_HOST.to_string()
    }
}
#[cfg(not(mobile))]
const OFFLINE_RETRY_SECS: u64 = 3;

mod injector;

#[cfg(not(mobile))]
#[derive(Default)]
struct QuitFlag(AtomicBool);

#[cfg(not(mobile))]
#[allow(dead_code)]
struct TrayState(tauri::tray::TrayIcon);

#[cfg(not(mobile))]
#[allow(dead_code)]
struct TrayMenuItems {
    show: MenuItem<tauri::Wry>,
    hide: MenuItem<tauri::Wry>,
    home: MenuItem<tauri::Wry>,
    reload: MenuItem<tauri::Wry>,
    quit: MenuItem<tauri::Wry>,
}

// Injection toggles: keep the scripts available but disabled by default.
// Uncomment these to re-enable.
const INTERACTION_OVERRIDE_SCRIPT: &str = "injections/interaction_overrides.js";
const ENHANCED_SCRIPT: &str = "injections/skycrypt_enhanced.js";
const ESSENTIALS_SCRIPT: &str = "injections/skycrypt_essentials.js";
// const DEFAULT_THEME: &str = "default.json";

fn inject_interaction_overrides(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    let _ = injector::inject_from_resource(app, window, INTERACTION_OVERRIDE_SCRIPT);
}

fn inject_skycrypt_scripts(app: &tauri::AppHandle, window: &tauri::WebviewWindow) {
    inject_api_shim(window);
    let _ = injector::inject_from_resource(app, window, ESSENTIALS_SCRIPT);
    let _ = injector::inject_from_resource(app, window, ENHANCED_SCRIPT);
}

fn is_allowed_host(host: &str) -> bool {
    host == MAIN_HOST || host == DEV_HOST || host.ends_with(".shiiyu.moe")
}

fn inject_api_shim(window: &tauri::WebviewWindow) {
    let shim = r#"
        (function() {
            if (!window.api) {
                window.api = {
                    invoke: function(cmd, args) {
                        return window.__TAURI__.core.invoke(cmd, args);
                    }
                };
                console.log('[SkyCrypt Desktop] API shim initialized');
            }
        })();
    "#;
    let _ = window.eval(shim);
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

#[cfg(not(mobile))]
fn is_blank_url(url: &Url) -> bool {
    url.as_str() == "about:blank" || url.scheme() == "about"
}

#[cfg(not(mobile))]
fn show_main_window(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label != "splash" {
            let _ = window.show();
            let _ = window.set_focus();
            update_tray_menu(app);
            return;
        }
    }
}

#[cfg(not(mobile))]
fn reload_main_window(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label != "splash" {
            let _ = window.reload();
            return;
        }
    }
}

#[cfg(not(mobile))]
fn go_home(app: &tauri::AppHandle) {
    show_main_window(app);
    let host = get_current_host(app);
    for (label, window) in app.webview_windows() {
        if label != "splash" {
            let _ = window.eval(&format!(
                "window.location.replace('{MAIN_SCHEME}://{host}/')"
            ));
            return;
        }
    }
}

#[cfg(not(mobile))]
fn open_player_window(app: &tauri::AppHandle, player_name: &str) -> Result<(), String> {
    let sanitized = player_name.replace(|c: char| !c.is_alphanumeric() && c != '_', "");
    if sanitized.is_empty() {
        return Err("Invalid player name".to_string());
    }

    let sanitized_lower = sanitized.to_lowercase();
    let window_label = format!("player_{}", sanitized_lower);

    if let Some(existing) = app.get_webview_window(&window_label) {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    let host = get_current_host(app);
    let player_url = format!("{MAIN_SCHEME}://{host}/stats/{sanitized_lower}");
    log::info!("Opening player window: {} -> {}", sanitized, player_url);

    let is_hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
        || std::env::var("XDG_CURRENT_DESKTOP")
            .map(|v| v.to_lowercase().contains("hyprland"))
            .unwrap_or(false);

    let window = WebviewWindowBuilder::new(
        app,
        &window_label,
        WebviewUrl::External(player_url.parse().unwrap()),
    )
    .title(format!("SkyCrypt - {}", sanitized))
    .inner_size(1400.0, 900.0)
    .center()
    .resizable(true)
    .decorations(!is_hyprland)
    .zoom_hotkeys_enabled(false)
    .on_page_load({
        let app_handle = app.clone();
        move |window, payload| {
            if payload.event() == PageLoadEvent::Finished {
                inject_interaction_overrides(&app_handle, &window);
                inject_skycrypt_scripts(&app_handle, &window);
            }
        }
    })
    .build()
    .map_err(|e| format!("Failed to create window: {}", e))?;

    let app_handle = app.clone();
    let label = window_label.clone();
    window.on_window_event(move |event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            let should_quit = app_handle.state::<QuitFlag>().0.load(Ordering::SeqCst);
            if !should_quit {
                api.prevent_close();
                if let Some(w) = app_handle.get_webview_window(&label) {
                    let _ = w.hide();
                }
            }
        }
    });

    log::info!("Opened player window: {}", sanitized);
    Ok(())
}

#[cfg(not(mobile))]
fn hide_main_window(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label != "splash" {
            let _ = window.hide();
        }
    }
    update_tray_menu(app);
}

#[cfg(not(mobile))]
fn close_to_tray(app: &tauri::AppHandle) {
    for (label, window) in app.webview_windows() {
        if label != "splash" {
            let _ = window.hide();
        }
    }
    log::info!("Window closed to tray");
}

#[cfg(not(mobile))]
fn setup_logging(app: &tauri::AppHandle) {
    let log_dir = app.path().app_log_dir().expect("failed to get log dir");
    let _ = fs::create_dir_all(&log_dir);
    let log_path = log_dir.join("skycrypt.log");

    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {} {}] {}",
                chrono_lite(),
                record.level(),
                record.target(),
                record.args()
            )
        })
        .init();

    let log_path_for_panic = log_path.clone();
    std::panic::set_hook(Box::new(move |panic_info| {
        let msg = format!("[PANIC] {}\n", panic_info);
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path_for_panic)
        {
            let _ = file.write_all(msg.as_bytes());
        }
        eprintln!("{}", msg);
    }));

    log::info!("SkyCrypt Desktop started. Log file: {:?}", log_path);
}

#[cfg(not(mobile))]
fn chrono_lite() -> String {
    use std::time::SystemTime;
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = now.as_secs();
    let hours = (secs / 3600) % 24;
    let mins = (secs / 60) % 60;
    let secs = secs % 60;
    format!("{:02}:{:02}:{:02}", hours, mins, secs)
}

#[cfg(not(mobile))]
fn get_config_path(app: &tauri::AppHandle) -> std::path::PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_else(|_| std::path::PathBuf::from("."))
        .join("config.json")
}

#[cfg(not(mobile))]
fn load_config(app: &tauri::AppHandle) -> Config {
    let path = get_config_path(app);
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str::<Config>(&data) {
                return config;
            }
        }
    }
    Config {
        website_version: "stable".to_string(),
        auto_refresh: "off".to_string(),
    }
}

#[cfg(not(mobile))]
fn save_config(app: &tauri::AppHandle, config: &Config) -> Result<(), String> {
    let path = get_config_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let data = serde_json::to_string_pretty(config).map_err(|e| e.to_string())?;
    fs::write(&path, data).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg(not(mobile))]
#[tauri::command]
fn get_website_version(app: tauri::AppHandle) -> String {
    load_config(&app).website_version
}

#[cfg(not(mobile))]
#[tauri::command]
fn save_website_version(app: tauri::AppHandle, version: String) -> Result<(), String> {
    let mut config = load_config(&app);
    config.website_version = version;
    save_config(&app, &config)
}

#[cfg(not(mobile))]
#[tauri::command]
fn get_auto_refresh(app: tauri::AppHandle) -> String {
    load_config(&app).auto_refresh
}

#[cfg(not(mobile))]
#[tauri::command]
fn save_auto_refresh(app: tauri::AppHandle, interval: String) -> Result<(), String> {
    let mut config = load_config(&app);
    config.auto_refresh = interval;
    save_config(&app, &config)
}

#[cfg(not(mobile))]
#[tauri::command]
fn reset_config(app: tauri::AppHandle) -> Result<(), String> {
    let path = get_config_path(&app);
    if path.exists() {
        fs::remove_file(&path).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(not(mobile))]
#[tauri::command]
fn close_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("settings") {
        window.close().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(not(mobile))]
#[tauri::command]
fn open_settings_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(existing) = app.get_webview_window("settings") {
        let _ = existing.show();
        let _ = existing.set_focus();
        return Ok(());
    }

    let is_hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
        || std::env::var("XDG_CURRENT_DESKTOP")
            .map(|v| v.to_lowercase().contains("hyprland"))
            .unwrap_or(false);

    WebviewWindowBuilder::new(&app, "settings", WebviewUrl::App("settings".into()))
        .title("Settings")
        .inner_size(450.0, 520.0)
        .center()
        .resizable(false)
        .decorations(!is_hyprland)
        .build()
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[cfg(not(mobile))]
fn update_tray_menu(app: &tauri::AppHandle) {
    let items = app.state::<TrayMenuItems>();
    let _ = items.show.set_enabled(true);
    let _ = items.hide.set_enabled(true);
    let _ = items.reload.set_enabled(true);
    let _ = items.quit.set_enabled(true);
}

#[cfg(not(mobile))]
pub fn run(initial_player: Option<String>) {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_deep_link::init());
    let builder = builder.plugin(tauri_plugin_single_instance::init({
        move |app, args, _cwd| {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
            for arg in &args {
                if let Ok(url) = Url::parse(arg) {
                    let path = url.path().trim_start_matches('/');
                    if !path.is_empty() {
                        log::info!("Opening player from URL: {}", path);
                        let _ = open_player_window(app, path);
                    }
                } else if !arg.starts_with('-') {
                    log::info!("Opening player from arg: {}", arg);
                    let _ = open_player_window(app, arg);
                }
            }
        }
    }));

    builder
        .invoke_handler(tauri::generate_handler![
            get_website_version,
            save_website_version,
            get_auto_refresh,
            save_auto_refresh,
            reset_config,
            close_settings_window,
            open_settings_window,
        ])
        .setup(move |app| {
            #[cfg(not(mobile))]
            {
                setup_logging(app.handle());

                app.listen("deep-link://new-url", {
                    let app_handle = app.handle().clone();
                    move |event| {
                        let urls = event.payload();
                        log::info!("Deep link received: {}", urls);
                        if let Ok(url) = Url::parse(urls) {
                            let path = url.path();
                            let player = path.trim_start_matches('/');
                            if !player.is_empty() {
                                log::info!("Opening player from deep link: {}", player);
                                let _ = open_player_window(&app_handle, player);
                            }
                        }
                    }
                });

                app.manage(QuitFlag::default());
                let menu = Menu::new(app)?;
                let tray_show = MenuItem::with_id(app, "tray_show", "Show", true, None::<&str>)?;
                let tray_hide = MenuItem::with_id(app, "tray_hide", "Hide", true, None::<&str>)?;
                let tray_home = MenuItem::with_id(app, "tray_home", "Home", true, None::<&str>)?;
                let tray_reload =
                    MenuItem::with_id(app, "tray_reload", "Reload", true, None::<&str>)?;
                let tray_quit = MenuItem::with_id(app, "tray_quit", "Quit", true, None::<&str>)?;
                let tray_sep = PredefinedMenuItem::separator(app)?;

                menu.append(&tray_show)?;
                menu.append(&tray_hide)?;
                menu.append(&tray_home)?;
                menu.append(&tray_reload)?;
                menu.append(&tray_sep)?;
                menu.append(&tray_quit)?;

                let mut tray_builder = TrayIconBuilder::new()
                    .menu(&menu)
                    .tooltip("SkyCrypt Desktop")
                    .on_menu_event(|app, event| match event.id().as_ref() {
                        "tray_show" => {
                            show_main_window(app);
                        }
                        "tray_hide" => {
                            hide_main_window(app);
                        }
                        "tray_home" => {
                            go_home(app);
                        }
                        "tray_reload" => {
                            reload_main_window(app);
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
                    });

                if let Some(icon) = app.default_window_icon().cloned() {
                    tray_builder = tray_builder.icon(icon);
                }

                let tray = tray_builder.build(app)?;
                app.manage(TrayState(tray));
                app.manage(TrayMenuItems {
                    show: tray_show,
                    hide: tray_hide,
                    home: tray_home,
                    reload: tray_reload,
                    quit: tray_quit,
                });
                update_tray_menu(app.handle());

                let app_handle = app.handle().clone();
                let is_hyprland = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").is_ok()
                    || std::env::var("XDG_CURRENT_DESKTOP")
                        .map(|v| v.to_lowercase().contains("hyprland"))
                        .unwrap_or(false);

                if initial_player.is_none() {
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
                }

                let main_created = Arc::new(AtomicBool::new(false));
                let create_main_window: Arc<dyn Fn(&tauri::AppHandle) + Send + Sync> = {
                    let main_created = Arc::clone(&main_created);
                    Arc::new(move |app_handle: &tauri::AppHandle| {
                        if main_created.swap(true, Ordering::SeqCst) {
                            return;
                        }
                        let host = get_current_host(app_handle);
                        let window = WebviewWindowBuilder::new(
                            app_handle,
                            "main",
                            WebviewUrl::External(
                                format!("{MAIN_SCHEME}://{host}").parse().unwrap(),
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
                                    inject_skycrypt_scripts(&app_handle, &window);
                                    if let Some(splash) = app_handle.get_webview_window("splash") {
                                        let _ = splash.close();
                                    }
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                    update_tray_menu(&app_handle);
                                }
                            }
                        })
                        .on_navigation({
                            move |url| {
                                if is_blank_url(url) {
                                    return false;
                                }
                                let allowed = url.scheme() == MAIN_SCHEME
                                    && url.host_str().map(is_allowed_host).unwrap_or(false);
                                if allowed {
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

                        let app_handle_for_event = app_handle.clone();
                        window.on_window_event(move |event| {
                            if let WindowEvent::CloseRequested { api, .. } = event {
                                let should_quit = app_handle_for_event
                                    .state::<QuitFlag>()
                                    .0
                                    .load(Ordering::SeqCst);
                                if !should_quit {
                                    api.prevent_close();
                                    close_to_tray(&app_handle_for_event);
                                }
                            }
                        });
                    })
                };

                tauri::async_runtime::spawn_blocking({
                    let app_handle = app_handle.clone();
                    let main_created = Arc::clone(&main_created);
                    let create_main_window = Arc::clone(&create_main_window);
                    let init_player = initial_player.clone();
                    move || {
                        if let Some(raw_arg) = &init_player {
                            let player = if let Ok(url) = Url::parse(raw_arg) {
                                url.path().trim_start_matches('/').to_string()
                            } else {
                                raw_arg.clone()
                            };
                            if !player.is_empty() {
                                log::info!("Opening player from args: {}", player);
                                let _ = open_player_window(&app_handle, &player);
                            }
                            return;
                        }
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
                        inject_skycrypt_scripts(&app_handle, window);
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

#[cfg(mobile)]
#[tauri::mobile_entry_point]
pub fn run() {
    crate::run(None)
}
