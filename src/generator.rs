use pulldown_cmark::{html, Options, Parser};
use std::{fs::create_dir_all, fs::File, io::prelude::*, io::Error, path::Path};
use walkdir::WalkDir;

pub fn rf(path: &str) -> Result<String, Error> {
    println!("{}", path);
    let f = File::open(path);
    let mut data: String = String::new();

    match f {
        Ok(mut file) => match file.read_to_string(&mut data) {
            Ok(_) => Ok(data.to_string()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub fn wf(data: String, path: &str) -> Result<(), Error> {
    let file = File::create(path);

    match file {
        Ok(mut f) => match f.write_all(data.as_bytes().as_ref()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        },
        Err(e) => Err(e),
    }
}

pub fn md_to_html(input: &str) -> Result<String, Error> {
    let mardown_input = rf(input);

    match mardown_input {
        Ok(data) => {
            let mut options = Options::empty();
            options.insert(Options::all());
            let parser = Parser::new_ext(&data, options);

            let mut html_output = String::new();
            html::push_html(&mut html_output, parser);

            Ok(html_output)
        }
        Err(e) => Err(e),
    }
}

// pub fn gen_all() {
//     let dirs = WalkDir::new("content");

//     let mut data: Vec<String> = dirs
//         .into_iter()
//         .map(|d| {
//             d.unwrap()
//                 .path()
//                 .display()
//                 .to_string()
//                 .split('/')
//                 // .filter(|root| root.to_string() != "content")
//                 .map(|e| e.to_string())
//                 .collect::<Vec<String>>()
//                 .join("/")
//         })
//         .collect::<Vec<String>>()
//         .iter()
//         .filter(|e| !e.is_empty())
//         .map(|e| e.to_string())
//         .collect::<Vec<String>>();

//     data.sort();

//     let (_, file): (Vec<String>, Vec<String>) =
//         data.iter().cloned().partition(|e| Path::new(e).is_dir());

//     let en: Vec<String> = file
//         .iter()
//         .cloned()
//         .filter(|f| f.contains(".en.md"))
//         .collect();

//     let kh: Vec<String> = file
//         .iter()
//         .cloned()
//         .filter(|f| f.contains(".kh.md"))
//         .collect();

//     en.iter().for_each(|f| {
//         let mut name: String = f
//             .split("/")
//             .collect::<Vec<&str>>()
//             .last()
//             .unwrap()
//             .split(".")
//             .collect::<Vec<&str>>()
//             .first()
//             .unwrap()
//             .to_string();
//         name.push_str(".html");

//         let mut path = f.split("/").map(|f| f.to_string()).collect::<Vec<String>>();
//         path[0] = "static".to_string();
//         path.pop();
//         create_dir_all(path.join("/")).unwrap();

//         path.push(name);
//         md_to_html(f, path.join("/").as_str()).unwrap();
//     });
//     kh.iter().for_each(|f| {
//         let mut name: String = f
//             .split("/")
//             .collect::<Vec<&str>>()
//             .last()
//             .unwrap()
//             .split(".")
//             .collect::<Vec<&str>>()
//             .first()
//             .unwrap()
//             .to_string();
//         name.push_str(".html");

//         let mut path = f.split("/").map(|f| f.to_string()).collect::<Vec<String>>();
//         path[0] = "static".to_string();
//         path.pop();
//         create_dir_all(path.join("/")).unwrap();

//         path.push(name);
//         md_to_html(f, path.join("/").as_str()).unwrap();
//     });
// }
