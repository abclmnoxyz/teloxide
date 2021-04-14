use serde::{Serialize, Deserialize};

use crate::{
    net,
    requests::{Request, ResponseResult},
    types::{InlineQueryResult, True},
    Bot,
};
use std::sync::Arc;

/// Use this method to send answers to an inline query.
///
/// No more than **50** results per query are allowed.
///
/// [The official docs](https://core.telegram.org/bots/api#answerinlinequery).
#[serde_with_macros::skip_serializing_none]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AnswerInlineQuery {
    #[serde(skip_serializing, skip_deserializing)]
    pub bot: Arc<Bot>,
    pub inline_query_id: String,
    pub results: Vec<InlineQueryResult>,
    pub cache_time: Option<i32>,
    pub is_personal: Option<bool>,
    pub next_offset: Option<String>,
    pub switch_pm_text: Option<String>,
    pub switch_pm_parameter: Option<String>,
}

#[async_trait::async_trait]
impl Request for AnswerInlineQuery {
    type Output = True;

    async fn send(&self) -> ResponseResult<True> {
        net::request_json(
            self.bot.client(),
            self.bot.token(),
            "answerInlineQuery",
            &self,
        )
        .await
    }
}

impl AnswerInlineQuery {
    pub(crate) fn new<I, R>(
        bot: Arc<Bot>,
        inline_query_id: I,
        results: R,
    ) -> Self
    where
        I: Into<String>,
        R: Into<Vec<InlineQueryResult>>,
    {
        let inline_query_id = inline_query_id.into();
        let results = results.into();
        Self {
            bot,
            inline_query_id,
            results,
            cache_time: None,
            is_personal: None,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    /// Unique identifier for the answered query.
    pub fn inline_query_id<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.inline_query_id = val.into();
        self
    }

    /// A JSON-serialized array of results for the inline query.
    pub fn results<T>(mut self, val: T) -> Self
    where
        T: Into<Vec<InlineQueryResult>>,
    {
        self.results = val.into();
        self
    }

    /// The maximum amount of time in seconds that the result of the inline
    /// query may be cached on the server.
    ///
    /// Defaults to 300.
    pub fn cache_time(mut self, val: i32) -> Self {
        self.cache_time = Some(val);
        self
    }

    /// Pass `true`, if results may be cached on the server side only for the
    /// user that sent the query.
    ///
    /// By default, results may be returned to any user who sends the same
    /// query.
    #[allow(clippy::wrong_self_convention)]
    pub fn is_personal(mut self, val: bool) -> Self {
        self.is_personal = Some(val);
        self
    }

    /// Pass the offset that a client should send in the next query with the
    /// same text to receive more results.
    ///
    /// Pass an empty string if there are no more results or if you don‘t
    /// support pagination. Offset length can’t exceed 64 bytes.
    pub fn next_offset<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.next_offset = Some(val.into());
        self
    }

    /// If passed, clients will display a button with specified text that
    /// switches the user to a private chat with the bot and sends the bot a
    /// start message with the parameter [`switch_pm_parameter`].
    ///
    /// [`switch_pm_parameter`]:
    /// crate::requests::AnswerInlineQuery::switch_pm_parameter
    pub fn switch_pm_text<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.switch_pm_text = Some(val.into());
        self
    }

    /// [Deep-linking] parameter for the /start message sent to the bot when
    /// user presses the switch button. 1-64 characters, only `A-Z`, `a-z`,
    /// `0-9`, `_` and `-` are allowed.
    ///
    /// Example: An inline bot that sends YouTube videos can ask the user to
    /// connect the bot to their YouTube account to adapt search results
    /// accordingly. To do this, it displays a ‘Connect your YouTube account’
    /// button above the results, or even before showing any. The user presses
    /// the button, switches to a private chat with the bot and, in doing so,
    /// passes a start parameter that instructs the bot to return an oauth link.
    /// Once done, the bot can offer a [`switch_inline`] button so that the user
    /// can easily return to the chat where they wanted to use the bot's
    /// inline capabilities.
    ///
    /// [Deep-linking]: https://core.telegram.org/bots#deep-linking
    /// [`switch_inline`]: crate::types::InlineKeyboardMarkup
    pub fn switch_pm_parameter<T>(mut self, val: T) -> Self
    where
        T: Into<String>,
    {
        self.switch_pm_parameter = Some(val.into());
        self
    }
}
