function log(msg) {
    Window.this.xcall("log_info", msg);
}

function logDebug(msg) {
    Window.this.xcall("log_debug", msg);
}

function httpRequestGet(url, done_callback) {
    Window.this.xcall("http_request_get", url, done_callback);
}

function httpRequestPost(url, body, done_callback) {
    Window.this.xcall("http_request_post", url, body, done_callback);
}

function getTags(done_callback) {
    Window.this.xcall("get_tags", done_callback);
}

function populateSettings() {
    const settings = document.getElementById("options-modal");
    Window.this.xcall("populate_settings", settings);
}

function setSettings() {
    const settings = document.getElementById("options-modal");
    Window.this.xcall("set_settings", settings);
}

function setImage(art_id, image) {
    const quality = Window.this.xcall("get_load_artworks");
    const proxy = Window.this.xcall("get_proxy");

    if (proxy == proxyType.NONE) {
        Window.this.xcall("set_image",
        "https://f4.bcbits.com/img/a" + art_id + "_" + quality + ".jpg",
        image, false);
    }

    if (proxy == proxyType.USE_HTTP) {
        Window.this.xcall("set_image",
        "http://f4.bcbits.com/img/a" + art_id + "_" + quality  + ".jpg",
        image, false);
    }

    if (proxy == proxyType.USE_PROXY) {
        Window.this.xcall("set_image",
        "http://f4.bcbits.com/img/a" + art_id + "_" + quality + ".jpg",
        image, true);
    }
}

function setStateChangeCallback(callback) {
    Window.this.xcall("set_state_change_callback", callback);
}

function loadTrack(url) {
    Window.this.xcall("load_track", url);
}

function setPaused(paused) {
    Window.this.xcall("set_paused", paused);
}

function isPaused() {
    return Window.this.xcall("is_paused");
}

function stop() {
    Window.this.xcall("stop");
}

function seek(secs) {
    Window.this.xcall("seek", secs);
}

function getTime() {
    return Window.this.xcall("get_time");
}

function fmtTime(time) {
    return Window.this.xcall("fmt_time", time);
}

function parseAlbumData(html) {
    return Window.this.xcall("parse_album_data", html);
}

function getVolume() {
    return Window.this.xcall("get_volume");
}

function setVolume(value) {
    return Window.this.xcall("set_volume", value);
}

function openInBrowser(url) {
    return Window.this.xcall("open_in_browser", url);
}

function getSettings() {
    return {
        loadArtworks: Window.this.xcall("get_load_artworks"),
        proxyType: Window.this.xcall("get_proxy"),
    }
}

function saveSettings() {
    Window.this.xcall("save_config");
}