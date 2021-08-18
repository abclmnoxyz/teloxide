use serde::{Deserialize, Serialize};


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CircleComment {
    post_id: i64,
    comment_id: i64,
    content: Option<String>,
}

impl CircleComment {
    pub fn new(post_id: i64, comment_id: i64, content: Option<String>) -> Self {
        CircleComment {
            post_id,
            comment_id,
            content,
        }
    }
}

