# BandcampOnlinePlayer Rust Edition
**IN DEVELOPMENT! - breakable changes and random bugs are inevitable. Use it on own risk and report bugs in "issues" section!**

![CI Build](https://github.com/LaineZ/bc_rs/workflows/CI%20Build/badge.svg)


A next cross-platform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust!

A simple desktop-orienteted client for bandcamp.com - Allows listen albums from tags or URL's much easier; and provides features like play queue, low memory/cpu usage rather than browser, etc.

You can download latest dev version from "actions" menu:
https://github.com/LaineZ/bc_rs/actions

## Application modes

### TUI Mode:
Terminal graphics based interface. Built with [console_engine](https://github.com/VincentFoulon80/console_engine) library. Runs with ``tui`` flag

### CLI Mode:
Command line based interface. Runs with ``cli`` flag. Type ``help`` to see commands

## Running

### Linux/Mac

Works on any Linux with ALSA-compitable sound card.

CI-Builds and future release builds supports **only amd64** architecture. If you want run this on **i386, arm64, armhf, ...** you need compile it **yourself**.

```./bandcamp-online-cli [tui or cli]```

![Screenshot](https://i.imgur.com/jKar1mc.png)

### Windows

``bandcamp-online-cli.exe [tui or cli]``

![ScreeshotWindows](https://i.imgur.com/NIg76L6.png)

### TUI Mode controls
#### Currently, TUI Mode in heavy development, and can works incorrectly - use it on own risk
**General keys**

<kbd>Esc</kbd> OR <kbd>Ctrl</kbd>+<kbd>C</kbd> - closes program

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
7. DONE! You can run in CLI/TUI Mode by instructions above

## Notice
This application **DOES NOT** provide any ways to **DOWNLOAD** music on computer. it's just make easier stream/play music from site and finding cool tracks =)
