use std::io;
use std::path::PathBuf;
use std::process::Command;

pub fn call_subprocess(path_to_file: &PathBuf, editor: &String) -> io::Result<()> {
    // Vimを起動する
    let mut child = Command::new(editor)
        .arg(path_to_file)
        .spawn() // プロセスを生成して実行
        .unwrap_or_else(|_| panic!("failed to execute child: {}", editor));

    // Vimの終了を待つ
    child.wait()?;

    Ok(())
}
