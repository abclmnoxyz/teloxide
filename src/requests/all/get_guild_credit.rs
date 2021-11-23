use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{GuildCredit},
    Bot,
};
use std::sync::Arc;

/// Use this method to change the title of a chat.
///
/// Titles can't be changed for private chats. The bot must be an administrator
/// in the chat for this to work and must have the appropriate admin rights.
///
/// [The official docs](https://core.telegram.org/bots/api#setchattitle).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGuildCredit {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub guild_id: Option<i64>,
    pub user_id: i64,
    pub bot_id: Option<i64>,
}

#[async_trait::async_trait]
impl Request for GetGuildCredit {
    type Output = Vec<GuildCredit>;

    async fn send(&self) -> ResponseResult<Vec<GuildCredit>> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "getCredit",
            &self,
        ).await
    }
}

impl GetGuildCredit {
    pub(crate) fn new(bot: Arc<Bot>, bot_id: Option<i64>, guild_id: Option<i64>, user_id: i64) -> Self {
        Self { bot, bot_id, guild_id, user_id }
    }

}
