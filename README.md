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

A sample script file:

```
# sample installScript for Spotify

[Debian]
curl -sS https://download.spotify.com/debian/pubkey_6224F9941A8AA6D1.gpg | sudo gpg --dearmor --yes -o /etc/apt/trusted.gpg.d/spotify.gpg
echo "deb http://repository.spotify.com stable non-free" | sudo tee /etc/apt/sources.list.d/spotify.list
sudo apt-get update && sudo apt-get install spotify-client
[Ubuntu]
snap install spotify
#[other distros etc]
[end]

# (if snap, curl, etc do not exist on the system, prompt the user if they want to install it)

# the program reading/parsing this should run the bash scripts according to the distro the user is running.
# If it sees that sudo is required, ask for the sudo password.

# Users should be able to drag and drop installScripts into the app, and the app should install based on them.
# This progam is a universal installer for Linux. It should have: (1) this file parsing and app installation functionality, and (2) an online store where the user can download and install these scripts.
# 

```

# Tauri + Solid + Typescript

This template should help get you started developing with Tauri, Solid and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

cd universal-installer
npm install
npm run tauri dev

if you have issues, update npm and install vite.

possible missing dependencies:
apt-get install -y libgtk-3-dev
sudo apt-get install -y libsoup2.4-dev
sudo apt-get install -y libjavascriptcoregtk-4.0-dev
etc
export PKG_CONFIG_PATH=/usr/lib/x86_64-linux-gnu/pkgconfig