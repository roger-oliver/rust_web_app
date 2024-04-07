use std::collections::HashMap;

use actix_web::{web, HttpResponse};
use diesel::{QueryDsl, ExpressionMethods, RunQueryDsl};

use crate::jwt::JwToken;
use crate::{
    database::DB, json_serialization::login::Login, models::user::user::User, schema::users,
};

pub async fn login(credentials: web::Json<Login>, db: DB) -> HttpResponse {
    let password = credentials.password.clone();

    let users = users::table
        .filter(users::columns::username.eq(credentials.username.clone()))
        .load::<User>(&db.connection).unwrap();

    if users.len() == 0 {
        return HttpResponse::NotFound().body("invalid credentials");
    } else if users.len() > 1 {
        return HttpResponse::Conflict().finish();
    }

    match users[0].verify(password) {
        true => {
            let token = JwToken::new(users[0].id);
            let raw_token = token.encode();
            let mut body = HashMap::new();
            body.insert("token", raw_token);
            HttpResponse::Ok().json(body)
        },
        false => HttpResponse::Unauthorized().finish()
    }
}
