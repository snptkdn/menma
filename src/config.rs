use anyhow::Result;
use home_dir::*;
use serde::Deserialize;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dir_path: PathBuf,
    pub editor: String,
}

pub fn config(config_file: &PathBuf) -> Result<Config> {
    println!("{:?}", config_file);

    let file = match File::open(config_file) {
        Ok(file) => file,
        Err(_) => {
            println!("Config file not found. Creating one at {:?}", config_file);
            if let Some(parent) = config_file.parent() {
                std::fs::create_dir_all(parent)?;
            }
            File::create(config_file)?
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    let mut config = from_reader::<BufReader<File>, Config>(reader)?;
    config.dir_path = config.dir_path.expand_home()?;

    Ok(config)
}
