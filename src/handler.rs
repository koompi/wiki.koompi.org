use crate::generator::md_to_html;
use crate::menu::{build_tree, dir, Dir, Path};
use actix_web::{error::InternalError, http::StatusCode, HttpRequest, HttpResponse};
use sailfish::TemplateOnce;
use walkdir::WalkDir;

#[derive(TemplateOnce)]
#[template(path = "index.stpl")]
struct Greet<'a> {
    lang: &'a str,
    // routes: Dir,
    menu: String,
    content: String,
}

pub async fn greet(req: HttpRequest) -> actix_web::Result<HttpResponse> {
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
    let path = req.path();
    println!("{}", path);
    let file_path = format!("content{}", path);
    let content = md_to_html(&file_path).unwrap();
    let menu = md_to_html("menu.md").unwrap();

    let body = Greet {
        lang,
        // routes: top,
        menu,
        content: content,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(body))
}
