use serde::{Deserialize, Serialize};

use super::{RobloxApi, RobloxError};

mod request_types;

const OMNI_RECOMMENDATIONS_API: &str = "https://apis.roblox.com/discovery-api/omni-recommendation";

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum TreatmentType {
    /// Carousel of friends
    #[default]
    FriendCarousel,

    /// Carousel (wide cards 16:9)
    Carousel,

    /// Grid (square cards)
    SortlessGrid,
}

impl TryFrom<String> for TreatmentType {
    type Error = RobloxError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FriendCarousel" => Ok(Self::FriendCarousel),
            "Carousel" => Ok(Self::Carousel),
            "SortlessGrid" => Ok(Self::SortlessGrid),
            _ => Err(RobloxError::MalformedResponse),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct RecommendationsTopic {
    /// Topic id
    #[serde(alias = "topicId")]
    pub topic_id: u64,

    /// Topic / Title
    pub topic: Option<String>,

    /// Subtitle
    pub subtitle: Option<String>,

    /// Type of topic
    ///
    /// Can be:
    ///  - "FriendCarousel" (friends),
    ///  - "Carousel" (wide cards),
    ///  - "SortlessGrid" (grid of small square cards)
    pub treatment_type: TreatmentType,

    /// Array of recommendations
    pub recommendation_list: Vec<Recommendation>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct Recommendation {
    pub universe_id: u64,
    pub root_place_id: u64,

    pub name: String,
    pub description: Option<String>,

    pub total_up_votes: u64,
    pub total_down_votes: u64,
    pub player_count: usize,
}

impl RobloxApi {
    /// Gets sections from Home using <https://apis.roblox.com/discovery-api/omni-recommendation>.
    pub async fn omni_recommendations(&self) -> Result<Vec<RecommendationsTopic>, RobloxError> {
        match self.omni_recommendations_internal().await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.omni_recommendations_internal().await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use reqwest::header;

    use crate::client::{RobloxApi, RobloxError, XCSRF_HEADER};

    use super::{
        request_types::OmniRecommendationsResponse, Recommendation, RecommendationsTopic,
        TreatmentType, OMNI_RECOMMENDATIONS_API,
    };

    impl RobloxApi {
        pub(super) async fn omni_recommendations_internal(
            &self,
        ) -> Result<Vec<RecommendationsTopic>, RobloxError> {
            let cookie = self.cookie_string().await?;

            let body = serde_json::json!({
                "pageType": "Home",
                "sessionId": "fbf5b8ae-3b7e-4cc6-b387-324743f04036"
            });

            let request_result = self
                .reqwest_client
                .post(OMNI_RECOMMENDATIONS_API)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .json(&body)
                .send()
                .await;

            let response = Self::validate_request_result(request_result).await?;
            let raw = Self::parse_to_raw::<OmniRecommendationsResponse>(response).await?;

            let mut topics = Vec::new();

            for raw_topic in raw.sorts {
                if raw_topic.recommendation_list.is_none() {
                    continue;
                }

                let mut recommendation_list = Vec::new();

                for raw_recommend in raw_topic.recommendation_list.unwrap() {
                    let metadata = raw
                        .content_metadata
                        .game
                        .get(&raw_recommend.content_id.to_string())
                        .ok_or(RobloxError::MalformedResponse)?;

                    recommendation_list.push(Recommendation {
                        universe_id: metadata.universe_id,
                        root_place_id: metadata.root_place_id,
                        name: metadata.name.clone(),
                        description: metadata.description.clone(),
                        total_up_votes: metadata.total_up_votes,
                        total_down_votes: metadata.total_down_votes,
                        player_count: metadata.player_count,
                    })
                }

                topics.push(RecommendationsTopic {
                    topic_id: raw_topic.topic_id,
                    topic: raw_topic.topic,
                    subtitle: raw_topic.subtitle,
                    treatment_type: TreatmentType::try_from(raw_topic.treatment_type)?,
                    recommendation_list,
                })
            }

            // raw.sorts

            // We don't care about the response, just that it's a status code 200.
            Ok(topics)
        }
    }
}
