# BandcampOnlinePlayer Rust Edition
**IN DEVELOPMENT!**

![Create Release](https://github.com/LaineZ/bc_rs/workflows/Create%20Release/badge.svg?event=push)

A next crossplatform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written in Rust!

You can download latest version from "actions" menu

## Stream mode
Сurrently, the player only supports **CLI Stream mode** to play in CLI stream run command:
### Linux/Mac
```./bandcamp-online-cli stream [tag]```

**Example:**

``./bandcamp-online-cli stream metal`` - plays a metal tag

### Windows

``bandcamp-online-cli.exe stream [tag]``

**Example:** ``bandcamp-online-cli.exe stream metal`` - plays a metal tag

### CLI Commands in runtime

Also, at playback you can use some commands to control playback
``c``/``exit`` - closes program

``p`` - play/pause

``next`` - next track

``vol [number: float]`` - sets volume (default: 1.0) CAUTION: values above 1.0 causes a serious clipping, also - negative values inverts track phase

``seek [serconds: number]`` - sets track position (in seconds)

``switchadvanced`` - switches to new playback system (enabled by default) thats playback system fast but can have short-time memory peaks around 150-180 mb

``switchsimple`` - switch to old-patched playback system this playback system is clean in terms of memory but may have some perfomance issues

## Building
1. [Download Rust]([https://www.rust-lang.org/learn/get-started) and run this command
2. ```git clone https://github.com/LaineZ/bc_rs.git```
3. ```cd bc_rs```
4. If you run on Linux you need install these audio libs: ``libaudio`` and ``libasound2``
   1. On Ubuntu you can just run that command ``sudo apt install libaudio-dev libasound2-dev``
5. ```cargo build``` also you can add ``--release`` option to reduce binary file size and speed up tracks operations
6. ```cd target/debug```
7. DONE! You can run in CLI Mode by instructions above
