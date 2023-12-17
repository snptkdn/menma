use std::fs::File;
use std::io::BufReader;
use serde_json::from_reader;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub dir_path: String,
}

pub fn config(config_file: String) -> Result<Config> {
    println!("{:?}", config_file);
    let file = File::open(config_file)?;
    let reader: BufReader<File> = BufReader::new(file);

    let config = from_reader::<BufReader<File>, Config>(reader)?;

    Ok(config)
}

