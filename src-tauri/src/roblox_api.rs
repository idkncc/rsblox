use std::sync::Mutex;

use roboat::discovery::RecommendationsTopic;
use roboat::friends::FriendUserInformation;
use roboat::games::{GameDetail, GameMedia, GameServer, PlaceDetails};
use roboat::presence::UserPresence;
use roboat::ClientBuilder;
use serde::{Deserialize, Serialize};
use tauri::{
    api,
    plugin::{Builder, TauriPlugin},
    AppHandle, Manager, Runtime, State,
};

use super::commands;

#[derive(Default)]
struct RobloxApiState(Mutex<String>);

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
async fn get_me(state: State<'_, RobloxApiState>) -> Result<ClientInfo, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    Ok(ClientInfo {
        user_id: client.user_id().await.map_err(|err| err.to_string())?,
        username: client.username().await.map_err(|err| err.to_string())?,
        display_name: client.display_name().await.map_err(|err| err.to_string())?,
        robux: client.robux().await.map_err(|err| err.to_string())?,
    })
}

#[tauri::command]
async fn friends_list(
    state: State<'_, RobloxApiState>,
) -> Result<Vec<FriendUserInformation>, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    let user_id = match client.user_id().await {
        Ok(v) => v,
        Err(err) => return Err(err.to_string()),
    };

    client
        .friends_list(user_id)
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

#[tauri::command]
async fn place_details(
    state: State<'_, RobloxApiState>,
    place_id: u64,
) -> Result<PlaceDetails, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .place_details(place_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn game_media(universe_id: u64) -> Result<Vec<GameMedia>, String> {
    let client = ClientBuilder::new().build();

    client
        .game_media(universe_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn game_details(
    state: State<'_, RobloxApiState>,
    universe_id: u64,
) -> Result<GameDetail, String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .game_details(universe_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn game_servers(
    state: State<'_, RobloxApiState>,
    place_id: u64,
    cursor: Option<String>,
) -> Result<(Vec<GameServer>, Option<String>), String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .game_servers(place_id, None, None, cursor)
        .await
        .map_err(|err| err.to_string())
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
fn open_place<R: Runtime>(app: AppHandle<R>, place_id: u64) -> Result<(), String> {
    api::shell::open(
        &app.shell_scope(),
        format!("roblox://experiences/start?placeId={}", place_id),
        None,
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

#[tauri::command]
fn open_server<R: Runtime>(
    app: AppHandle<R>,
    place_id: u64,
    game_id: String,
) -> Result<(), String> {
    api::shell::open(
        &app.shell_scope(),
        format!(
            "roblox://experiences/start?placeId={}&gameInstanceId={}",
            place_id, game_id
        ),
        None,
    )
    .map_err(|err| err.to_string())?;

    Ok(())
}

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("roblox-api")
        .invoke_handler(tauri::generate_handler![
            auth,
            presence,
            get_me,
            friends_list,
            recommendations,
            get_presences,
            game_media,
            game_details,
            game_servers,
            place_details,
            commands::thumbnail_url_bulk,
            commands::token_thumbnail_url_bulk,
            open_place,
            open_server,
        ])
        .setup(|app_handle| {
            app_handle.manage(RobloxApiState::default());
            Ok(())
        })
        .build()
}
