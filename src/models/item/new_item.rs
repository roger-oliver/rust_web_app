use chrono::{NaiveDateTime, Utc};
use diesel::Insertable;

use crate::to_do::enums::TaskStatus;
use crate::schema::to_do; // used as referece in the table_name attribute

#[derive(Insertable)]
#[table_name="to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> Self {
        let now = Utc::now().naive_local();
        return NewItem {
            title,
            status: TaskStatus::PENDING.stringfy(),
            date: now,
            user_id
        }
    }
}