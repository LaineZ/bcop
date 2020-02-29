# A next crossplatform version of [BandcampOnlinePlayer](https://github.com/LaineZ/BandcampOnlinePlayer) written on Rust!
**IN DEVELOPMENT!**
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

``vol [number: float]`` - sets volume (default: 1.0) CAUTION: values above 1.0 causes a serious clipping!

``seek [serconds: number]`` - sets track position (in seconds)

## Building
1. [Download Rust]([https://www.rust-lang.org/learn/get-started) and run this command
2. ```git clone https://github.com/LaineZ/bc_rs.git```
3. ```cd bc_rs```
4. ```cargo build``` also you can add ``--release`` option to reduce binary file size
5. ```cd target/debug```
6. DONE! You can run in CLI Mode by instructions above