use serde::{Deserialize, Serialize};
use chrono::naive::NaiveDateTime;
use chrono::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct PageView {
    pub page_name: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PageEvent {
    pub event_name: String,
    pub user_id: i32,
    pub created_at: NaiveDateTime
}

pub fn now() -> chrono::naive::NaiveDateTime {
    Utc::now().naive_local()
}