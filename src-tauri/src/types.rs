use crate::client::RobloxApi;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

#[derive(Default)]
pub struct RobloxApiState(pub RwLock<RobloxApi>);

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
    pub robux: u64,
}

#[derive(Serialize, Deserialize)]
pub struct UserProfileStats {
    pub friends: usize,
    pub followers: usize,
    pub followings: usize,
}
