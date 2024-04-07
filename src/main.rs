// this attribute is really important to make the references from schema file!!!!!
// is a declaration that tells the Rust compiler to link against the Diesel library, 
// making its contents available to the current crate. Before the 2018 edition of Rust, 
// this was the standard way to include external crates within a Rust project.
#[macro_use] extern crate diesel;
mod json_serialization;
mod to_do;
mod views;
mod jwt;
mod database;
mod models;
mod schema;
mod config;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{App, HttpServer};

use crate::views::views_factory;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        println!("http server is starting!");
        let cors = Cors::default().allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap_fn(|req, srv| {
                println!("it came from middleware!!! {:?}", req);
                let future = srv.call(req);
                async {
                    let result = future.await?;
                    Ok(result)
                }
            })
            .configure(views_factory)
            .wrap(cors);
        return app;
    })
    .bind("127.0.0.1:8080")?
    .workers(3)
    .run()
    .await
}
