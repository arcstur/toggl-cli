use std::{
    io::{self, Write},
    path::{Path, PathBuf},
};

use colored::Colorize;

pub fn remove_trailing_newline(value: String) -> String {
    value.trim_end().to_string()
}

pub fn read_from_stdin(text: &str) -> String {
    print_without_buffer(text);
    let mut result = String::new();
    io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");
    remove_trailing_newline(result)
}

pub fn read_from_stdin_with_constraints(text: &str, valid_values: &[String]) -> String {
    loop {
        let result = read_from_stdin(text);
        if valid_values.contains(&result) {
            return result;
        } else {
            let error_message = format!(
                "Invalid value \"{}\". Valid values are: {}\n",
                result,
                valid_values.join(", ")
            )
            .red();
            print_without_buffer(&error_message);
        }
    }
}

pub fn open_path_in_editor<P>(path: P) -> Result<(), Box<dyn std::error::Error>>
where
    P: AsRef<Path> + std::convert::AsRef<std::ffi::OsStr>,
{
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "vim".to_string());
    let mut child = std::process::Command::new(editor)
        .arg(path)
        .spawn()
        .expect("Failed to spawn editor");
    child.wait().expect("Failed to wait for editor");
    Ok(())
}

pub fn get_git_branch_for_dir(dir: &PathBuf) -> Option<String> {
    let output = std::process::Command::new("git")
        .arg("branch")
        .arg("--show-current")
        .current_dir(dir)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let branch = String::from_utf8(output.stdout).ok()?;
    Some(branch.trim().to_string())
}

#[cfg(unix)]
pub fn get_shell_cmd(command: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("sh");
    cmd.arg("-c");
    cmd.arg(command);
    cmd
}

#[cfg(windows)]
pub fn get_shell_cmd(command: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("cmd");
    cmd.arg("/C");
    cmd.arg(command);
    cmd
}

fn print_without_buffer(text: &str) {
    print!("{}", text);
    io::stdout().flush().unwrap();
}