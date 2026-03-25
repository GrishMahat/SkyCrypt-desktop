//! JavaScript injection utilities for webview scripts
//!
//! Provides functions to inject JavaScript files from resources into webviews.

use std::fs;
use tauri::{path::BaseDirectory, AppHandle, Manager, WebviewWindow};

/// Injects a JavaScript file from resources into the webview
pub fn inject_from_resource(
    app: &AppHandle,
    window: &WebviewWindow,
    resource_path: &str,
) -> Result<(), String> {
    let path = app
        .path()
        .resolve(resource_path, BaseDirectory::Resource)
        .map_err(|err| format!("failed to resolve resource path: {err}"))?;
    let script =
        fs::read_to_string(&path).map_err(|err| format!("failed to read inject script: {err}"))?;
    window
        .eval(&script)
        .map_err(|err| format!("failed to eval inject script: {err}"))
}

/// Injects a JavaScript file from resources with optional string replacements
#[allow(dead_code)]
pub fn inject_from_resource_with_replacements(
    app: &AppHandle,
    window: &WebviewWindow,
    resource_path: &str,
    replacements: &[(&str, &str)],
) -> Result<(), String> {
    let path = app
        .path()
        .resolve(resource_path, BaseDirectory::Resource)
        .map_err(|err| format!("failed to resolve resource path: {err}"))?;
    let mut script =
        fs::read_to_string(&path).map_err(|err| format!("failed to read inject script: {err}"))?;

    for (key, value) in replacements {
        script = script.replace(key, value);
    }

    window
        .eval(&script)
        .map_err(|err| format!("failed to eval inject script: {err}"))
}
