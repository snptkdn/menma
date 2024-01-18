use crate::exec;
use anyhow::Result;
use chrono::Local;
use std::{fs::File, path::PathBuf};

pub fn add(dir_path: PathBuf, title: String, tags: &[String], editor: String) -> Result<()> {
    let tags = tags.join("#");
    let path = format!(
        "{}/{}_{}_#{}.md",
        &dir_path.to_str().unwrap(),
        date(),
        title,
        tags
    );
    File::create(&path)?;
    exec::call_subprocess(&PathBuf::from(path), &editor)?;
    Ok(())
}

fn date() -> String {
    Local::now().format("%Y%m%d").to_string()
}
