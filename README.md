# BandcampOnlinePlayer Rust Edition
**IN DEVELOPMENT! - breakable changes are inevitable**

![Create Release](https://github.com/LaineZ/bc_rs/workflows/Download/badge.svg?event=push)


A next crossplatform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust!

A simple client for bandcamp.com that's allows listen albums from tags much easier; and provides features like play queue, low memory/cpu usage rather than browser, etc.

You can download latest version from "actions" menu
https://github.com/LaineZ/bc_rs/actions

### CLI/TUI Mode
This a terminal based interface that uses cross-platform [crossterm]([https://github.com/crossterm-rs/crossterm) library

### Running

### Linux/Mac

Works on any Linux with ALSA-compitable sound card but CI-Builds and future release builds supports **only amd64** architecture. If you want run this on i386, arm64, armhf, ... you need compile it youself.
```./bandcamp-online-cli```

![Screenshot](https://i.imgur.com/jKar1mc.png)

### Windows

``bandcamp-online-cli.exe``

![ScreeshotWindows](https://i.imgur.com/NIg76L6.png)

### TUI Mode controls
**General keys**

<kbd>Esc</kbd> - closes program

<kbd>H</kbd> - hide/show tag list

<kbd>Q</kbd> - switch to queue list

<kbd>C</kbd> - add album/track to queue by URL from clipboard

<kbd>⭾</kbd> - cycle between views

<kbd>↑</kbd> <kbd>↓</kbd> - navigation

<kbd>PageUp</kbd> <kbd>PageDown</kbd> - scroll pages

**Playback only**

<kbd>←</kbd> - move track backwards by 5 secs

<kbd>→</kbd> - move track forward by 5 secs

<kbd>W</kbd> <kbd>S</kbd> - inrease / decrease volume by 1%

<kbd>O</kbd> - open current playing album URL in browser

<kbd>Delete</kbd> - clear selected list

**Tags list only**

<kbd>Space</kbd> - add tag

<kbd>Enter</kbd> - load albums by tags

<kbd>Delete</kbd> - diselect all tags

**Queue list only**

<kbd>Enter</kbd> - play selected track

<kbd>Space</kbd> - play/pause current track

**Album list only**

<kbd>Enter</kbd> - add selected album to queue

## Building
1. [Download Rust]([https://www.rust-lang.org/learn/get-started) and run this command
2. ```git clone https://github.com/LaineZ/bc_rs.git```
3. ```cd bc_rs```
4. If you run on Linux you need install these audio libs: ``libaudio``, ``libasound2``, ``libxcb-shape0-dev``, ``libxcb-xfixes0-dev``
   1. On Ubuntu/Debian you can just run that command ``sudo apt install libaudio-dev libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev``
5. ```cargo build --release```
6. ```cd target/release```
7. DONE! You can run in TUI Mode by instructions above

## Notice
This application **DOES NOT** provide any ways to **DOWNLOAD** music on computer. it's just make easier stream/play music from site and finding cool tracks =)
