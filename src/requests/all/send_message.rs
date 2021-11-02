use serde::{Serialize, Deserialize};

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ParseMode, ReplyMarkup},
    Bot,
};
use std::sync::Arc;

/// Use this method to send text messages.
///
/// [The official docs](https://core.telegram.org/bots/api#sendmessage).
#[serde_with_macros::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SendMessage {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub chat_id: ChatId,
    pub text: String,
    pub desc: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub selective: Option<bool>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_to_message_id_level_2: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
    pub unreactive: Option<i32>,
    pub ephemeral: Option<bool>,
    pub users: Option<Vec<String>>,
}

#[async_trait::async_trait]
impl Request for SendMessage {
    type Output = Message;

    async fn send(&self) -> ResponseResult<Message> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "sendMessage",
            &self,
        )
            .await
    }
}

impl SendMessage {
    pub(crate) fn new<C, T>(bot: Arc<Bot>, chat_id: C, text: T) -> Self
        where
            C: Into<ChatId>,
            T: Into<String>,
    {
        Self {
            bot,
            chat_id: chat_id.into(),
            text: text.into(),
            desc: None,
            parse_mode: None,
            selective: None,
            disable_web_page_preview: None,
            disable_notification: None,
            reply_to_message_id: None,
            reply_to_message_id_level_2: None,
            reply_markup: None,
            unreactive: None,
            users: None,
            ephemeral: None,
        }
    }

    /// Unique identifier for the target chat or username of the target channel
    /// (in the format `@channelusername`).
    pub fn chat_id<T>(mut self, value: T) -> Self
        where
            T: Into<ChatId>,
    {
        self.chat_id = value.into();
        self
    }

    /// Text of the message to be sent.
    pub fn text<T>(mut self, value: T) -> Self
        where
            T: Into<String>,
    {
        self.text = value.into();
        self
    }

    /// desc of the text will show in chat list
    pub fn desc<T>(mut self, value: T) -> Self
        where
            T: Into<String>,
    {
        self.desc = Some(value.into());
        self
    }

    /// Send [Markdown] or [HTML], if you want Telegram apps to show
    /// [bold, italic, fixed-width text or inline URLs] in the media caption.
    ///
    /// [Markdown]: crate::types::ParseMode::Markdown
    /// [HTML]: crate::types::ParseMode::HTML
    /// [bold, italic, fixed-width text or inline URLs]:
    /// crate::types::ParseMode
    pub fn parse_mode(mut self, value: ParseMode) -> Self {
        self.parse_mode = Some(value);
        self
    }

    /// Disables link previews for links in this message.
    pub fn disable_web_page_preview(mut self, value: bool) -> Self {
        self.disable_web_page_preview = Some(value);
        self
    }

    /// Sends the message [silently]. Users will receive a notification with no
    /// sound.
    ///
    /// [silently]: https://telegram.org/blog/channels-2-0#silent-messages
    pub fn disable_notification(mut self, value: bool) -> Self {
        self.disable_notification = Some(value);
        self
    }

    /// If the message is a reply, ID of the original message.
    pub fn reply_to_message_id(mut self, value: i64) -> Self {
        self.reply_to_message_id = Some(value);
        self
    }

    /// Additional interface options.
    ///
    /// A JSON-serialized object for an [inline keyboard], [custom reply
    /// keyboard], instructions to remove reply keyboard or to force a reply
    /// from the user.
    ///
    /// [inline keyboard]: https://core.telegram.org/bots#inline-keyboards-and-on-the-fly-updating
    /// [custom reply keyboard]: https://core.telegram.org/bots#keyboards
    pub fn reply_markup<T>(mut self, value: T) -> Self
        where
            T: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(value.into());
        self
    }
    pub fn selective<T>(mut self, val: T) -> Self
        where
            T: Into<Option<bool>>,
    {
        self.selective = val.into();
        self
    }

    pub fn unreactive(mut self, val: Option<i32>) -> Self {
        self.unreactive = val;
        self
    }

    pub fn ephemeral(mut self, val: Option<bool>) -> Self {
        self.ephemeral = val;
        self
    }

    pub fn users(mut self, users: Option<Vec<String>>) -> Self {
        self.users = users;
        self
    }
}
