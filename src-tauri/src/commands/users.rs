use crate::client::friends::FriendUserInformation;
use crate::{
    client::RobloxError,
    types::{ClientInfo, RobloxApiState},
};
use tauri::State;

#[tauri::command]
pub async fn get_me(state: State<'_, RobloxApiState>) -> Result<ClientInfo, String> {
    let client = state.0.read().await;
    println!("Checking cookie: {:?}", client.cookie_string().await);
    let Some(user_info) = client.user_information().await else {
        return Err(RobloxError::InvalidRoblosecurity.to_string());
    };

    Ok(ClientInfo {
        user_id: user_info.user_id,
        username: user_info.username,
        display_name: user_info.display_name,
        robux: client.robux().await.map_err(|err| err.to_string())?,
    })
}

#[tauri::command(async)]
pub async fn friends_list(
    state: State<'_, RobloxApiState>,
) -> Result<Vec<FriendUserInformation>, String> {
    let client = state.0.read().await;
    let Some(user_info) = client.user_information().await else {
        return Err(RobloxError::InvalidRoblosecurity.to_string());
    };

    client
        .friends_list(user_info.user_id)
        .await
        .map_err(|err| err.to_string())
}
