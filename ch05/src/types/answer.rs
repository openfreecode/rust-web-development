use serde::{Deserialize, Serialize};

use super::QuestionId;

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Hash, Clone)]
pub struct AnswerId(pub String);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}
