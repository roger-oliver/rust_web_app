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
use actix_web::{App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

use crate::views::views_factory;

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    const ALLOWED_VERSION: &'static str = include_str!("output_data.txt");
    HttpServer::new(|| {
        println!("http server is starting!");
        let cors = Cors::default().allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;
                if req.path().contains(ALLOWED_VERSION) {
                    passed = true;
                } else {
                    passed = false;
                }

                let end_result = match passed {
                    true => Either::Left(srv.call(req)),
                    false => {
                        let resp = HttpResponse::NotImplemented().body(format!("only the {} API is supported", ALLOWED_VERSION));
                        Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                    }
                };
                
                async move {
                    let result = end_result.await?;
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
