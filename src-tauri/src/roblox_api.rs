use serde::{Deserialize, Serialize};
use tauri::{
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

use crate::{
    client::{discovery::RecommendationsTopic, presence::UserPresence},
    types::RobloxApiState,
};

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
async fn auth<R: Runtime>(
    _app: AppHandle<R>,
    state: State<'_, RobloxApiState>,
    roblosecurity: String,
) -> Result<(), String> {
    let client = state.0.write().await;

    client.set_cookie(roblosecurity.clone()).await;
    Ok(())
}

#[tauri::command]
async fn is_authed(state: State<'_, RobloxApiState>) -> Result<bool, ()> {
    let client = state.0.read().await;

    Ok(client.cookie_string().await.is_ok())
}

#[tauri::command(async)]
async fn presence(state: State<'_, RobloxApiState>) -> Result<(), String> {
    let client = state.0.read().await;

    client
        .register_presence()
        .await
        .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command(async)]
async fn get_presences(
    state: State<'_, RobloxApiState>,
    user_ids: Vec<u64>,
) -> Result<Vec<UserPresence>, String> {
    let client = state.0.read().await;

    client
        .get_presence(user_ids)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
async fn recommendations(
    state: State<'_, RobloxApiState>,
) -> Result<Vec<RecommendationsTopic>, String> {
    let client = state.0.read().await;

    client
        .omni_recommendations()
        .await
        .map_err(|err| err.to_string())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("roblox-api")
        .invoke_handler(tauri::generate_handler![
            auth,
            is_authed,
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
