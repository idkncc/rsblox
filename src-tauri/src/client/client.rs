use reqwest::{header::HeaderValue, Client};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

use super::RobloxError;

/// Basic information about the account of the Roblosecurity. Retrieved
/// from <https://users.roblox.com/v1/users/authenticated>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub(crate) struct ClientUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Default)]
pub struct RobloxApi {
    /// The full cookie that includes the roblosecurity token.
    pub(crate) cookie_string: RwLock<Option<HeaderValue>>,

    /// The field holding the value for the X-CSRF-TOKEN header used in and returned by endpoints.
    pub(crate) xcsrf: RwLock<String>,

    /// Holds the user id, username, and display name of the user.
    pub(crate) user_information: RwLock<Option<ClientUserInformation>>,

    /// A Reqwest HTTP client used to send web requests.
    pub(crate) reqwest_client: reqwest::Client,
}

impl RobloxApi {
    pub fn new() -> Self {
        let client = Client::builder()
            .build()
            .expect("Failed to setup reqwest client");

        Self {
            reqwest_client: client,
            ..Default::default()
        }
    }

    /// Get basic logged in user information
    pub(crate) async fn user_information(&self) -> Option<ClientUserInformation> {
        self.user_information.read().await.clone()
    }

    /// Used in [`Client::user_information_internal`]. This is implemented in the client
    /// module as we do not want other modules to have to interact with the rwlock directly.
    pub(crate) async fn set_user_information(&self, user_information: ClientUserInformation) {
        *self.user_information.write().await = Some(user_information);
    }

    /// Sets the xcsrf token of the client. Remember to .await this method.
    pub(crate) async fn set_xcsrf(&self, xcsrf: String) {
        *self.xcsrf.write().await = xcsrf;
    }

    /// Returns a copy of the xcsrf stored in the client. Remember to .await this method.
    pub(crate) async fn xcsrf(&self) -> String {
        self.xcsrf.read().await.clone()
    }

    /// Sets cookie
    /// If cookie value is invalid, None will be set
    pub async fn set_cookie(&self, roblosecurity: String) {
        let cookie = HeaderValue::from_str(&format!(".ROBLOSECURITY={}", roblosecurity))
            .ok()
            .map(|mut header| {
                header.set_sensitive(true);

                header
            });

        *self.cookie_string.write().await = cookie.clone();

        if cookie.is_some() {
            let result = self.user_information_internal().await;

            if let Err(_) = result {
                *self.cookie_string.write().await = None;
                return;
            }
        }
    }

    /// Returns a copy of the cookie string stored in the client.
    /// If the roblosecurity has not been set, [`RoboatError::RoblosecurityNotSet`] is returned.
    pub(crate) async fn cookie_string(&self) -> Result<HeaderValue, RobloxError> {
        let cookie_string_opt = self.cookie_string.read().await.clone();

        match cookie_string_opt {
            Some(cookie) => Ok(cookie.clone()),
            None => Err(RobloxError::RoblosecurityNotSet),
        }
    }
}

/// User info
impl RobloxApi {
    pub async fn user_id(&self) -> Result<u64, RobloxError> {
        let Some(user_info) = self.user_information().await else {
            return Err(RobloxError::InvalidRoblosecurity);
        };

        Ok(user_info.user_id)
    }

    pub async fn username(&self) -> Result<String, RobloxError> {
        let Some(user_info) = self.user_information().await else {
            return Err(RobloxError::InvalidRoblosecurity);
        };

        Ok(user_info.username)
    }

    pub async fn display_name(&self) -> Result<String, RobloxError> {
        let Some(user_info) = self.user_information().await else {
            return Err(RobloxError::InvalidRoblosecurity);
        };

        Ok(user_info.display_name)
    }
}
