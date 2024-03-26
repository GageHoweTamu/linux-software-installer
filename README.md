# Universal-Installer

Bringing the ease of Windows installers to Linux

This is a gui app to give users a more convenient, Windows-like app installation process. Users will download a "install script" from a software vendor or the integrated software store, and use it to install their software.

Resources:
https://www.youtube.com/watch?v=DHjqpvDnNGE - JS
https://www.youtube.com/watch?v=5C_HPTJg5ek - Rust
https://www.youtube.com/watch?v=-X8evddpu7M&t=158s - Tauri
https://www.youtube.com/watch?v=uPXn9S31o7Q - Solid.js
https://www.youtube.com/watch?v=hw3Bx5vxKl0 - Solid.js

What you'll need:    
```
sudo apt update    
sudo apt install nodejs    
sudo apt install npm
```  

To build:    
```npm run tauri dev``` in src-tauri

# Tauri + Solid + Typescript

Tauri, Solid and Typescript in Vite

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

cd universal-installer
npm install
npm run tauri dev

if you have issues, update npm and install vite.

possible missing dependencies:
sudo apt-get install -y libgtk-3-dev (and other core libraries)
cargo install tauri-cli
sudo npm install --global yarn
etc
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig
