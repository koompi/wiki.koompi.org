use walkdir::WalkDir;

pub struct View {
    menu: Vec<MenuItem>,
    content: String,
}

pub struct MenuItem {
    order: u32,
    title: String,
    address: String,
    children: Option<Vec<MenuItem>>,
}

pub fn get_all() -> Result<(), std::io::Error> {
    let all = WalkDir::new("content").follow_links(true);
    let dirs = all
        .into_iter()
        // .filter(|e| e.as_ref().unwrap().path().is_dir())
        .map(|e| e.unwrap().path().display().to_string())
        .collect::<Vec<String>>();
    // // dirs.sort();

    Ok(())
}
