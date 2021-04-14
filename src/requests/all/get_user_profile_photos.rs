use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::UserProfilePhotos,
    Bot,
};
use std::sync::Arc;

/// Use this method to get a list of profile pictures for a user.
///
/// [The official docs](https://core.telegram.org/bots/api#getuserprofilephotos).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct GetUserProfilePhotos {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    user_id: i64,
    offset: Option<i64>,
    limit: Option<i32>,
}

#[async_trait::async_trait]
impl Request for GetUserProfilePhotos {
    type Output = UserProfilePhotos;

    async fn send(&self) -> ResponseResult<UserProfilePhotos> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getUserProfilePhotos",
            &self,
        )
        .await
    }
}

impl GetUserProfilePhotos {
    pub(crate) fn new(bot: Arc<Bot>, user_id: i64) -> Self {
        Self { bot, user_id, offset: None, limit: None }
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    /// Sequential number of the first photo to be returned. By default, all
    /// photos are returned.
    pub fn offset(mut self, val: i64) -> Self {
        self.offset = Some(val);
        self
    }

    /// Limits the number of photos to be retrieved. Values between 1—100 are
    /// accepted.
    ///
    /// Defaults to 100.
    pub fn limit(mut self, val: i32) -> Self {
        self.limit = Some(val);
        self
    }
}
