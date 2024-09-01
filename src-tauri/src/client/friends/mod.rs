use reqwest::header;
use serde::{Deserialize, Serialize};

use super::{presence::PresenceType, RobloxApi, RobloxError};

mod request_types;

const FRIENDS_LIST_API: &str = "https://friends.roblox.com/v1/users/{user_id}/friends";
const FRIEND_REQUESTS_API: &str = "https://friends.roblox.com/v1/my/friends/requests";
const PENDING_FRIEND_REQUESTS_API: &str =
    "https://friends.roblox.com/v1/user/friend-requests/count";

const ACCEPT_FRIEND_REQUEST_API: &str =
    "https://friends.roblox.com/v1/users/{requester_id}/accept-friend-request";
const DECLINE_FRIEND_REQUEST_API: &str =
    "https://friends.roblox.com/v1/users/{requester_id}/decline-friend-request";

const SEND_FRIEND_REQUEST_API: &str =
    "https://friends.roblox.com/v1/users/{target_id}/request-friendship";
const UNFRIEND_API: &str = "https://friends.roblox.com/v1/users/{target_id}/unfriend";

/// Model, representing user information that also contains select presence information
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendUserInformation {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String,

    pub description: Option<String>,

    pub created: String,

    pub presence_type: PresenceType,

    /// Whether the user is banned/terminated.
    #[serde(alias = "isBanned")]
    pub is_terminated: bool,

    /// The user's verified badge status.
    pub has_verified_badge: bool,
}

/// Model, representing a friend request.
#[allow(missing_docs)]
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub struct FriendRequest {
    #[serde(alias = "id")]
    pub user_id: u64,

    #[serde(alias = "name")]
    pub username: String,

    #[serde(alias = "displayName")]
    pub display_name: String,

    pub description: Option<String>,

    pub created: String,

    #[serde(alias = "isBanned")]
    pub is_terminated: bool,

    #[serde(alias = "hasVerifiedBadge")]
    pub has_verified_badge: bool,

    #[serde(alias = "mutualFriendsList")]
    pub mutual_friends_list: Vec<String>,

    /// The origin source type associated with the friend request.
    pub origin_source_type: String,

    /// The source universe id which the request was sent in.
    /// # Note
    ///  * Default universe id: `0`
    pub source_universe_id: u64,

    /// When the friend request was sent.
    pub sent_at: String,
}

impl RobloxApi {
    /// Get list of all friends for the specified user using <https://friends.roblox.com/v1/users/{userId}/friends>.
    pub async fn friends_list(
        &self,
        user_id: u64,
    ) -> Result<Vec<FriendUserInformation>, RobloxError> {
        let formatted_url = FRIENDS_LIST_API.replace("{user_id}", &user_id.to_string());

        let request_result = self.reqwest_client.get(formatted_url).send().await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendsListResponse>(response).await?;

        let mut friends = Vec::new();

        for friend_raw in raw.data {
            let friend = FriendUserInformation {
                user_id: friend_raw.id,
                username: friend_raw.username,
                display_name: friend_raw.display_name,

                description: friend_raw.description,
                created: friend_raw.created,

                presence_type: PresenceType::try_from(friend_raw.presence_type.unwrap_or(0))
                    .unwrap_or(PresenceType::Offline),
                is_terminated: friend_raw.is_banned,

                has_verified_badge: friend_raw.has_verified_badge,
            };

            friends.push(friend);
        }

        Ok(friends)
    }

    /// Get list of friend requests with cursor using <https://friends.roblox.com/v1/my/friends/requests>.
    pub async fn friend_requests(
        &self,
        cursor: Option<String>,
    ) -> Result<(Vec<FriendRequest>, Option<String>), RobloxError> {
        let cookie = self.cookie_string().await?;
        let mut formatted_url = format!("{}?limit={}", FRIEND_REQUESTS_API, 10);

        if let Some(cursor) = cursor {
            formatted_url = format!("{}&cursor={}", formatted_url, cursor)
        }

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw = Self::parse_to_raw::<request_types::FriendRequestsResponse>(response).await?;

        let mut friend_requests = Vec::new();

        for friend_request_raw in raw.data {
            let friend_request = FriendRequest {
                user_id: friend_request_raw.user_id,
                username: friend_request_raw.username,
                display_name: friend_request_raw.display_name,
                description: friend_request_raw.description,
                created: friend_request_raw.created,
                is_terminated: friend_request_raw.is_terminated,
                has_verified_badge: friend_request_raw.has_verified_badge,
                mutual_friends_list: friend_request_raw.mutual_friends_list,
                origin_source_type: friend_request_raw.friend_request.origin_source_type,
                source_universe_id: friend_request_raw.friend_request.source_universe_id,
                sent_at: friend_request_raw.friend_request.sent_at,
            };

            friend_requests.push(friend_request);
        }

        Ok((friend_requests, raw.next_page_cursor))
    }

