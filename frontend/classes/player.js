/** Player and queue managment */
class Player {
    constructor() {
        this.queue = [];
        this.queuePosition = 0;
    }

    /** Setups all required player callbacks and handlers. 
     * Should be called after creation player instance */
    setup() {
        var me = this;

        this.setVolume($("#volume").val());

        this.setStateChangeCallback(function () {
            $('#controls').each(function () {
                $(this).attr("disabled", me.queue.length <= 0);
            });

            $("#seekbar").attr("disabled", me.queue.length <= 0);

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
        });
    }

    loadTrack() {
        Window.this.xcall("load_track", this.queue[this.queuePosition].file["mp3-128"]);
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

    updatePlayerInformation() {
        if (this.queue.length > 0) {
            $('#current-time').text(this.fmtTime(this.getTime()));
            $('#total-time').text(this.fmtTime(Math.floor(this.queue[this.queuePosition].duration)));
            $('#seekbar').val(this.getTime());
            $('#seekbar').attr('max', Math.floor(this.queue[this.queuePosition].duration));
            $('#track-name').text(this.queue[this.queuePosition].artist + " - " + this.queue[this.queuePosition].title);

            // load next track
            if (this.queue.length > this.queuePosition + 1) {
                if (this.getTime() >= this.queue[this.queuePosition].duration) {
                    this.queuePosition += 1;
                    this.loadTrack( );
                }
            } else {
                this.stop();
            }
        } else {
            $('#controls').each(function () {
                $(this).attr("disabled", true);
            });
        }
    }

    clearQueue() {
        this.stop();
        this.queue = [];
        this.queuePosition = 0;
        $('#queue-select').empty();
    }

    addToQueue(url) {
        const loading = document.getElementById("loading");
        loading.style.display = "block";

        var me = this;

        httpRequestGet(url, function (response) {
            const aldata = parseAlbumData(response);
            if (aldata) {
                const jsonRes = JSON.parse(aldata);
                jsonRes.trackinfo.forEach(element => {
                    if (element.file != null) {
                        element.artist = jsonRes.artist;
                        element.art_id = jsonRes.art_id;
                        me.queue.push(element);
                        log(element);
                    }
                });
            }

            me.renderQueue();
            loading.style.display = "none";
        });
    }

    renderQueue() {
        const queueSelector = $('#queue-select');

        queueSelector.empty();
        this.queue.forEach(element => {
            const node = createElementFromHTML(queuedTrackCard(element.title, element.artist));

            $(node).children(function() {
                if ($(this).prop("className") == "track-img") {
                    setImage(element.art_id, $(this)[0]);
                }
            });

            queueSelector.append(node);
        });
    }

    removeTrackAt(index) {
        if (index == queuePosition) {
            stop();
        }

        this.queue.splice(index, 1);
        this.renderQueue();
    }
} 