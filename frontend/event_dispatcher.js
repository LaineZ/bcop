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

function parseAlbumData(html) {
    return Window.this.xcall("parse_album_data", html);
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