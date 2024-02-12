use tauri::{command, AppHandle, Runtime, State, Window};

use crate::{desktop::PluginZustandStorage, Result};

#[command]
pub(crate) async fn set_item<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, PluginZustandStorage<R>>,
    key: &str,
    value: &str,
) -> Result<()> {
    state.set_item(key, value)
}

#[command]
pub(crate) async fn get_item<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, PluginZustandStorage<R>>,
    key: &str,
) -> Result<Option<String>> {
    state.get_item(key)
}

#[command]
pub(crate) async fn remove_item<R: Runtime>(
    _app: AppHandle<R>,
    _window: Window<R>,
    state: State<'_, PluginZustandStorage<R>>,
    key: &str,
) -> Result<()> {
    state.remove_item(key)
}
