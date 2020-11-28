use crate::generator::{md_to_html, rf};
use crate::menu::{build_tree, dir, Dir, Path};
use actix_web::{error::InternalError, http::StatusCode, HttpRequest, HttpResponse};
use sailfish::TemplateOnce;
use std::io::{Error, ErrorKind};
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
pub struct Greet<'a> {
    pub lang: &'a str,
    pub menu: String,
    pub content: String,
}

pub async fn greet(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    // let all = WalkDir::new("content")
    //     .follow_links(true)
    //     .sort_by(|a, b| a.file_name().cmp(b.file_name()));

    // let paths: Vec<Path> = all.into_iter().fold(Vec::new(), |mut v, e| {
    //     let d = e.unwrap().path().display().to_string();
    //     v.push(Path::new(&d));
    //     v
    // });

    // let mut top = dir("root");
    // for path in paths.iter() {
    //     build_tree(&mut top, &path.parts, 0);
    // }

    let req_path = if req.path().to_string() == "/" {
        String::from("/en/")
    } else {
        req.path().to_string()
    };

    if std::path::Path::new(&format!("content{}", req_path)).is_dir() {
        let mut p1 = format!("{}", req_path);
        p1.push_str("index.md");
        return Ok(HttpResponse::Found()
            .header(actix_web::http::header::LOCATION, p1)
            .finish()
            .into_body());
    };

    let mut file_path = format!("content{}", req_path);
    let mut vec_path = file_path.split("/").collect::<Vec<&str>>();
    vec_path.pop();
    file_path = vec_path.join("/");

    match req.match_info().get("lang") {
        Some(lang) => match lang {
            "en" => {
                let content_reader = md_to_html(&file_path);
                match content_reader {
                    Ok(content) => {
                        let menu_reader = md_to_html(&format!("menu/{}.md", lang));
                        match menu_reader {
                            Ok(menu) => {
                                let body_builder = Greet {
                                    lang,
                                    menu,
                                    content,
                                }
                                .render_once()
                                .map_err(|e| {
                                    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
                                });
                                match body_builder {
                                    Ok(body) => Ok(HttpResponse::Ok()
                                        .content_type("text/html; charset=utf-8")
                                        .body(body)),
                                    Err(e) => Err(actix_web::Error::from(e)),
                                }
                            }
                            Err(e) => {
                                println!("Reading menu error: \n{}", e);
                                Err(actix_web::Error::from(e))
                            }
                        }
                    }
                    Err(e) => {
                        println!("Reading content error: \n{}", e);
                        match e.kind() {
                            ErrorKind::NotFound => Ok(HttpResponse::Ok()
                                .content_type("text/html; charset=utf-8")
                                .body(rf("public/en.404.html").unwrap())),
                            _ => {
                                println!("Reading menu error: \n{}", e);
                                Err(actix_web::Error::from(e))
                            }
                        }
                    }
                }
            }
            "kh" => {
                let content_reader = md_to_html(&file_path);
                match content_reader {
                    Ok(content) => {
                        let menu_reader = md_to_html(&format!("menu/{}.md", lang));
                        match menu_reader {
                            Ok(menu) => {
                                let body_builder = Greet {
                                    lang,
                                    menu,
                                    content,
                                }
                                .render_once()
                                .map_err(|e| {
                                    InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR)
                                });
                                match body_builder {
                                    Ok(body) => Ok(HttpResponse::Ok()
                                        .content_type("text/html; charset=utf-8")
                                        .body(body)),
                                    Err(e) => Err(actix_web::Error::from(e)),
                                }
                            }
                            Err(e) => {
                                println!("Reading menu error: \n{}", e);
                                Err(actix_web::Error::from(e))
                            }
                        }
                    }
                    Err(e) => {
                        println!("Reading content error: \n{}", e);
                        match e.kind() {
                            ErrorKind::NotFound => Ok(HttpResponse::Ok()
                                .content_type("text/html; charset=utf-8")
                                .body(rf("public/kh.404.html").unwrap())),
                            _ => {
                                println!("Reading menu error: \n{}", e);
                                Err(actix_web::Error::from(e))
                            }
                        }
                    }
                }
            }
            _ => Ok(HttpResponse::Ok()
                .content_type("text/html; charset=utf-8")
                .body(rf("public/en.404.html").unwrap())),
        },
        None => Ok(HttpResponse::Found()
            .header(
                actix_web::http::header::LOCATION,
                format!("/en/{}", req_path),
            )
            .finish()
            .into_body()),
    }
}
