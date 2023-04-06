/** Player and queue managment */
class Player {
    constructor() {
        this.queue = [];
        this.queuePosition = 0;
        this.shuffle = false;
    }

    /** Setups all required player callbacks and handlers. 
     * Should be called after creation player instance */
    setup() {
        var me = this;

        this.setStateChangeCallback(function () {
            if (me.queue.length == 0) {
                $('#queue-select')[0].classList.add("closed");
                $('#track-name').text("");
            } else {
                $('#queue-select')[0].classList.remove("closed");
                $('#track-name').text(me.queue[me.queuePosition].artist + " - " + me.queue[me.queuePosition].title);
            }

            $('#controls').each(function () {
                $(this).attr("disabled", me.queue.length == 0);
            });

            $("#seekbar").attr("disabled", me.queue.length == 0);

            if (me.isPaused()) {
                $('#play-pause').attr("src", "icons/play.svg");
            } else {
                $('#play-pause').attr("src", "icons/pause.svg");
            }

            $(".track-card").each(function (index) {
                if (index == me.queuePosition) {
                    $(this).attr("class", "track-card current");
                } else {
                    $(this).attr("class", "track-card");
                }
            });

            Window.this.xcall("restart_player_on_fault");
        });

        this.forceUpdate();

        // restore play queue
        const file = readFile("queue.json");

        if (file && file.length != 0) {
            const jsonRes = JSON.parse(file);
            if (jsonRes.queue.length > 0) {
                jsonRes.queue.forEach(element => {
                    this.#addToQueueInternal(element);
                });

                this.queuePosition = jsonRes.position;
                this.loadTrack();
                this.seek(jsonRes.play_position);
                this.setPaused(true);
            }
        } else {
            log("Unable to find queue cache file");
        }

        this.setVolume($("#volume").val());
    }

    switchBackend(to) {
        return Window.this.xcall("switch_backend", to);
    }

    previous() {
        if (player.queuePosition > 0) {
            player.queuePosition -= 1;
            player.loadTrack();
        } else {
            seek(0);
        }
    }

    next() {
        const result = player.queuePosition < player.queue.length - 1;
        if (result) {
            player.queuePosition += 1;
            player.loadTrack();
        }

        return result;
    }

    /** Saves all queue contents and position to disk */
    saveQueue(filename = "queue.json") {
        writeFile(filename, JSON.stringify({
            queue: this.queue,
            position: this.queuePosition,
            play_position: this.getTime()
        }))
    }

    #loadTrackInternal() {
        const useHttp = Window.this.xcall("request_http");
        const url = this.queue[this.queuePosition].file["mp3-128"];

        if (useHttp) {
            url.replace("https://", "http://");
        }

        return Window.this.xcall("load_track", url);
    }

    revokeAudioUrlForAll() {
        var me = this;

        me.queue.forEach(element => {
            loading.spawn();
            httpRequestGet(element.title_link, function (response) {
                const aldata = parseAlbumData(response);
                if (aldata) {
                    const jsonRes = JSON.parse(aldata);
                    element.file["mp3-128"] = jsonRes.trackinfo[0].file["mp3-128"];
                }
                loading.destroy();
            });
        });
    }

    loadTrack() {
        var me = this;

        if (!me.#loadTrackInternal()) {
            // probably needs revoke track URL
            loading.spawn();
            httpRequestGet(this.queue[this.queuePosition].title_link, function (response) {
                const aldata = parseAlbumData(response);
                if (aldata) {
                    const jsonRes = JSON.parse(aldata);
                    me.queue[me.queuePosition].file["mp3-128"] = jsonRes.trackinfo[0].file["mp3-128"];
                    me.#loadTrackInternal()
                }
                loading.destroy();
            });
        }
    }

    setPaused(paused) {
        Window.this.xcall("set_paused", paused);
    }

    isPaused() {
        return Window.this.xcall("is_paused");
    }

    stop() {
        Window.this.xcall("stop");
    }

    seek(secs) {
        Window.this.xcall("seek", secs);
    }

    getTime() {
        return Window.this.xcall("get_time");
    }

    fmtTime(time) {
        return Window.this.xcall("fmt_time", time);
    }

    getVolume() {
        return Window.this.xcall("get_volume");
    }

    setVolume(value) {
        return Window.this.xcall("set_volume", value);
    }

    setStateChangeCallback(callback) {
        Window.this.xcall("set_state_change_callback", callback);
    }

    forceUpdate() {
        Window.this.xcall("force_update");
        this.updatePlayerInformation();
    }

    updatePlayerInformation() {
        if (this.queue.length > 0) {
            $('#current-time').text(this.fmtTime(this.getTime()));
            $('#total-time').text(this.fmtTime(Math.floor(this.queue[this.queuePosition].duration)));
            $('#seekbar').val(this.getTime());
            $('#seekbar').attr('max', Math.floor(this.queue[this.queuePosition].duration));

            // load next track
            if (this.queue.length > this.queuePosition + 1) {
                if (this.getTime() >= Math.floor(this.queue[this.queuePosition].duration)) {
                    this.queuePosition += 1;
                    this.loadTrack();
                }
            }
        }
    }

    clearQueue() {
        this.stop();
        this.queue = [];
        this.queuePosition = 0;
        $('#queue-select').empty();
        this.forceUpdate();
    }

    #addToQueueInternal(element) {
        this.queue.push(element);
        //log(element);

        const node = createElementFromHTML(queuedTrackCard(element.title, element.artist));

        $(node).children(function () {
            if ($(this).prop("className") == "track-img") {
                setImage(element.art_id, $(this)[0]);
            }
        });

        $("#queue-select").append(node);
    }

    addToQueue(url) {
        var me = this;

        // ["https:","","thealgorithm.bandcamp.com","album","brute-force"]
        //              ^
        const artistPage = url.split("/")[2];

        loading.spawn();
        httpRequestGet(url, function (response) {
            const aldata = parseAlbumData(response);
            if (aldata) {
                const jsonRes = JSON.parse(aldata);
                jsonRes.trackinfo.forEach(element => {
                    element.artist = jsonRes.artist;
                    element.art_id = jsonRes.art_id;
                    element.title_link = "https://" + artistPage + element.title_link;

                    if (element.file != null) {
                        me.#addToQueueInternal(element);
                    }
                });
            }

            me.forceUpdate();
            loading.destroy();
        }, showErrorModal);
    }

    removeTrackAt(index) {
        if (index == this.queuePosition - 1) {
            this.stop();
        }

        var idx = 0;
        $("#queue-select").children(function () {
            if (idx == index) {
                $(this).remove();
            }
            idx += 1;
        });

        this.queue.splice(index, 1);
    }
}