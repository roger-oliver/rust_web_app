use actix_web::{web, HttpResponse};
use diesel::ExpressionMethods;
use diesel::{query_dsl::methods::FilterDsl, RunQueryDsl};

use crate::database::DB;
use crate::{
    json_serialization::{to_do_item::ToDoItem, to_do_items::ToDoItems},
    jwt::JwToken,
    schema::to_do,
};

pub async fn edit(to_do_item: web::Json<ToDoItem>, _: JwToken, db: DB) -> HttpResponse {

    let results = to_do::table.filter(to_do::columns::title.eq(&to_do_item.title));

    let _ = diesel::update(results)
        .set(to_do::columns::status.eq("DONE"))
        .execute(&db.connection);

    return HttpResponse::Ok().json(ToDoItems::get_state());
}
