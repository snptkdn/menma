use std::{fs::File, path::PathBuf};
use chrono::Local;
use anyhow::Result;

pub fn add(dir_path: PathBuf, title: String, tags: &[String]) -> Result<()> {
    let tags = tags.join("#");
    File::create(format!("{}/{}_{}_#{}.md", dir_path.to_str().unwrap(), date(), title, tags))?;
    Ok(())
}

fn date() -> String {
    Local::now().format("%Y%m%d").to_string()
}
