use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, ChatPermissions, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to restrict a user in a supergroup.
///
/// The bot must be an administrator in the supergroup for this to work and must
/// have the appropriate admin rights. Pass `true` for all permissions to lift
/// restrictions from a user.
///
/// [The official docs](https://core.telegram.org/bots/api#restrictchatmember).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct RestrictChatMember {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    user_id: i64,
    permissions: ChatPermissions,
    until_date: Option<i32>,
}

#[async_trait::async_trait]
impl Request for RestrictChatMember {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "restrictChatMember",
            &self,
        )
        .await
    }
}

impl RestrictChatMember {
    pub(crate) fn new<C>(
        bot: Arc<Bot>,
        chat_id: C,
        user_id: i64,
        permissions: ChatPermissions,
    ) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, user_id, permissions, until_date: None }
    }

    /// Unique identifier for the target chat or username of the target
    /// supergroup (in the format `@supergroupusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }

    /// Unique identifier of the target user.
    pub fn user_id(mut self, val: i64) -> Self {
        self.user_id = val;
        self
    }

    /// New user permissions.
    pub fn permissions(mut self, val: ChatPermissions) -> Self {
        self.permissions = val;
        self
    }

    /// Date when restrictions will be lifted for the user, unix time.
    ///
    /// If user is restricted for more than 366 days or less than 30 seconds
    /// from the current time, they are considered to be restricted forever.
    pub fn until_date(mut self, val: i32) -> Self {
        self.until_date = Some(val);
        self
    }
}
