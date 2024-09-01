use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GamesDetailsResponse {
    pub data: Vec<GameDetailRaw>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameDetailRaw {
    pub id: u64,
    pub root_place_id: u64,

    pub name: String,
    pub description: String,
    pub source_name: String,
    pub source_description: String,

    pub creator: GameCreatorRaw,

    pub price: Option<i32>,

    pub allowed_gear_genres: Vec<String>,
    pub allowed_gear_categories: Vec<String>,
    pub is_genre_enforced: bool,
    pub copying_allowed: bool,

    pub playing: u64,
    pub visits: u64,
    pub max_players: u64,
    pub created: String,
    pub updated: String,

    pub studio_access_to_apis_allowed: bool,
    pub create_vip_servers_allowed: bool,

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    pub universe_avatar_type: String,

    pub genre: String,
    pub is_all_genre: bool,

    pub is_favorited_by_user: bool,
    pub favorited_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameCreatorRaw {
    pub id: u64,
    pub name: String,

    #[serde(alias = "type")]
    pub creator_type: String,

    #[serde(alias = "isRNVAccount")]
    pub is_rnv_account: bool,

    pub has_verified_badge: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceDetailsRaw {
    pub place_id: u64,
    pub name: String,
    pub description: String,
    pub source_name: String,
    pub source_description: String,
    pub url: String,

    pub is_playable: bool,
    pub reason_prohibited: String,
    pub price: i32,
    pub image_token: String,

    pub builder: String,
    pub builder_id: u64,
    pub has_verified_badge: bool,

    pub universe_id: u64,
    pub universe_root_place_id: u64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameMediaResponse {
    pub data: Vec<GameMediaRaw>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GameMediaRaw {
    pub asset_type_id: u64,
    pub asset_type: String,
    pub approved: bool,

    pub image_id: Option<u64>,
    pub alt_text: Option<String>,
    pub video_hash: Option<String>,
    pub video_title: Option<String>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GameServersResponse {
    pub previous_page_cursor: Option<String>,
    pub next_page_cursor: Option<String>,
    pub data: Vec<GameServerRaw>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct GameServerRaw {
    pub id: String,
    pub max_players: usize,
    pub playing: usize,

    pub player_tokens: Vec<String>,

    // pub players: Vec<String>,
    pub fps: f32,
    pub ping: u64,
}
