use serde::{Serialize, Deserialize};

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{ChatId, Message, ParseMode, ReplyMarkup},
    Bot,
};
use std::sync::Arc;

/// Use this method to send task messages.
///
/// [The official docs](https://core.telegram.org/bots/api#sendmessage).
#[serde_with_macros::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SendTaskInduction {
    #[serde(skip)]
    bot: Arc<Bot>,
    pub chat_id: ChatId,
    pub task: String,
    pub parse_mode: Option<ParseMode>,
    pub selective: Option<bool>,
    pub disable_web_page_preview: Option<bool>,
    pub disable_notification: Option<bool>,
    pub reply_to_message_id: Option<i64>,
    pub reply_markup: Option<ReplyMarkup>,
}