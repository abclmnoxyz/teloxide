use crate::{
    net,
    requests::{form_builder::FormBuilder, Request, ResponseResult},
    types::{ChatId, InputMedia, Message},
    Bot,
};
use std::sync::Arc;

/// Use this method to send a group of photos or videos as an album.
///
/// [The official docs](https://core.telegram.org/bots/api#sendmediagroup).
#[derive(Debug, Clone)]
pub struct SendMediaGroup {
    bot: Arc<Bot>,
    chat_id: ChatId,
    media: Vec<InputMedia>, // TODO: InputMediaPhoto and InputMediaVideo
    disable_notification: Option<bool>,
    reply_to_message_id: Option<i64>,
}

#[async_trait::async_trait]
impl Request for SendMediaGroup {
    type Output = Vec<Message>;

    async fn send(&self) -> ResponseResult<Vec<Message>> {
        net::request_multipart(
            self.bot.client(),
            self.bot.token(),
            "sendMediaGroup",
            FormBuilder::new()
                .add("chat_id", &self.chat_id)
                .await
                .add("media", &self.media)
                .await
                .add("disable_notification", &self.disable_notification)
                .await
                .add("reply_to_message_id", &self.reply_to_message_id)
                .await
                .build(),
        )
        .await
    }
}

impl SendMediaGroup {
    pub(crate) fn new<C, M>(bot: Arc<Bot>, chat_id: C, media: M) -> Self
    where
        C: Into<ChatId>,
        M: Into<Vec<InputMedia>>,
    {
        let chat_id = chat_id.into();
        let media = media.into();
        Self {
            bot,
            chat_id,
            media,
            disable_notification: None,
            reply_to_message_id: None,
        }
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

    /// A JSON-serialized array describing photos and videos to be sent, must
    /// include 2–10 items.
    pub fn media<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<InputMedia>>,
    {
        self.media = val.into();
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, val: bool) -> Self {
        self.disable_notification = Some(val);
        self
    }

    /// If the messages are a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, val: i64) -> Self {
        self.reply_to_message_id = Some(val);
        self
    }
}
