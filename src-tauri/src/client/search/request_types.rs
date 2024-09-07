use serde::{Deserialize, Serialize};

/// Model, representing omni search response
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OnmiSearchResponse {
    pub search_results: Vec<SearchResultRaw>,
    pub next_page_token: String,
    // pub filteredSearchQuery: null,
    pub vertical: String, // pub sorts: null
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultRaw {
    pub content_group_type: String,
    pub contents: Vec<SearchContentRaw>,
    pub topic_id: String,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchContentRaw {
    pub universe_id: u64,
    pub name: String,
    pub description: String,
    pub player_count: usize,
    pub total_up_votes: usize,
    pub total_down_votes: usize,
    pub emphasis: bool,
    pub is_sponsored: bool,
    pub creator_id: u64,
    pub creator_name: String,
    pub creator_has_verified_badge: bool,
    pub root_place_id: u64,
    pub minimum_age: i32,
    pub age_recommendation_display_name: String,
    pub content_type: String,
    pub content_id: u64,
}
