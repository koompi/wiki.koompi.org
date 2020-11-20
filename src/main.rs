mod config;
mod generator;
mod menu;
#[macro_use]
extern crate sailfish_macros;

use actix_cors::Cors;
use actix_files;
use actix_web::error::InternalError;
use actix_web::http::{self, StatusCode};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use generator::md_to_html;
use menu::{build_tree, dir, Dir, Path};
use sailfish::TemplateOnce;
use walkdir::WalkDir;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet<'a> {
    lang: &'a str,
    routes: Dir,
    content: String,
}

async fn greet(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let all = WalkDir::new("content")
        .follow_links(true)
        .sort_by(|a, b| a.file_name().cmp(b.file_name()));

    let paths: Vec<Path> = all.into_iter().fold(Vec::new(), |mut v, e| {
        let d = e.unwrap().path().display().to_string();
        v.push(Path::new(&d));
        v
    });

    let mut top = dir("root");
    for path in paths.iter() {
        build_tree(&mut top, &path.parts, 0);
    }

    let lang = req.match_info().get("lang").unwrap_or("en");
    // let path = req.path();
    // let file_path = format!("content{}/index.md", path);
    let content = md_to_html("content/en/Applications/Browser/index.md").unwrap();
    // println!("{:?}", path);
    let body = Greet {
        lang,
        routes: top,
        content: content,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
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
            .route("/{lang}/{chapter}/{lesson}", web::get().to(greet))
            .route("/{lang}/{chapter}", web::get().to(greet))
            .route("/{lang}", web::get().to(greet))
            .route("/", web::get().to(greet))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
