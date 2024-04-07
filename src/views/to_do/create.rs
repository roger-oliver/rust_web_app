use actix_web::{HttpRequest, HttpResponse};
use diesel::{
    query_dsl::methods::{FilterDsl, OrderDsl},
    ExpressionMethods, RunQueryDsl,
};

use crate::{
    database::establish_connection, json_serialization::to_do_items::ToDoItems, jwt::JwToken, models::item::{item::Item, new_item::NewItem}, schema::to_do
};

pub async fn create(req: HttpRequest, _: JwToken) -> HttpResponse {
    // let state: Map<String, Value> = read_file("./state.json");

    let title = req.match_info().get("title").unwrap();

    let conn = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(title))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();

    if items.len() == 0 {
        let new_post = NewItem::new(title.to_owned(), 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&conn);
    }

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
