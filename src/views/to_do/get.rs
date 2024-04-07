use actix_web::Responder;

use crate::{
    json_serialization::to_do_items::ToDoItems, jwt::JwToken}
;

pub async fn get(_: JwToken) -> impl Responder {
    return ToDoItems::get_state();
}
