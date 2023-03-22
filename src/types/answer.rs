use serde::{Deserialize, Serialize};

use crate::types::question::QuestionId,

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Answer {
    pub id: AnswerId,
    pub content: String,
    pub question_id: QuestionId,
}

#[derive(Deserialize, Serialize, Debug, CLone, PartialEq, Eq, Hash)]
pub struct AnswerId(pub String);
