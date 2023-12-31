use clap::{Parser, Subcommand};
use home_dir::*;
use std::path::Path;
mod add;
mod config;
mod exec;
mod init;
mod ls;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Add { title: String, tags: Vec<String> },
    Ls { tags: Vec<String> },
    Init,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    
    let config_path = &Path::new("~/.config/menma/config.json").expand_home()?;

    match args.command {
        Commands::Add { title, tags } => {
            let config = config::config(config_path)?;
            add::add(config.dir_path, title, &tags)
        }
        Commands::Ls { tags } => {
            let config = config::config(config_path)?;
            ls::ls(config.dir_path, tags, &config.editor)
        }
        Commands::Init => {
            let config_path = &Path::new("~/.config/menma/config.json").expand_home()?;
            init::init(config_path)
        }
    }
}
