use regex::Regex;
use anyhow::Result;
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{read, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Write};


pub fn ls(dir_path: String, target_tags: Vec<String>) -> Result<()> {
    let files = std::fs::read_dir(dir_path)?;
    let mut target_files = Vec::new();

    for file in files {
        let file_name = file?.file_name().into_string().unwrap();
        if target_tags.iter().all(|tag| tags(&file_name).contains(tag)) {
            target_files.push(file_name);
        }
    }

    select_event(&target_files)?;

    Ok(())
}

fn tags(file_name: &String) -> Vec<String> {
    let re = Regex::new(r"#(\w+)").unwrap();

    re.captures_iter(&file_name)
      .map(|cap| cap[1].to_string())
      .collect()
}

pub fn select_event(target_files: &Vec<String>) -> Result<()> {
    let mut stdout = stdout();
    let mut selected = 0;

    stdout.execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    stdout.execute(Hide)?;

    loop {
        for (index, item) in target_files.iter().enumerate() {
            if index == selected {
                stdout.execute(MoveTo(0, index as u16))?;
                println!("> {}", item);
            } else {
                stdout.execute(MoveTo(0, index as u16))?;
                println!("  {}", item);
            }
        }

        match read()? {
            Event::Key(event) => {
                match event.code {
                    KeyCode::Char('k') => {
                        if selected > 0 {
                            selected -= 1;
                        }
                    }
                    KeyCode::Char('j') => {
                        if selected < target_files.len() - 1 {
                            selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        println!("Selected: {}", target_files[selected]);
                        if target_files[selected] == "Exit" {
                            break;
                        }
                    }
                    KeyCode::Char('q') => {
                        break;
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    disable_raw_mode()?;
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
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


