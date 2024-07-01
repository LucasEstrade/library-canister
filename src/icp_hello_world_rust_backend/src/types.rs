use serde::{Deserialize, Serialize};
use candid::CandidType;

#[derive(Clone, Debug, CandidType, Deserialize, Serialize)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub is_borrowed: bool,
}
