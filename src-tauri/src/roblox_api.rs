use roboat::discovery::RecommendationsTopic;
use roboat::presence::UserPresence;
use roboat::ClientBuilder;
use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

use crate::types::RobloxApiState;

use super::commands;

#[derive(Serialize, Deserialize)]
struct ClientInfo {
    user_id: u64,
    username: String,
    display_name: String,
    robux: u64,
}

/// Log in method
#[tauri::command]
fn auth<R: Runtime>(_app: AppHandle<R>, state: State<'_, RobloxApiState>, roblosecurity: String) {
    *state.0.lock().unwrap() = roblosecurity.clone();
}

#[tauri::command]
async fn presence(state: State<'_, RobloxApiState>) -> Result<(), String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .register_presence()
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
async fn get_presences(
    state: State<'_, RobloxApiState>,
    user_ids: Vec<u64>,
) -> Result<Vec<UserPresence>, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .get_presence(user_ids)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn recommendations(
    state: State<'_, RobloxApiState>,
) -> Result<Vec<RecommendationsTopic>, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .omni_recommendations()
        .await
        .map_err(|err| err.to_string())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("roblox-api")
        .invoke_handler(tauri::generate_handler![
            auth,
            presence,
            recommendations,
            get_presences,
            // users.rs
            commands::get_me,
            commands::friends_list,
            // games.rs
            commands::game_media,
            commands::game_details,
            commands::game_servers,
            commands::place_details,
            // thumbnails.rs
            commands::thumbnail_url_bulk,
            commands::token_thumbnail_url_bulk,
            // game_launcher.rs
            commands::open_place,
            commands::open_server,
        ])
        .setup(|app_handle| {
            app_handle.manage(RobloxApiState::default());
            Ok(())
        })
        .build()
}
