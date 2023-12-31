use anyhow::Result;
use serde::{Serialize, Deserialize};
use colored::*;
use home_dir::*;
use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub dir_path: PathBuf,
    pub editor: String,
}

pub fn config(config_file: &PathBuf) -> Result<Config> {
    let file = match File::open(config_file) {
        Ok(file) => file,
        Err(_) => {
            let text = "Config file not found";
            println!("{} at {:?}", text.red().bold(), config_file);
            Err(anyhow::anyhow!("please run init command"))?
        }
    };
    let reader: BufReader<File> = BufReader::new(file);

    let mut config = match from_reader::<BufReader<File>, Config>(reader) {
        Ok(config) => config,
        Err(err) => {
            let text = "Config file is invalid at this line↓";
            println!("{}", text.red().bold());
            Err(anyhow::anyhow!(
                "{}\nplease modify config.json or initialize config.json by init command",
                err
            ))?
        }
    };
    config.dir_path = config.dir_path.expand_home()?;

    Ok(config)
}
