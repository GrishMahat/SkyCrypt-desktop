use std::fs;
use tauri::{path::BaseDirectory, AppHandle, Manager, WebviewWindow};

pub fn inject_from_resource(
    app: &AppHandle,
    window: &WebviewWindow,
    resource_path: &str,
) -> Result<(), String> {
    let path = app
        .path()
        .resolve(resource_path, BaseDirectory::Resource)
        .map_err(|err| format!("failed to resolve resource path: {err}"))?;
    let script = fs::read_to_string(&path)
        .map_err(|err| format!("failed to read inject script: {err}"))?;
    window
        .eval(&script)
        .map_err(|err| format!("failed to eval inject script: {err}"))
}
