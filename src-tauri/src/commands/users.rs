use tauri::State;

use crate::client::friends::{FriendStatus, FriendUserInformation};
use crate::client::users::UserDetails;
use crate::types::UserProfileStats;
use crate::{
    client::RobloxError,
    types::{ClientInfo, RobloxApiState},
};

#[tauri::command]
pub async fn get_me(state: State<'_, RobloxApiState>) -> Result<ClientInfo, String> {
    let client = state.0.read().await;

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
pub async fn get_user(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<UserDetails, String> {
    let client = state.0.read().await;

    client
        .user_details(user_id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command(async)]
pub async fn get_user_stats(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<UserProfileStats, String> {
    let client = state.0.read().await;

    Ok(UserProfileStats {
        friends: client
            .friends_count(user_id)
            .await
            .map_err(|e| e.to_string())?,
        followers: client
            .followers_count(user_id)
            .await
            .map_err(|e| e.to_string())?,
        followings: client
            .followings_count(user_id)
            .await
            .map_err(|e| e.to_string())?,
    })
}

#[tauri::command(async)]
pub async fn friend_status(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<FriendStatus, String> {
    let client = state.0.read().await;

    client
        .friend_status(user_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn friends_list(
    state: State<'_, RobloxApiState>,
) -> Result<Vec<FriendUserInformation>, String> {
    let client = state.0.read().await;

    client
        .friends_list(client.user_id().await.map_err(|err| err.to_string())?)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn users_friends_list(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<Vec<FriendUserInformation>, String> {
    let client = state.0.read().await;

    client
        .friends_list(user_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn friend(state: State<'_, RobloxApiState>, user_id: u64) -> Result<(), String> {
    let client = state.0.read().await;

    client
        .send_friend_request(user_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn unfriend(state: State<'_, RobloxApiState>, user_id: u64) -> Result<(), String> {
    let client = state.0.read().await;

    client
        .unfriend(user_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn accept_friend_request(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<(), String> {
    let client = state.0.read().await;

    client
        .accept_friend_request(user_id)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command(async)]
pub async fn decline_friend_request(
    state: State<'_, RobloxApiState>,
    user_id: u64,
) -> Result<(), String> {
    let client = state.0.read().await;

    client
        .decline_friend_request(user_id)
        .await
        .map_err(|err| err.to_string())
}
