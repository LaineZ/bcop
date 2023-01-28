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
            if (me.queue.length <= 0) {
                $('#queue-select')[0].classList.add("closed");
                $('#track-name').text("");
            } else {
                $('#queue-select')[0].classList.remove("closed");
                $('#track-name').text(me.queue[me.queuePosition].artist + " - " + me.queue[me.queuePosition].title);
            }

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

        this.forceUpdate();
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

    forceUpdate() {
        Window.this.xcall("force_update");
    }

    updatePlayerInformation() {
        if (this.queue.length > 0) {
            $('#current-time').text(this.fmtTime(this.getTime()));
            $('#total-time').text(this.fmtTime(Math.floor(this.queue[this.queuePosition].duration)));
            $('#seekbar').val(this.getTime());
            $('#seekbar').attr('max', Math.floor(this.queue[this.queuePosition].duration));

            // load next track
            if (this.queue.length > this.queuePosition + 1) {
                if (this.getTime() >= this.queue[this.queuePosition].duration) {
                    this.queuePosition += 1;
                    this.loadTrack( );
                }
            } else {
                this.stop();
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

    addToQueue(url) {
        var me = this;

        loading.spawn();
        httpRequestGet(url, function (response) {
            const aldata = parseAlbumData(response);
            if (aldata) {
                const jsonRes = JSON.parse(aldata);
                jsonRes.trackinfo.forEach(element => {
                    if (element.file != null) {
                        element.artist = jsonRes.artist;
                        element.art_id = jsonRes.art_id;
                        me.queue.push(element);
                        //log(element);

                        const node = createElementFromHTML(queuedTrackCard(element.title, element.artist));

                        $(node).children(function() {
                            if ($(this).prop("className") == "track-img") {
                                setImage(element.art_id, $(this)[0]);
                            }
                        });
            
                        $("#queue-select").append(node);
                    }
                });
            }

            me.forceUpdate();
            loading.destroy();
        });
    }

    removeTrackAt(index) {
        if (index == this.queuePosition) {
            this.stop();
        }


        var idx = 0;
        $("#queue-select").children(function() {
            if (idx == index) {
                $(this).remove();
            }
            idx += 1;
        });

        this.queue.splice(index, 1);
    }
} 