use actix_web::{web, HttpResponse};
use diesel::{
    query_dsl::methods::{FilterDsl, OrderDsl},
    ExpressionMethods, RunQueryDsl,
};

use crate::{
    database::establish_connection,
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    models::item::item::Item,
    schema::to_do,
};

pub async fn delete(to_do_item: web::Json<ToDoItem>, _: JwToken) -> HttpResponse {
    let conn = establish_connection();

    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&conn)
        .unwrap();

    let _ = diesel::delete(&items[0]).execute(&conn);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
