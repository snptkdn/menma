use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use dialoguer::{theme::ColorfulTheme, Select};
use regex::Regex;
use std::{io::stdout, path::PathBuf};

use crate::exec;

pub fn ls(dir_path: PathBuf, target_tags: Vec<String>, editor: &String) -> Result<()> {
    let files = std::fs::read_dir(dir_path)?;
    let mut target_files = Vec::new();

    for file in files {
        let file_path = file?.path();
        if target_tags
            .iter()
            .all(|tag| tags(&file_path.file_name().unwrap().to_str().unwrap()).contains(tag))
        {
            target_files.push(file_path);
        }
    }

    select_event(&target_files, editor)?;

    Ok(())
}

fn tags(file_name: &str) -> Vec<String> {
    let re = Regex::new(r"#(\w+)").unwrap();

    re.captures_iter(&file_name)
        .map(|cap| cap[1].to_string())
        .collect()
}

pub fn select_event(target_files: &Vec<PathBuf>, editor: &String) -> Result<()> {
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select file")
        .items(
            &target_files
                .iter()
                .map(|file| file.file_name().unwrap().to_str().unwrap())
                .collect::<Vec<&str>>(),
        )
        .interact_opt()?;

    match selection {
        Some(index) => {
            exec::call_subprocess(&target_files[index], editor)?;
        }
        None => {
            println!("Canceled");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags() {
        let file_name = String::from("20231020_test_#aws#dev#console.md");
        let tags = tags(&file_name);
        assert_eq!(tags, vec!["aws", "dev", "console"])
    }
}
