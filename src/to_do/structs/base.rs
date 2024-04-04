use serde::Serialize;

use crate::to_do::enums::TaskStatus;

#[derive(Serialize)]
pub struct Base {
    pub title: String,
    pub status: TaskStatus
}