use request_types::{
    GameMediaResponse, GameServersResponse, GamesDetailsResponse, PlaceDetailsRaw,
};
use reqwest::header;
use reqwest::header::HeaderValue;
use serde::{Deserialize, Serialize};

use super::{RobloxApi, RobloxError};

mod request_types;

const GAMES_DETAILS_API: &str = "https://games.roblox.com/v1/games?universeIds={universe_ids}";
const PLACE_DETAILS_API: &str =
    "https://games.roblox.com/v1/games/multiget-place-details?placeIds={place_ids}";

const GAME_MEDIA_API: &str = "https://games.roblox.com/v2/games/{universe_id}/media";
const GAME_SERVERS_API: &str = "https://games.roblox.com/v1/games/{place_id}/servers/{servers_type}?sortOrder={sort_order}&excludeFullGames={exclude_full_games}&limit=10";

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum AvatarType {
    MorphToR6,
    MorphToR15,

    #[default]
    PlayerChoice,
}

impl TryFrom<String> for AvatarType {
    type Error = RobloxError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "MorphToR6" => Ok(Self::MorphToR6),
            "MorphToR15" => Ok(Self::MorphToR15),
            "PlayerChoice" => Ok(Self::PlayerChoice),
            _ => Err(RobloxError::MalformedResponse),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum GameMediaType {
    #[default]
    Image,
    YouTubeVideo,
}

impl TryFrom<String> for GameMediaType {
    type Error = RobloxError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "Image" => Ok(Self::Image),
            "YouTubeVideo" => Ok(Self::YouTubeVideo),
            _ => Err(RobloxError::MalformedResponse),
        }
    }
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct GameDetail {
    #[serde(alias = "id")]
    pub universe_id: u64,
    pub root_place_id: u64,

    /// Translated game's name
    pub name: String,

    /// Translated game's description
    pub description: String,

    /// Original game's name
    pub source_name: String,

    /// Original game's description
    pub source_description: String,

    pub creator: GameCreator,

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

    /// Avatar type. Possible values are MorphToR6, MorphToR15, and PlayerChoice
    pub universe_avatar_type: AvatarType,

    pub genre: String,
    pub is_all_genre: bool,

    pub is_favorited_by_user: bool,
    pub favorited_count: u64,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct PlaceDetails {
    #[serde(alias = "id")]
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

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct GameCreator {
    pub id: u64,
    pub name: String,

    #[serde(alias = "type")]
    pub creator_type: String,

    #[serde(alias = "isRNVAccount")]
    pub is_rnv_account: bool,

    pub has_verified_badge: bool,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct GameMedia {
    pub asset_type_id: u64,
    pub asset_type: GameMediaType,
    pub approved: bool,

    pub image_id: Option<u64>,
    pub alt_text: Option<String>,
    pub video_hash: Option<String>,
    pub video_title: Option<String>,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum ServerType {
    #[default]
    Public,
    Friends,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum SortOrder {
    Ascending,

    #[default]
    Descending,
}

#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, PartialOrd, Default, Serialize, Deserialize)]
pub struct GameServer {
    pub id: String,
    pub max_players: usize,
    pub playing: usize,

    pub player_tokens: Vec<String>,

    // pub players: Vec<String>,
    pub fps: f32,
    pub ping: u64,
}

impl RobloxApi {
    /// Gets games' details using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * If you pass valid roblosecurity, `is_favorited_by_user` would have right value
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const UNIVERSE_ID: u64 = 3717264063;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn games_details(
        &self,
        universe_ids: Vec<u64>,
    ) -> Result<Vec<GameDetail>, RobloxError> {
        let formatted_universe_ids = universe_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let formatted_url = GAMES_DETAILS_API.replace("{universe_ids}", &formatted_universe_ids);

        let cookie_string = self
            .cookie_string()
            .await
            .unwrap_or(HeaderValue::from_static(""));

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<GamesDetailsResponse>(response).await?;

        let mut games_details = Vec::new();

        for game_detail in raw.data {
            games_details.push(GameDetail {
                universe_id: game_detail.id,
                root_place_id: game_detail.root_place_id,
                name: game_detail.name,
                description: game_detail.description,
                source_name: game_detail.source_name,
                source_description: game_detail.source_description,
                creator: GameCreator {
                    id: game_detail.creator.id,
                    name: game_detail.creator.name,
                    creator_type: game_detail.creator.creator_type,
                    is_rnv_account: game_detail.creator.is_rnv_account,
                    has_verified_badge: game_detail.creator.has_verified_badge,
                },
                price: game_detail.price,
                allowed_gear_genres: game_detail.allowed_gear_genres,
                allowed_gear_categories: game_detail.allowed_gear_categories,
                is_genre_enforced: game_detail.is_genre_enforced,
                copying_allowed: game_detail.copying_allowed,
                playing: game_detail.playing,
                visits: game_detail.visits,
                max_players: game_detail.max_players,
                created: game_detail.created,
                updated: game_detail.updated,

                universe_avatar_type: AvatarType::try_from(game_detail.universe_avatar_type)?,
                genre: game_detail.genre,
                is_all_genre: game_detail.is_all_genre,
                is_favorited_by_user: game_detail.is_favorited_by_user,
                favorited_count: game_detail.favorited_count,
            })
        }

        Ok(games_details)
    }

    /// Gets games' details using <https://groups.roblox.com/v1/groups/{group_id}/roles>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * If you pass valid roblosecurity, `is_favorited_by_user` would have right value
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const UNIVERSE_ID: u64 = 3717264063;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn game_details(&self, universe_id: u64) -> Result<GameDetail, RobloxError> {
        let games_details = self.games_details(vec![universe_id]).await?;

        let game_detail = games_details.first().ok_or(RobloxError::BadRequest)?;

        Ok(game_detail.clone())
    }

    /// Gets places' details using <https://games.roblox.com/v1/games/multiget-place-details?placeIds={place_ids}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const PLACES_ID: Vec<u64> = vec![10118559731];
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_details_bulk(
        &self,
        place_ids: Vec<u64>,
    ) -> Result<Vec<PlaceDetails>, RobloxError> {
        let formatted_place_ids = place_ids
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let cookie_string = self.cookie_string().await?;
        let formatted_url = PLACE_DETAILS_API.replace("{place_ids}", &formatted_place_ids);

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw: Vec<PlaceDetailsRaw> = Self::parse_to_raw(response).await?;

        let mut places_details = Vec::new();

        for place_details in raw {
            places_details.push(PlaceDetails {
                place_id: place_details.place_id,

                name: place_details.name,
                description: place_details.description,
                source_name: place_details.source_name,
                source_description: place_details.source_description,

                url: place_details.url,

                is_playable: place_details.is_playable,
                reason_prohibited: place_details.reason_prohibited,
                price: place_details.price,
                image_token: place_details.image_token,

                builder: place_details.builder,
                builder_id: place_details.builder_id,
                has_verified_badge: place_details.has_verified_badge,

                universe_id: place_details.universe_id,
                universe_root_place_id: place_details.universe_root_place_id,
            })
        }

        Ok(places_details)
    }

    /// Gets place's details using <https://games.roblox.com/v1/games/multiget-place-details?placeIds={place_id}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const PLACE_ID: u64 = 10118559731;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_details(&self, universe_id: u64) -> Result<PlaceDetails, RobloxError> {
        let places_details = self.place_details_bulk(vec![universe_id]).await?;

        let place_details = places_details.first().ok_or(RobloxError::BadRequest)?;

        Ok(place_details.clone())
    }

    /// Gets places' details using <https://games.roblox.com/v2/games/{universe_id}/media>.
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const UNIVERSE_ID: u64 = 3717264063;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn game_media(&self, universe_id: u64) -> Result<Vec<GameMedia>, RobloxError> {
        let formatted_url = GAME_MEDIA_API.replace("{universe_id}", &universe_id.to_string());

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<GameMediaResponse>(response).await?;

        let mut game_media_result = Vec::new();

        for game_media in raw.data {
            game_media_result.push(GameMedia {
                asset_type_id: game_media.asset_type_id,
                asset_type: GameMediaType::try_from(game_media.asset_type)?,
                approved: game_media.approved,

                image_id: game_media.image_id,
                alt_text: game_media.alt_text,
                video_hash: game_media.video_hash,
                video_title: game_media.video_title,
            })
        }

        Ok(game_media_result)
    }

    /// Gets places' details using <https://games.roblox.com/v1/games/{place_id}/servers/{servers_type}>.
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    ///
    /// # Notes:
    /// * Returns Vector of `GameServer` and Next page cursor
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    /// const PLACES_ID: Vec<u64> = vec![10118559731];
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// // TODO: example
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn game_servers(
        &self,
        place_id: u64,
        servers_type: Option<ServerType>,
        sort_order: Option<SortOrder>,
        exclude_full_games: Option<bool>,
        cursor: Option<String>,
    ) -> Result<(Vec<GameServer>, Option<String>), RobloxError> {
        let servers_type = if servers_type.unwrap_or_default() == ServerType::Public {
            "0"
        } else {
            "1"
        };

        let sort_order = if sort_order.unwrap_or_default() == SortOrder::Ascending {
            1
        } else {
            2
        };

        let exclude_full_games = if exclude_full_games.unwrap_or(false) {
            "true"
        } else {
            "false"
        };

        let cookie_string = self
            .cookie_string()
            .await
            .unwrap_or(HeaderValue::from_static(""));

        let mut formatted_url = GAME_SERVERS_API
            .replace("{place_id}", &place_id.to_string())
            .replace("{servers_type}", servers_type)
            .replace("{sort_order}", &sort_order.to_string())
            .replace("{exclude_full_games}", exclude_full_games);

        if let Some(cursor) = cursor {
            formatted_url = format!("{}&cursor={}", formatted_url, cursor);
        }

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie_string)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<GameServersResponse>(response).await?;

        let mut game_servers = Vec::new();

        for game_server in raw.data {
            game_servers.push(GameServer {
                id: game_server.id,
                max_players: game_server.max_players,
                playing: game_server.playing,
                player_tokens: game_server.player_tokens,
                fps: game_server.fps,
                ping: game_server.ping,
            })
        }

        Ok((game_servers, raw.next_page_cursor))
    }
}
