use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{GuildRole, ChatId, ChatMember},
    Bot,
};
use std::sync::Arc;

/// Use this method to get up to date information about the chat (current name
/// of the user for one-on-one conversations, current username of a user, group
/// or channel, etc.).
///
/// [The official docs](https://core.telegram.org/bots/api#getchat).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleMembersCount {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub guild_id: i64,
    pub role_id: i64,
}

#[async_trait::async_trait]
impl Request for GetRoleMembersCount {
    type Output = i32;

    async fn send(&self) -> ResponseResult<i32> {
        net::request_json(self.bot.client(), self.bot.token(), "getRoleMembersCount", &self)
            .await
    }
}

impl GetRoleMembersCount {
    pub(crate) fn new(bot: Arc<Bot>, guild_id: i64, role_id: i64) -> Self {
        Self { bot, guild_id , role_id}
    }
}
