# BandcampOnlinePlayer
![example screenshot](/resources/bc_rs.png)
[![forthebadge](https://forthebadge.com/images/badges/powered-by-black-magic.svg)](https://forthebadge.com)
[![forthebadge](https://forthebadge.com/images/badges/60-percent-of-the-time-works-every-time.svg)](https://forthebadge.com)


**IN DEVELOPMENT! - breakable changes and random bugs are inevitable. Use it on own risk and report bugs in "issues" section!**

![CI Build](https://github.com/LaineZ/bc_rs/workflows/CI%20Build/badge.svg)

A next cross-platform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust with Sciter library!

This simple and user-friendly desktop-oriented client for Bandcamp.com makes it easier to listen to albums from tags or URLs, with features such as a **play queue** and Low memory/CPU usage, making it a superior alternative to using a web browser.

## Features

* Audio playback from site in mp3 128k quality
* Playback control: seek, pause, next, prev, volume control ...
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

1. [Download Rust](https://www.rust-lang.org/learn/get-started) and follow installation instructions
2. ```$ git clone https://github.com/LaineZ/bc_rs.git```
3. ```$ cd bc_rs```
4. ```$ ./download.sh``` - This script downloads Sciter SDK for build on linux or mac. On Windows you can use ``download.ps1`` script. If powershell script fails to run. Try run this command: ``Set-ExecutionPolicy -ExecutionPolicy Unrestricted -Scope CurrentUser`` and try again.
5. If you run on Linux you need install these libs: ``libxcb-shape0``, ``libxcb-xfixes0``
   1. On Ubuntu/Debian you can run that command ``sudo apt install libxcb-shape0-dev libxcb-xfixes0-dev``
   2. On Void linux you can install with this command ``sudo xbps-install libxcb-devel``
   3. On Arch linux you can install with this command ``sudo pacman -S libxcb``
6. ```cargo build --release```
7. ```cd target/release```
8. DONE! You can run it

## Screenshots

![bc](/resources/screenshot1.png)
![bc](/resources/screenshot2.png)

## Notice

This application is not intended to facilitate the unauthorized download or sharing of music. Its sole purpose is to provide users with an easier way to stream and play music from the site, and to help them discover new and exciting tracks to enjoy. We do not condone or support piracy in any form, and we encourage all users to respect the intellectual property rights of artists and content creators.