    /// Get count of pending friend requests using <https://friends.roblox.com/v1/user/friend-requests/count>.
    pub async fn pending_friend_requests(&self) -> Result<u64, RobloxError> {
        let cookie = self.cookie_string().await?;
        let formatted_url = PENDING_FRIEND_REQUESTS_API;

        let request_result = self
            .reqwest_client
            .get(formatted_url)
            .header(header::COOKIE, cookie)
            .send()
            .await;

        let response = Self::validate_request_result(request_result).await?;

        let raw =
            Self::parse_to_raw::<request_types::PendingFriendRequestsResponse>(response).await?;

        Ok(raw.count)
    }

    /// Accepts friend request using <https://friends.roblox.com/v1/users/{requester_id}/accept-friend-request>.
    pub async fn accept_friend_request(&self, requester_id: u64) -> Result<(), RobloxError> {
        match self.accept_friend_request_internal(requester_id).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.accept_friend_request_internal(requester_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// Declines friend request using <https://friends.roblox.com/v1/users/{requester_id}/decline-friend-request>.
    pub async fn decline_friend_request(&self, requester_id: u64) -> Result<(), RobloxError> {
        match self.decline_friend_request_internal(requester_id).await {
            Ok(x) => Ok(x),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.decline_friend_request_internal(requester_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// Sends friend request using <https://friends.roblox.com/v1/users/{target_id}/request-friendship>.
    pub async fn send_friend_request(&self, target_id: u64) -> Result<(), RobloxError> {
        match self.send_friend_request_internal(target_id).await {
            Ok(_) => Ok(()),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.send_friend_request_internal(target_id).await
                }
                _ => Err(e),
            },
        }
    }

    /// Unfriends using <https://friends.roblox.com/v1/users/{target_id}/unfriend>.
    pub async fn unfriend(&self, target_id: u64) -> Result<(), RobloxError> {
        match self.unfriend_internal(target_id).await {
            Ok(_) => Ok(()),
            Err(e) => match e {
                RobloxError::InvalidXcsrf(new_xcsrf) => {
                    self.set_xcsrf(new_xcsrf).await;

                    self.unfriend_internal(target_id).await
                }
                _ => Err(e),
            },
        }
    }
}

mod internal {
    use reqwest::header;
    use serde_json::json;

    use crate::client::{RobloxApi, RobloxError, XCSRF_HEADER};

    impl RobloxApi {
        pub(super) async fn accept_friend_request_internal(
            &self,
            requester_id: u64,
        ) -> Result<(), RobloxError> {
            let formatted_url = super::ACCEPT_FRIEND_REQUEST_API
                .replace("{requester_id}", &requester_id.to_string());

            let cookie = self.cookie_string().await?;
            let xcsrf = self.xcsrf().await;

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // If we got a status code 200, it was successful.

            Ok(())
        }

        pub(super) async fn decline_friend_request_internal(
            &self,
            requester_id: u64,
        ) -> Result<(), RobloxError> {
            let formatted_url = super::DECLINE_FRIEND_REQUEST_API
                .replace("{requester_id}", &requester_id.to_string());

            let cookie = self.cookie_string().await?;
            let xcsrf = self.xcsrf().await;

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // If we got a status code 200, it was successful.

            Ok(())
        }

        pub(super) async fn send_friend_request_internal(
            &self,
            target_id: u64,
        ) -> Result<(), RobloxError> {
            let formatted_url =
                super::SEND_FRIEND_REQUEST_API.replace("{target_id}", &target_id.to_string());

            let cookie = self.cookie_string().await?;
            let xcsrf = self.xcsrf().await;

            // TODO: maybe add settable friendshipOriginSourceType parameter
            let body = json!({
                "friendshipOriginSourceType": 0
            });

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, xcsrf)
                .json(&body)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // If we got a status code 200, it was successful.

            Ok(())
        }

        pub(super) async fn unfriend_internal(&self, target_id: u64) -> Result<(), RobloxError> {
            let formatted_url = super::UNFRIEND_API.replace("{target_id}", &target_id.to_string());

            let cookie = self.cookie_string().await?;
            let xcsrf = self.xcsrf().await;

            let request_result = self
                .reqwest_client
                .post(formatted_url)
                .header(header::COOKIE, cookie)
                .header(XCSRF_HEADER, xcsrf)
                .send()
                .await;

            let _ = Self::validate_request_result(request_result).await?;

            // If we got a status code 200, it was successful.

            Ok(())
        }
    }
}
