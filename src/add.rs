use crate::config::Editor;
use crate::exec;
use anyhow::Result;
use chrono::Local;
use std::{fs::File, path::PathBuf, fs::create_dir};
use std::path::Path;

pub fn add(dir_path: PathBuf, title: String, project: String, tags: &[String], editor_map: &Vec<Editor>) -> Result<()> {
    let tags = tags.join("#");

    let project_path = format!("{}/{}", &dir_path.to_str().unwrap(), project);
    let project_path = PathBuf::from(project_path);
    if !Path::is_dir(&project_path) {
        create_dir(project_path)?
    };
    
    let path = format!(
        "{}/{}/{}_{}_#{}.md",
        &dir_path.to_str().unwrap(),
        project,
        date(),
        title,
        tags
    );
    
    File::create(&path)?;
    exec::call_subprocess(&PathBuf::from(path), &editor_map)?;
    Ok(())
}

fn date() -> String {
    Local::now().format("%Y%m%d").to_string()
}
