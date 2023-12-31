use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn call_subprocess(path_to_file: &PathBuf) -> io::Result<()> {
    // Vimを起動する
    let mut child = Command::new("nvim")
        .arg(path_to_file)
        .spawn() // プロセスを生成して実行
        .expect("Vimの起動に失敗しました");

    // Vimの終了を待つ
    child.wait()?;

    Ok(())
}
