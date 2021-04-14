use serde::Serialize;

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to unpin a message in a group, a supergroup, or a channel.
///
/// The bot must be an administrator in the chat for this to work and must have
/// the `can_pin_messages` admin right in the supergroup or `can_edit_messages`
/// admin right in the channel.
///
/// [The official docs](https://core.telegram.org/bots/api#unpinchatmessage).
#[serde_with_macros::skip_serializing_none]
#[derive(Debug, Clone, Serialize)]
pub struct UnpinChatMessage {
    #[serde(skip_serializing)]
    bot: Arc<Bot>,
    chat_id: ChatId,
    message_id: i64,
}

#[async_trait::async_trait]
impl Request for UnpinChatMessage {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "unpinChatMessage",
            &self,
        )
        .await
    }
}

impl UnpinChatMessage {
    pub(crate) fn new<C>(bot: Arc<Bot>, chat_id: C, message_id: i64) -> Self
    where
        C: Into<ChatId>,
    {
        let chat_id = chat_id.into();
        Self { bot, chat_id, message_id }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, val: T) -> Self
    where
        T: Into<ChatId>,
    {
        self.chat_id = val.into();
        self
    }
    pub fn message_id(mut self, val: i64) -> Self {
        self.message_id = val;
        self
    }
}
