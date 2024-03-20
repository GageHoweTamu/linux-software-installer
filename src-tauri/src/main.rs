// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::{Command, exit};
use std::str;
// use tauri::Error;

#[tauri::command]
fn read_file(filename: &str) -> String { // return file content as a string
    match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => e.to_string()
    }
}

// from javascript: call read_file -> parse_script -> run_bash_script
// we need to implement this in Solid.js
// buttons to (select the installScript) and (run the script), with a script preview on the side
// two sides of the ui: install local file, and install from store. install from store will depend on a website/api, and is not important yet.
// The install local file side will be the focus. We need to be able to read the file, parse the script, and run the script.

#[tauri::command]   
fn parse_script(script: &str) -> String {
    let os = os_info::get(); 
    let os_name = os.os_type();
    let mut script_path = String::new();
    let mut script_lines = script.lines();
    let mut line = script_lines.next();
    while line.is_some() {
        let line_str = match line {
            Some(l) => l,
            None => return "Error: Failed to read line".to_string(),
        };
        if line_str.contains(&os_name.to_string()) {
            line = script_lines.next();
            while line.is_some() && !line_str.contains("[end]") {
                script_path.push_str(line_str);
                script_path.push('\n');
                line = script_lines.next();
            }
            break;
        }
        line = script_lines.next();
    }
    script_path
}

#[tauri::command] // runs the selected bash script
fn run_bash_script(script_path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut command = std::process::Command::new("bash");
    command.arg(script_path);

    let status = command.status()?;
    if status.success() {
        println!("Script executed successfully!");
    } else {
        eprintln!("Script execution failed with exit code: {}", status.code().unwrap_or(-1));
    }
    Ok("ran script".to_string())
}

#[tauri::command]
fn run_bash(filepath: &str) -> String {
    let output = Command::new("bash")
        .arg(filepath)
        .output()
        .expect("Failed to execute command");

    if !output.status.success() {
        eprintln!("Error: {}", str::from_utf8(&output.stderr).unwrap());
        exit(1);
    }

    str::from_utf8(&output.stdout).unwrap().to_string()
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![read_file, run_bash]) // update this when adding new functions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}