use crate::client::thumbnails::{ThumbnailSize, ThumbnailType};
use tauri::State;

use crate::types::RobloxApiState;

#[tauri::command(async)]
pub async fn thumbnail_url_bulk(
    state: State<'_, RobloxApiState>,
    ids: Vec<u64>,
    thumbnail_size: ThumbnailSize,
    thumbnail_type: ThumbnailType,
) -> Result<Vec<String>, String> {
    let client = state.0.read().await;

    client
        .thumbnail_url_bulk(ids, thumbnail_size, thumbnail_type)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn token_thumbnail_url_bulk(
    state: State<'_, RobloxApiState>,
    tokens: Vec<String>,
    thumbnail_size: ThumbnailSize,
    thumbnail_type: ThumbnailType,
) -> Result<Vec<String>, String> {
    let client = state.0.read().await;

    client
        .token_thumbnail_url_bulk(tokens, thumbnail_size, thumbnail_type)
        .await
        .map_err(|err| err.to_string())
}
