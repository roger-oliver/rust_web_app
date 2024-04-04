use crate::{models::user::user::User, schema::to_do};
use chrono::NaiveDateTime;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name="to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}