use tauri::{plugin::PluginApi, AppHandle, Manager, Runtime};

use crate::{config::Config, storage::StorageState};

pub fn init<R: Runtime>(
    app: &AppHandle<R>,
    api: PluginApi<R, Config>,
) -> crate::Result<PluginZustandStorage<R>> {
    // manage state so it is accessible by the commands
    app.manage(StorageState::new(&api.config().path));
    Ok(PluginZustandStorage(app.clone()))
}

/// Access to the plugin-zustand-storage APIs.
pub struct PluginZustandStorage<R: Runtime>(AppHandle<R>);

impl<R: Runtime> PluginZustandStorage<R> {
    pub fn get_item(&self, key: &str) -> crate::Result<Option<String>> {
        let state = self.0.state::<StorageState>();
        let storage = state.lock().unwrap();
        Ok(storage.get_item(key)?)
    }

    pub fn set_item(&self, key: &str, value: &str) -> crate::Result<()> {
        let state = self.0.state::<StorageState>();
        let storage = state.lock().unwrap();
        storage.set_item(key, value)
    }

    pub fn remove_item(&self, key: &str) -> crate::Result<()> {
        let state = self.0.state::<StorageState>();
        let storage = state.lock().unwrap();
        storage.remove_item(key)
    }
}
