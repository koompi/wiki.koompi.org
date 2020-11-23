// pub mod config;
pub mod generator;
pub mod handler;
pub mod menu;
#[macro_use]
extern crate sailfish_macros;

use actix_cors::Cors;
use actix_files;
use actix_web::{web, App, HttpServer};
use handler::greet;

/// 404 handler
async fn p404() -> String {
    // Ok(fs::NamedFile::open("./public/index.html")?.set_status_code(StatusCode::NOT_FOUND))
    String::from("Page not found")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method()
            .max_age(3600);

        App::new()
            .wrap(cors)
            .service(actix_files::Files::new("/public", "./public"))
            .route(
                "/{lang}/{chapter}/{lesson}/{section}/{file}",
                web::get().to(greet),
            )
            .route("/{lang}/{chapter}/{lesson}/{section}", web::get().to(greet))
            .route("/{lang}/{chapter}/{lesson}", web::get().to(greet))
            .route("/{lang}/{chapter}", web::get().to(greet))
            .route("/{lang}", web::get().to(greet))
            .route("/", web::get().to(greet))
            .default_service(web::resource("").route(web::get().to(p404)))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
