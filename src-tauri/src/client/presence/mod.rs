use request_types::{GetPresenceReqBody, GetPresenceResponse};
use reqwest::header::{self, HeaderValue};
use serde::{Deserialize, Serialize};

use super::{RobloxApi, RobloxError};

mod request_types;

const REGISTER_PRESENCE_API: &str = "https://presence.roblox.com/v1/presence/register-app-presence";
const GET_PRESENCE_API: &str = "https://presence.roblox.com/v1/presence/users";

/// Presence of user
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum PresenceType {
    #[default]
    Offline,
    Online,
    InGame,
    InStudio,
    Invisible,
}

impl TryFrom<i32> for PresenceType {
    type Error = RobloxError;

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(Self::Offline),
            1 => Ok(Self::Online),
            2 => Ok(Self::InGame),
            3 => Ok(Self::InStudio),
            4 => Ok(Self::Invisible),
            _ => Err(RobloxError::MalformedResponse),
        }
    }
}

/// Model, representing a User Presence
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UserPresence {
    pub user_id: u64,
    pub presence_type: PresenceType,
    pub last_online: String,
    pub last_location: String,

    pub place_id: Option<u64>,
    pub game_id: Option<String>,
    pub universe_id: Option<u64>,
}

// TODO: add method for fetching users' presence
impl RobloxApi {
    /// Registers presence on the website (makes you appear to be online). Endpoint called is
    /// <https://presence.roblox.com/v1/presence/register-app-presence>
    ///
    /// # Notes
    /// * Requires a valid roblosecurity.
    /// * Will repeat once if the x-csrf-token is invalid.
    /// * Normally repeats every 15 seconds when viewing the Roblox homepage.
    ///
    /// # Return Value Notes
    /// * Will return `Ok(())` if presence was successfully registered.
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    /// * All errors under [Auth Required Errors](#auth-required-errors).
    /// * All errors under [X-CSRF-TOKEN Required Errors](#x-csrf-token-required-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// match client.register_presence().await {
    ///    Ok(_) => println!("Successfully registered presence!"),
    ///    Err(e) => println!("Error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn register_presence(&self) -> Result<(), RobloxError> {
        match self.register_presence_internal().await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.register_presence_internal().await
                }
                _ => Err(e),
            },
        }
    }

    /// Gets presence of users using <https://presence.roblox.com/v1/presence/users>
    ///
    /// # Notes
    /// * Does not require a valid roblosecurity.
    /// * But if provided valid roblosecurity, you can access to peoples' presences, restricted due privacy
    ///
    /// # Errors
    /// * All errors under [Standard Errors](#standard-errors).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use roboat::ClientBuilder;
    ///
    /// const ROBLOSECURITY: &str = "roblosecurity";
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = ClientBuilder::new().roblosecurity(ROBLOSECURITY.to_string()).build();
    ///
    /// match client.register_presence().await {
    ///    Ok(_) => println!("Successfully registered presence!"),
    ///    Err(e) => println!("Error: {}", e),
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_presence(&self, user_ids: Vec<u64>) -> Result<Vec<UserPresence>, RobloxError> {
        let cookie = self
            .cookie_string()
            .await
            .unwrap_or(HeaderValue::from_static(""));
        let body = GetPresenceReqBody { user_ids };

        let request_result = self
            .reqwest_client
            .post(GET_PRESENCE_API)
            .json(&body)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw = Self::parse_to_raw::<GetPresenceResponse>(response).await?;

        let mut presences = Vec::new();

        for user_presence in raw.user_presences {
            presences.push(UserPresence {
                user_id: user_presence.user_id,
                presence_type: PresenceType::try_from(user_presence.user_presence_type)
                    .map_err(|_| RobloxError::MalformedResponse)?,
                last_online: user_presence.last_online,
                last_location: user_presence.last_location,
                place_id: user_presence.place_id,
                game_id: user_presence.game_id,
                universe_id: user_presence.universe_id,
            });
        }

        Ok(presences)
    }
}

mod internal {
    use reqwest::header;

    use super::REGISTER_PRESENCE_API;
    use crate::client::{RobloxApi, RobloxError, XCSRF_HEADER};

    impl RobloxApi {
        pub(super) async fn register_presence_internal(&self) -> Result<(), RobloxError> {
            let cookie = self.cookie_string().await?;

            let json = serde_json::json!({
                "location": "Home",
            });

            let request_result = self
                .reqwest_client
                .post(REGISTER_PRESENCE_API)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, self.xcsrf().await)
                .json(&json)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // We don't care about the response, just that it's a status code 200.
            Ok(())
        }
    }
}
