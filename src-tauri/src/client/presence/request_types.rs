use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetPresenceReqBody {
    pub user_ids: Vec<u64>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GetPresenceResponse {
    pub user_presences: Vec<UserPresenceRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct UserPresenceRaw {
    pub user_id: u64,
    pub user_presence_type: i32,
    pub last_online: String,
    pub last_location: String,

    pub place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
}
