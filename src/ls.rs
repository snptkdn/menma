use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, FuzzySelect};
// use regex::Regex;
use std::path::Path;
use std::path::PathBuf;

use crate::exec;

pub fn ls(dir_path: PathBuf, target_tags: Vec<String>, editor: &String) -> Result<()> {
    let files = std::fs::read_dir(&dir_path)?;
    let projects: Vec<PathBuf> = files
        .into_iter()
        .map(|entry| entry.unwrap().path())
        .filter(|path| Path::is_dir(path))
        .collect();

    select_project(dir_path, &projects, editor)?;

    Ok(())
}

// fn tags(file_name: &str) -> Vec<String> {
//     let re = Regex::new(r"#(\w+)").unwrap();
//
//     re.captures_iter(file_name)
//         .map(|cap| cap[1].to_string())
//         .collect()
// }

pub fn select_event(target_files: &Vec<PathBuf>, editor: &String) -> Result<()> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
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

pub fn select_project(dir_path: PathBuf, projects: &Vec<PathBuf>, editor: &String) -> Result<()> {
    let selection = FuzzySelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Select project")
        .items(
            &projects
                .iter()
                .map(|project: &PathBuf| project.to_str().unwrap())
                .collect::<Vec<&str>>(),
        )
        .interact_opt()?;

    let files = match selection {
        Some(index) => std::fs::read_dir(&projects[index])?,
        None => std::fs::read_dir(dir_path)?,
    };

    let target_files: Vec<PathBuf> = files
        .into_iter()
        .map(|entry| entry.unwrap().path()) // DirEntry から PathBuf を取得
        .collect();

    select_event(&target_files, editor)?;

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
