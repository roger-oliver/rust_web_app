use actix_web::{body::BoxBody, http::header::ContentType, HttpResponse, Responder};
use diesel::{query_dsl::methods::OrderDsl, RunQueryDsl, ExpressionMethods};
use serde::Serialize;

use crate::{
    database::establish_connection,
    models::item::item::Item,
    schema::to_do,
    to_do::{enums::TaskStatus, structs::base::Base, to_do_factory, ItemTypes},
};

#[derive(Serialize)]
pub struct ToDoItems {
    pub pending_items: Vec<Base>,
    pub done_items: Vec<Base>,
    pub pending_item_count: i8,
    pub done_item_count: i8,
}

impl ToDoItems {
    pub fn new(input_items: Vec<ItemTypes>) -> Self {
        let mut pending_array_buf = Vec::new();
        let mut done_array_buf = Vec::new();

        for item in input_items {
            match item {
                ItemTypes::Pending(result) => pending_array_buf.push(result.super_struct),
                ItemTypes::Done(result) => done_array_buf.push(result.super_struct),
            }
        }

        return Self {
            done_item_count: done_array_buf.len() as i8,
            done_items: done_array_buf,
            pending_item_count: pending_array_buf.len() as i8,
            pending_items: pending_array_buf,
        };
    }
    pub fn get_state() -> Self {
        let conn = establish_connection();

        let mut array_buf = Vec::new();

        // the ".asc()" comes from the "ExpressionMethods". the context error menu gives a hint
        let items = to_do::table
            .order(to_do::columns::id.asc())
            .load::<Item>(&conn)
            .unwrap();

        for item in items {
            let status = TaskStatus::from_string(&item.status.as_str());

            let item = to_do_factory(&item.title, status);

            array_buf.push(item);
        }

        ToDoItems::new(array_buf)
    }
}

impl Responder for ToDoItems {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        let body = serde_json::to_string(&self).unwrap();
        HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(body)
    }
}
