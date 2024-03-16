// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


#[tauri::command]
fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

#[tauri::command]
fn run_bash_script(script_path: &str) -> Result<(), std::io::Error> {
    let mut command = std::process::Command::new("bash");
    command.arg(script_path);

    let status = command.status()?;
    if status.success() {
        println!("Script executed successfully!");
    } else {
        eprintln!("Script execution failed with exit code: {}", status.code().unwrap_or(-1));
    }

    Ok(())
}

// takes a string containing multiple bash commands and runs them
/*
use std::fs;

fn read_bash_script(filename: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(filename)
}

use std::process::Command;

fn run_bash_script(script_path: &str) -> Result<(), std::io::Error> {
    let mut command = Command::new("bash");
    command.arg(script_path);

    let status = command.status()?;
    if status.success() {
        println!("Script executed successfully!");
    } else {
        eprintln!("Script execution failed with exit code: {}", status.code().unwrap_or(-1));
    }

    Ok(())
}

fn main() {
    if let Err(err) = run_bash_script("example_bash_script.sh") {
        eprintln!("Error executing script: {}", err);
    }
}


fn main() {
    match read_bash_script("example_bash_script.sh") {
        Ok(script_contents) => {
            println!("Bash script contents:\n{}", script_contents);
            // Now you can proceed to execute the commands inside the script.
        }
        Err(err) => eprintln!("Error reading script: {}", err),
    }
}

*/

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
