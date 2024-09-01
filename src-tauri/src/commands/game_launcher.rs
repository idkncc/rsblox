use tauri::{api, AppHandle, Manager, Runtime};

#[tauri::command]
pub fn open_place<R: Runtime>(app: AppHandle<R>, place_id: u64) -> Result<(), String> {
    api::shell::open(
        &app.shell_scope(),
        format!("roblox://experiences/start?placeId={}", place_id),
        None,
    )
    .map_err(|err| err.to_string())
}

#[tauri::command]
pub fn open_server<R: Runtime>(
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
    .map_err(|err| err.to_string())
}
