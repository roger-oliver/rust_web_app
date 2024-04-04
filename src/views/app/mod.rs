use actix_web::web::{self, get};

pub mod items;
mod content_loader;
pub fn app_views_factory(app: &mut web::ServiceConfig) {
    app.route("/", get().to(items::items));
}