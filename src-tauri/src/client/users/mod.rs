use super::{client::ClientUserInformation, RobloxApi, RobloxError, User};
use reqwest::header::{self, HeaderValue};
use serde::{Deserialize, Serialize};

mod request_types;

const AUTHENTICATED_USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/authenticated";
const USERS_SEARCH_API: &str = "https://users.roblox.com/v1/users/search";
const USER_DETAILS_API: &str = "https://users.roblox.com/v1/users/{user_id}";
const USER_FROM_USERNAME_API: &str = "https://users.roblox.com/v1/usernames/users";

// TODO: try to make a unified user details struct

/// The details of a user. Fetched from <https://users.roblox.com/v1/users/{user_id}>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UserDetails {
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub id: u64,
    pub description: String,
    /// A time string of when the account was created. Follows ISO 8061 format.
    #[serde(alias = "created")]
    pub created_at: String,
    /// Whether the account is terminated. Does not include non-termination bans.
    #[serde(alias = "isBanned")]
    pub is_terminated: bool,
    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
}

/// The details of a user. Fetched from <https://users.roblox.com/v1/usernames/users>.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct UsernameUserDetails {
    pub requested_username: String,
    #[serde(alias = "name")]
    pub username: String,
    #[serde(alias = "displayName")]
    pub display_name: String,
    pub id: u64,
    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,
}

impl RobloxApi {
    /// Grabs information about the user from <https://catalog.roblox.com/v1/catalog/items/details> using the
    /// Roblosecurity inside the client.
    pub(super) async fn user_information_internal(
        &self,
    ) -> Result<ClientUserInformation, RobloxError> {
        let cookie = self.cookie_string().await?;

        let request_result = self
            .reqwest_client
            .get(AUTHENTICATED_USER_DETAILS_API)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let user_information = Self::parse_to_raw::<ClientUserInformation>(response).await?;

        // Cache results.
        self.set_user_information(user_information.clone()).await;

        Ok(user_information)
    }

    /// Searches for a user using <https://users.roblox.com/v1/users/search>.
    pub async fn user_search(&self, keyword: String) -> Result<Vec<User>, RobloxError> {
        let formatted_url = format!("{}?keyword={}", USERS_SEARCH_API, keyword);

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
        let raw = Self::parse_to_raw::<request_types::UserSearchResponse>(response).await?;

        let mut users = Vec::new();

        for user in raw.data {
            let user_data = User {
                user_id: user.id,
                username: user.name,
                display_name: user.display_name,
            };

            users.push(user_data);
        }

        Ok(users)
    }

    /// Fetches user details using <https://users.roblox.com/v1/users/{user_id}>.
    pub async fn user_details(&self, user_id: u64) -> Result<UserDetails, RobloxError> {
        let formatted_url = USER_DETAILS_API.replace("{user_id}", &user_id.to_string());

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;
        let user_details = Self::parse_to_raw::<UserDetails>(response).await?;

        Ok(user_details)
    }

    /// Fetches user details using <https://users.roblox.com/v1/usernames/users>
    pub async fn username_user_details(
        &self,
        usernames: Vec<String>,
        exclude_banned_users: bool,
    ) -> Result<Vec<UsernameUserDetails>, RobloxError> {
        let request_result = self
            .reqwest_client
            .post(USER_FROM_USERNAME_API)
            .json(&request_types::UsernameUserDetailsRequest {
                usernames,
                exclude_banned_users,
            })
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;
        let raw =
            Self::parse_to_raw::<request_types::UsernameUserDetailsResponse>(response).await?;

        let users = raw
            .data
            .into_iter()
            .map(|user| UsernameUserDetails {
                requested_username: user.requested_username,
                username: user.name,
                display_name: user.display_name,
                id: user.id,
                has_verified_badge: user.has_verified_badge,
            })
            .collect();
        Ok(users)
    }
}
