# BandcampOnlinePlayer
[![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg)](https://forthebadge.com)


**IN DEVELOPMENT! - breakable changes and random bugs are inevitable. Use it on own risk and report bugs in "issues" section!**

![CI Build](https://github.com/LaineZ/bc_rs/workflows/CI%20Build/badge.svg)

A next cross-platform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust with Sciter library!

A simple desktop-orienteted client for bandcamp.com - Allows listen albums from tags or URL's much easier; and provides features like play queue, low memory/cpu usage rather than browser, etc.

## Features

* Audio playback from site in mp3 128k quality
* Playback control: seek, pause, next, prev, volume ...
* Play queue: add/remove album tracks
* Album explorer: allows to explore albums in specified tag
* Global search aroud website
* **AND MORE**

You can download latest dev version from "actions" menu:
https://github.com/LaineZ/bc_rs/actions

<!-- ## Installation
If you have [https://crates.io/](cargo) installed. bc-rs can be installed using this commands:

1. If you run on Linux you need install these audio libs: ``libaudio``, ``libasound2``, ``libxcb-shape0-dev``, ``libxcb-xfixes0-dev``
   1. On Ubuntu/Debian you can install with this command: ``sudo apt install libaudio-dev libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev``
   2. On Void linux you can install with this command ``sudo xbps-install alsa-lib-devel libxcb-devel``
2. Run this command: ``cargo install --git https://github.com/LaineZ/bc_rs.git``
3. DONE! You can run it with ``bc_rs`` command -->

## Building

1. [Download Rust](https://www.rust-lang.org/learn/get-started)
2. ```$ git clone https://github.com/LaineZ/bc_rs.git```
3. ```$ cd bc_rs```
4. ```$ ./download.sh``` - This script downloads Sciter SDK for build on linux or mac. On windows you need download sciter manually [here](https://gitlab.com/sciter-engine/sciter-js-sdk/-/archive/4.4.9.3/sciter-js-sdk-4.4.9.3.zip) and extract in project directory.
5. If you run on Linux you need install these audio libs: ``libaudio``, ``libasound2``, ``libxcb-shape0``, ``libxcb-xfixes0``
   1. On Ubuntu/Debian you can run that command ``sudo apt install libaudio-dev libasound2-dev libxcb-shape0-dev libxcb-xfixes0-dev``
   2. On Void linux you can install with this command ``sudo xbps-install alsa-lib-devel libxcb-devel``
   3. On Arch linux you can install with this command ``sudo pacman -S alsa-lib libxcb``
6. ```cargo build --release```
7. ```cd target/release```
8. DONE! You can run it

## Current development screenshots

![bc](/resources/screenshot1.png)
![bc](/resources/screenshot2.png)

## Notice
This application **DOES NOT** provide any ways to **DOWNLOAD** music on computer. it's just make easier stream/play music from site and finding cool tracks =)