use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct OmniRecommendationsResponse {
    /// AKA, array of topics
    pub sorts: Vec<RecommendationsTopicRaw>,

    pub content_metadata: RecommendationsContentMetadataRaw,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationsTopicRaw {
    /// Title
    pub topic: Option<String>,

    /// Subtitle
    pub subtitle: Option<String>,

    /// Topic id
    pub topic_id: u64,

    /// Type of topic
    ///
    /// Can be:
    ///  - "FriendCarousel" (friends),
    ///  - "Carousel" (wide cards),
    ///  - "SortlessGrid" (grid of small square cards)
    pub treatment_type: String,

    /// Array of recommendations
    pub recommendation_list: Option<Vec<RecommendationRaw>>,

    // pub next_page_token_for_topic: null,
    pub number_of_rows: i32,
    // pub topic_layout_data: {}
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecommendationRaw {
    /// Type
    ///
    /// Can be:
    ///  - "Game"
    pub content_type: String,

    pub content_id: u64,

    pub content_string_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RecommendationsContentMetadataRaw {
    pub game: HashMap<String, GameContentMetadataRaw>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameContentMetadataRaw {
    pub universe_id: u64,
    pub root_place_id: u64,
    pub name: String,
    pub description: Option<String>,

    pub total_up_votes: u64,
    pub total_down_votes: u64,
    pub player_count: usize,
}
