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
use desktop::ZustandStorage;
#[cfg(mobile)]
use mobile::ZustandStorage;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the plugin-zustand-storage APIs.
pub trait ZustandStorageExt<R: Runtime> {
    fn zustand_storage(&self) -> &ZustandStorage<R>;
}

impl<R: Runtime, T: Manager<R>> crate::ZustandStorageExt<R> for T {
    fn zustand_storage(&self) -> &ZustandStorage<R> {
        self.state::<ZustandStorage<R>>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R, Config> {
    Builder::<R, Config>::new("zustand-storage")
        .invoke_handler(tauri::generate_handler![
            commands::set_item,
            commands::get_item,
            commands::remove_item
        ])
        .setup(|app, api| {
            #[cfg(mobile)]
            let zustand_storage = mobile::init(app, api)?;
            #[cfg(desktop)]
            let zustand_storage = desktop::init(app, api)?;
            app.manage(zustand_storage);

            Ok(())
        })
        .build()
}
