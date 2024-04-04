use actix_web::web::{get, scope, ServiceConfig};

use self::{login::login, logout::logout};

mod login;
mod logout;

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(scope("v1/auth"))
        .route("login", get().to(login))
        .route("logout", get().to(logout));
}