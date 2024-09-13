use tauri::plugin::{Builder, TauriPlugin};
use tauri::{AppHandle, Manager, Runtime};

use utils::TrayGame;

pub mod utils;

#[tauri::command]
fn tray_update<R: Runtime>(app: AppHandle<R>, games: Vec<TrayGame>) {
    let tray_handle = app.tray_handle();

    tray_handle
        .set_menu(utils::get_system_tray(games).menu.expect("unreachable"))
        .expect("Cannot update tray")
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("tray-api")
        .invoke_handler(tauri::generate_handler![tray_update])
        // .setup(|app_handle| {
        // app_handle.manage(RobloxApiState::default());
        // Ok(())
        // })
        .build()
}
