// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// use tauri::Error;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_file(filename: &str) -> String { // return file content as a string
    match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => e.to_string()
    }
}

/*
# sample installScript for Spotify
[Debian]
curl -sS https://download.spotify.com/debian/pubkey_6224F9941A8AA6D1.gpg | sudo gpg --dearmor --yes -o /etc/apt/trusted.gpg.d/spotify.gpg
echo "deb http://repository.spotify.com stable non-free" | sudo tee /etc/apt/sources.list.d/spotify.list
sudo apt-get update && sudo apt-get install spotify-client
[Ubuntu]
snap install spotify
#[other distros etc]
[end]
 */
// from javascript: call read_file -> parse_script -> run_bash_script
// we need to implement this in Solid.js
// buttons to (select the installScript) and (run the script), with a script preview on the side
// two sides of the ui: install local file, and install from store. install from store will depend on a website/api, and is not important yet.
// The install local file side will be the focus. We need to be able to read the file, parse the script, and run the script.

/*
#[tauri::command]   // this function reads the installScript and returns a string depending on the OS
                    // Then fn run_bash_script is called with the returned string
fn parse_script(script: &str) -> Result<String, std::io::Error> {
    let os = os_info::get(); // get the OS info
    let os_name = os.os_type();
    let mut script_path = String::new();
    let mut script_lines = script.lines();
    let mut line = script_lines.next();
    while line.is_some() {
        if line.unwrap().contains(&os_name) {
            line = script_lines.next();
            while line.is_some() && !line.unwrap().contains("[end]") {
                script_path.push_str(line.unwrap());
                script_path.push('\n');
                line = script_lines.next();
            }
            break;
        }
        line = script_lines.next();
    }
    Ok(script_path)
}

#[tauri::command]
fn run_bash_script(script_path: &str) -> Result<(), tauri::Error> { // runs the selected bash script
    let mut command = std::process::Command::new("bash");
    command.arg(script_path);

    let status = command.status().map_err(|e| e.into())?;
    if status.success() {
        println!("Script executed successfully!");
    } else {
        eprintln!("Script execution failed with exit code: {}", status.code().unwrap_or(-1));
    }
    Ok(())
}
*/

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, read_file]) // update this when adding new functions
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}