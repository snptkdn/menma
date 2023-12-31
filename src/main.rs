use clap::{Parser, Subcommand};
use home_dir::*;
use std::path::{Path, PathBuf};
mod add;
mod config;
mod ls;
mod exec;

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
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let config = config::config(&Path::new("~/.config/menma/config.json").expand_home()?)?;
    if cfg!(debug_assertions) {
        println!("config info: {:?}", config);
    }

    match args.command {
        Commands::Add { title, tags } => {
            add::add(config.dir_path, title, &tags);
            Ok(())
        }
        Commands::Ls { tags } => {
            ls::ls(config.dir_path, tags);
            Ok(())
        }
    }
}
