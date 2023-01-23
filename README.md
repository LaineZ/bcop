# BandcampOnlinePlayer Rust Edition
**IN DEVELOPMENT! - breakable changes and random bugs are inevitable. Use it on own risk and report bugs in "issues" section!**

![CI Build](https://github.com/LaineZ/bc_rs/workflows/CI%20Build/badge.svg)


A next cross-platform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust!

A simple desktop-orienteted client for bandcamp.com - Allows listen albums from tags or URL's much easier; and provides features like play queue, low memory/cpu usage rather than browser, etc.

<!-- You can download latest dev version from "actions" menu:
https://github.com/LaineZ/bc_rs/actions -->

## Installation
If you have [https://crates.io/](cargo) installed. bc-rs can be installed using this commands:

1. If you run on Linux you need install these audio libs: ``libaudio``, ``libasound2``, ``libxcb-shape0-dev``, ``libxcb-xfixes0-dev``
   1. On Ubuntu/Debian you can install with this command: ``sudo apt install libaudio-dev libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev``
   2. On Void linux you can install with this command ``sudo xbps-install alsa-lib-devel libxcb-devel``
2. Run this command: ``cargo install --git https://github.com/LaineZ/bc_rs.git``
3. DONE! You can run it with ``bc_rs`` command

## Building
1. [Download Rust]([https://www.rust-lang.org/learn/get-started) and run this command
2. ```git clone https://github.com/LaineZ/bc_rs.git```
3. ```cd bc_rs```
4. If you run on Linux you need install these audio libs: ``libaudio``, ``libasound2``, ``libxcb-shape0-dev``, ``libxcb-xfixes0-dev``
   1. On Ubuntu/Debian you can just run that command ``sudo apt install libaudio-dev libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev``
   2. On Void linux you can install with this command ``sudo xbps-install alsa-lib-devel libxcb-devel``
5. ```cargo build --release```
6. ```cd target/release```
7. DONE! You can run in CLI/TUI Mode by instructions above

## Notice
This application **DOES NOT** provide any ways to **DOWNLOAD** music on computer. it's just make easier stream/play music from site and finding cool tracks =)
