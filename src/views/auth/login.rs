use actix_web::{web, HttpResponse, Responder};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

use crate::jwt::JwToken;
use crate::{
    database::DB, json_serialization::login::Login, models::user::user::User, schema::users,
};

pub async fn login(credentials: web::Json<Login>, db: DB) -> impl Responder {
    let password = credentials.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound();
    } else if users.len() > 1 {
        return HttpResponse::Conflict();
    }

    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            HttpResponse::Ok().append_header(("token", raw_token)).take()
        },
        false => HttpResponse::Unauthorized()
    }
}
