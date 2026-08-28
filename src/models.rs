use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flashcard {
    pub id: i64,
    pub front: String,
    pub back: String,
    pub ease_factor: f64,
    pub interval: i64,
    pub repetitions: i64,
    pub next_review: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CardForm {
    pub front: String,
    pub back: String,
}

#[derive(Debug, Deserialize)]
pub struct AnswerForm {
    pub card_id: i64,
    pub rating: String,
}
