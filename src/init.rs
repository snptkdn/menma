use anyhow::Result;
use colored::*;
use dialoguer::{Confirm, Input};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

use crate::config::Config;

pub fn init(dir_path: &PathBuf) -> Result<()> {
    if dir_path.exists() {
        let text = "Config file already exists";
        println!("{}", text.red().bold());
        if Confirm::new()
            .with_prompt("Deleted existing file and create new one?")
            .default(true)
            .interact()?
        {
            std::fs::remove_dir_all(dir_path.parent().unwrap()).unwrap();
            create_config_file(dir_path)?;
        } else {
            return Err(anyhow::anyhow!("canceled."));
        }
    } else {
        create_config_file(dir_path)?;
    }

    let file = OpenOptions::new().read(true).write(true).open(dir_path)?;
    let config = input_config_params()?;

    write!(&file, "{}", serde_json::to_string(&config)?)?;

    let text = "Config file created successfully!";
    println!("{}", text.blue().bold());
    Ok(())
}

fn create_config_file(dir_path: &PathBuf) -> Result<()> {
    if let Some(parent) = dir_path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    File::create(dir_path)?;

    Ok(())
}

fn input_config_params() -> Result<Config> {
    let store_path = Input::<String>::new()
        .with_prompt("Path to directory to store notes")
        .default("~/memo".to_string())
        .interact()?;

    let editor = Input::<String>::new()
        .with_prompt("Editor to open note")
        .default("vim".to_string())
        .interact()?;

    Ok(Config {
        dir_path: PathBuf::from(store_path),
        editor,
    })
}
