use serde::{Deserialize, Serialize};

use super::{RobloxApi, RobloxError};

mod request_types;

const OMNI_SEARCH_API: &str = "https://apis.roblox.com/search-api/omni-search?searchQuery={search_query}&pageToken={page_token}&sessionId={session_id}&pageType=all";

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct SearchContent {
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

impl RobloxApi {
    /// Get list of all friends for the specified user using <https://friends.roblox.com/v1/users/{userId}/friends>.
    pub async fn omni_search(
        &self,
        search_query: String,
        page_token: Option<String>,
    ) -> Result<Vec<SearchContent>, RobloxError> {
        let formatted_url = OMNI_SEARCH_API
            .replace("{search_query}", &search_query.to_string())
            .replace("{page_token}", &page_token.unwrap_or("".to_string()));

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::OnmiSearchResponse>(response).await?;

        let mut search_results = Vec::new();

        for content_raw in raw.search_results {
            let content_raw = content_raw.contents.first().unwrap();

            let search_content = SearchContent {
                universe_id: content_raw.universe_id,
                name: content_raw.name.to_string(),
                description: content_raw.description.to_string(),
                player_count: content_raw.player_count,
                total_up_votes: content_raw.total_up_votes,
                total_down_votes: content_raw.total_down_votes,
                emphasis: content_raw.emphasis,
                is_sponsored: content_raw.is_sponsored,
                creator_id: content_raw.creator_id,
                creator_name: content_raw.creator_name.to_string(),
                creator_has_verified_badge: content_raw.creator_has_verified_badge,
                root_place_id: content_raw.root_place_id,
                minimum_age: content_raw.minimum_age,
                age_recommendation_display_name: content_raw
                    .age_recommendation_display_name
                    .to_string(),
                content_type: content_raw.content_type.to_string(),
                content_id: content_raw.content_id,
            };

            search_results.push(search_content);
        }

        Ok(search_results)
    }
}
