use config::Config;

use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

mod storage;
pub use models::*;
mod config;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::PluginZustandStorage;
#[cfg(mobile)]
use mobile::PluginZustandStorage;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the plugin-zustand-storage APIs.
pub trait PluginZustandStorageExt<R: Runtime> {
    fn plugin_zustand_storage(&self) -> &PluginZustandStorage<R>;
}

impl<R: Runtime, T: Manager<R>> crate::PluginZustandStorageExt<R> for T {
    fn plugin_zustand_storage(&self) -> &PluginZustandStorage<R> {
        self.state::<PluginZustandStorage<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("plugin-zustand-storage")
        .invoke_handler(tauri::generate_handler![
            commands::set_item,
            commands::get_item,
            commands::remove_item
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let plugin_zustand_storage = mobile::init(app, api)?;
            #[cfg(desktop)]
            let plugin_zustand_storage = desktop::init(app, api)?;
            app.manage(plugin_zustand_storage);

            Ok(())
        })
        .build()
}
