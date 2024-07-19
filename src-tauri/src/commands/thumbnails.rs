use roboat::{
    thumbnails::{ThumbnailSize, ThumbnailType},
    ClientBuilder,
};

#[tauri::command]
pub async fn thumbnail_url_bulk(
    ids: Vec<u64>,
    thumbnail_size: ThumbnailSize,
    thumbnail_type: ThumbnailType,
) -> Result<Vec<String>, String> {
    let client = ClientBuilder::new().build();

    client
        .thumbnail_url_bulk(ids, thumbnail_size, thumbnail_type)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
pub async fn token_thumbnail_url_bulk(
    tokens: Vec<String>,
    thumbnail_size: ThumbnailSize,
    thumbnail_type: ThumbnailType,
) -> Result<Vec<String>, String> {
    let client = ClientBuilder::new().build();

    client
        .token_thumbnail_url_bulk(tokens, thumbnail_size, thumbnail_type)
        .await
        .map_err(|err| err.to_string())
}
