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
mod counter;

use actix_cors::Cors;
use actix_service::Service;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

use crate::views::views_factory;

#[actix_web::main]

async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    const ALLOWED_VERSION: &'static str = include_str!("output_data.txt");

    let site_counter = counter::Counter{count: 0};

    let _ = site_counter.save();

    HttpServer::new(|| {
        println!("http server is starting!");
        let cors = Cors::default().allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let app = App::new()
            .wrap_fn(|req, srv| {
                let passed: bool;

                let mut site_counter = counter::Counter::load().unwrap();

                site_counter.count += 1;

                println!("{:?}", &site_counter);

                let _ = site_counter.save();

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
            .wrap(cors)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"));
        return app;
    })
    .bind("127.0.0.1:8080")?
    .workers(3)
    .run()
    .await
}
