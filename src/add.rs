use std::fs::File;
use chrono::Local;
use anyhow::Result;

pub fn add(dir_path: String, title: String, tags: &Vec<String>) -> Result<()> {
    let tags = tags.join("#");
    File::create(format!("{}/{}_{}_#{}.md", dir_path, date(), title, tags))?;
    return Ok(());
}

fn date() -> String {
    Local::now().format("%Y%m%d").to_string()
}
