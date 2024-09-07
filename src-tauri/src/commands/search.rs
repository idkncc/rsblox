use tauri::State;

use crate::{client::search::SearchContent, types::RobloxApiState};

#[tauri::command(async)]
pub async fn omni_search(
    state: State<'_, RobloxApiState>,
    query_string: String,
) -> Result<Vec<SearchContent>, String> {
    let client = state.0.read().await;

    client
        .omni_search(query_string, None)
        .await
        .map_err(|e| e.to_string())
}
