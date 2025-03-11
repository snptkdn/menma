use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use crate::config::Editor;

pub fn call_subprocess(path_to_file: &PathBuf, editor_map: &Vec<Editor>) -> io::Result<()> {
    let ext = match path_to_file.extension() {
        Some(ext) => ext.to_str().unwrap_or_else(|| "txt"),
        None => "txt",
    };

    let default_command = "nvim".to_string();

    let command = editor_map
        .iter()
        .filter(|editor| editor.ext == ext)
        .map(|editor| &editor.command)
        .last()
        .unwrap_or_else(|| &default_command);

    // Vimを起動する
    let mut child = Command::new("sh")
        .arg("-c")
        .arg(format!("{} {}", command, path_to_file.to_str().unwrap()))
        .spawn() // プロセスを生成して実行
        .unwrap_or_else(|_| panic!("failed to execute child: {}", command));

    // Vimの終了を待つ
    child.wait()?;

    Ok(())
}
