use crate::types::ChatId;

use serde::{Deserialize, Serialize};

/// A chat message or inline message.
#[derive(Clone, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatOrInlineMessage {
    Chat { chat_id: ChatId, message_id: i64 },
    Inline { inline_message_id: i64 },
}
