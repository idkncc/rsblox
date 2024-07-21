use roboat::{
    games::{GameDetail, GameMedia, GameServer, PlaceDetails, ServerType},
    ClientBuilder,
};
use tauri::State;

use crate::types::RobloxApiState;

#[tauri::command]
pub async fn place_details(
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
pub async fn game_media(universe_id: u64) -> Result<Vec<GameMedia>, String> {
    let client = ClientBuilder::new().build();

    client
        .game_media(universe_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn game_details(
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
pub async fn game_servers(
    state: State<'_, RobloxApiState>,
    place_id: u64,
    servers_type: ServerType,
    cursor: Option<String>,
) -> Result<(Vec<GameServer>, Option<String>), String> {
    let cookie = { state.0.lock().unwrap().clone() };

    let client = ClientBuilder::new().roblosecurity(cookie).build();

    client
        .game_servers(place_id, Some(servers_type), None, None, cursor)
        .await
        .map_err(|err| err.to_string())
}
