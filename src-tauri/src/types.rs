use std::sync::Mutex;

use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct RobloxApiState(pub Mutex<String>);

#[derive(Serialize, Deserialize)]
pub struct ClientInfo {
    pub user_id: u64,
    pub username: String,
    pub display_name: String,
    pub robux: u64,
}
