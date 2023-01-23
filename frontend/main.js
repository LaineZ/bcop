let discover = [];
let queue = [];
let selectedTags = [];

let page = 1;
let queuePosition = 0;

let intervalUpdate = setInterval(updatePlayerInformation, 500);


const tagSelector = document.getElementById("tags-select");
const loading = document.getElementById("loading");

loading.style.display = "block";
getTags(function (done) {
    done.split("\n").forEach(element => {
        tagSelector.innerHTML += `<option class="tag">${element}</option>`
    });
    loading.style.display = "none";
});

function debounce(func, wait, immediate) {
    let timeout;

    return function executedFunction() {
        const context = this;
        const args = arguments;

        const later = function () {
            timeout = null;
            if (!immediate) func.apply(context, args);
        };

        const callNow = immediate && !timeout;

        clearTimeout(timeout);

        timeout = setTimeout(later, wait);

        if (callNow) func.apply(context, args);
    };
};


function updatePlayerInformation() {
    if (queue.length > 0) {
        $('#current-time').text(fmtTime(getTime()));
        $('#total-time').text(fmtTime(Math.floor(queue[queuePosition].duration)));
        $('#seekbar').val(getTime());
        $('#seekbar').attr('max', Math.floor(queue[queuePosition].duration));

        if (queue.length > queuePosition + 1) {
            if (getTime() >= queue[queuePosition].duration) {
                queuePosition += 1;
                loadTrack(queue[queuePosition].file["mp3-128"]);
            }
        } else {
            stop();
        }
    }
}

function extendDiscover(tags, page) {
    const json = {
        filters: {
            format: "all",
            location: 0,
            sort: "pop",
            tags: tags
        },
        page: page
    };

    loading.style.display = "block";
    httpRequestPost("https://bandcamp.com/api/hub/2/dig_deeper", JSON.stringify(json), function (response) {
        const jsonRes = JSON.parse(response);
        jsonRes.items.forEach(element => {
            discover.push(element.tralbum_url);
            $('#albums-select')
                .append(discoverAlbumCard(element.art_id, element.title, element.artist, element.genre));

            var card = document.getElementById("art_" + element.art_id);
            setImage(element.art_id, card);
        });
        loading.style.display = "none";
    });
}

function showQueueClearModal() {
    const modalDim = document.getElementById("modal-dim");
    const modalWindow = document.getElementById("clear-queue-modal");
    modalDim.classList.add("active");
    modalWindow.style.display = "block";
}

function showOptionsModal() {
    const modalDim = document.getElementById("modal-dim");
    const modalWindow = document.getElementById("options-modal");
    modalDim.classList.add("active");
    modalWindow.style.display = "block";

    // load settings values into view
    populateSettings();
}

function showAlbumImport() {
    const modalDim = document.getElementById("modal-dim");
    const modalWindow = document.getElementById("album-import-modal");

    if ($("#album-url-input").text()) {
        searchRequest();
    }

    modalDim.classList.add("active");
    modalWindow.style.display = "block";
}

function closeModals() {
    const modalDim = document.getElementById("modal-dim");

    $('#search-results').empty();

    $(".modal-content").each(function () {
        $(this)[0].style.display = "none";
    });

    modalDim.classList.add("closing");

    setTimeout(function() {
        modalDim.classList.remove("active");
        modalDim.classList.remove("closing");
    }, 200); 
}

function clearDiscover() {
    page = 1;
    discover = [];
    $('#albums-select').empty();
}

function clearQueue() {
    stop();
    queue = [];
    queuePosition = 0;
    $('#queue-select').empty();
}

function addToQueue(url) {
    const loading = document.getElementById("loading");
    loading.style.display = "block";

    httpRequestGet(url, function (response) {
        const aldata = parseAlbumData(response);

        if (aldata) {
            const jsonRes = JSON.parse(aldata);
            jsonRes.trackinfo.forEach(element => {
                if (element.file != null) {
                    queue.push(element);
                    $('#queue-select').append(
                        queuedTrackCard(jsonRes.current.art_id, element.title, jsonRes.artist));
                }
            });

            document.getElementsByClassName("queue_art_" + jsonRes.current.art_id).forEach(card => {
                setImage(jsonRes.current.art_id, card);
            });
        }

        loading.style.display = "none";
    });
}

// HANDLERS

$(".no").on("click", function () {
    closeModals();
});

$("#clear-queue-yes").on("click", function () {
    clearQueue();
    closeModals();
});

$("#tags-select").on("click", function () {
    clearDiscover();
    selectedTags = [$(this).val()];
    $('#discover-heading').text(selectedTags);
    for (let index = 0; index < 3; index++) {
        page += 1;
        extendDiscover(selectedTags, page)
    }
});

$("#albums-select").on("click", ".album-card", function () {
    var idx = $(this).index();
    addToQueue(discover[idx]);
});

$("#albums-select").on("click", ".album-card", function () {
    var idx = $(this).index();
    addToQueue(discover[idx]);
});

$("#search-results").on("click", ".track-card", function () {
    var value = $(this).prop("value");
    addToQueue(value);
    closeModals();
});

document.on("contextmenu", function (evt) {
    // handle discover context menu
    log(evt.target.className);
    if (evt.target.className == "album-card" || evt.target.parentElement.className == "album-card") {
        evt.source = Element.create(<DiscoverContextMenu />);
        return true;
    }

    if (evt.target.className.includes("track-card") || evt.target.parentElement.className.includes("track-card")) {
        evt.source = Element.create(<QueueContextMenu />);
        return true;
    }

    return false;
});

// workaround for context menus
$(window).on("click", "#discover-context-menu", "li", function (e) {
    const idx = $(e.target).index();

    // add to queue
    if (idx == 0) {
        const index = $(e.source.parentElement).index();
        addToQueue(discover[index]);
    }

    // open in browser
    if (idx == 1) {
        const index = $(e.source.parentElement).index();
        openInBrowser(discover[index]);
    }
});

$(window).on("click", "#discover-queue-menu", "li", function (e) {
    const idx = $(e.target).index();

    // remove track
    if (idx == 0) {
        const index = $(e.source.parentElement).index();

        log($(e.source.parentElement)[0]);


        if (index == queuePosition) {
            stop();
        }


        $(e.source.parentElement).eq(index).remove();
        queue.splice(index, 1);
    }
});

$("#queue-select").on("click", ".track-card", function () {
    var idx = $(this).index();
    queuePosition = idx;
    loadTrack(queue[queuePosition].file["mp3-128"]);
});

$('#clear-discover-btn').on("click", function () {
    clearDiscover()
});

$('#play-pause').on("click", function () {
    if (isPaused()) {
        setPaused(false);
    } else {
        setPaused(true);
    }
});

$('#github').on("click", function () {
    openInBrowser("https://github.com/lainez/bc_rs")
})

$('#sciter-link').on("click", function () {
    openInBrowser("https://sciter.com")
})

$('#back').on("click", function () {
    if (queuePosition > 0) {
        queuePosition -= 1;
        loadTrack(queue[queuePosition].file["mp3-128"]);
    } else {
        seek(0);
        log("error");
        const nativeElement = $("#error-modal")[0];
        nativeElement.style.animation = "500ms slide alternate";
        $("#error-modal").attr("class", "show");
        setTimeout(function () {
            void nativeElement.offsetWidth;
            nativeElement.style.animation = "";
            $("#error-modal").attr("class", "");
        }, 1000);
    }
});

$('#forward').on("click", function () {
    if (queuePosition < queue.length - 1) {
        queuePosition += 1;
        loadTrack(queue[queuePosition].file["mp3-128"]);
    }
});

$('#stop').on("click", function () {
    stop();
});

$('#clear-queue').on('click', function () {
    showQueueClearModal();
});

$('#close-settings').on('click', function () {
    setSettings();
    saveSettings();
    closeModals();
});

$('#settings').on('click', function () {
    showOptionsModal();
});

$('#tags-toggle').on('click', function () {
    const tags = document.getElementById("tags-select");
    if (tags.classList.contains("closed")) {
        tags.classList.remove("closed");
    } else {
        tags.classList.add("closed");
    }
});

$('#album-import').on('click', function () {
    showAlbumImport();
});

const searchRequest = debounce(function() {
    var text = $('#album-url-input').text();

    $('#search-results').empty();

    httpRequestGet("https://bandcamp.com/api/fuzzysearch/1/autocomplete?q=" + text, function (response) {
        var json = JSON.parse(response);
        if (json.auto && json.auto.results) {
            json.auto.results.forEach(element => {
                if (element.type == "a" || element.type == "t") {
                    $('#search-results').append(searchResultCard(element.art_id, element.name, element.band_name, element.url));

                    var card = document.getElementById("search_art_" + element.art_id);
                    setImage(element.art_id, card);
                }
            });
        }
    });
}, 500);

$('#album-url-input').on('input', function () {
    var text = $('#album-url-input').text();
    if (text.startsWith("https://") && text.includes("bandcamp.com")) {
        $('#search-results')
            .html("<h2>Press Enter to load album/track into queue</h2>");
    } else {
        searchRequest();
    }
});

$('#album-url-input').keyup(function (e) {
    if (e.keyCode == keys.ENTER) {
        var text = $('#album-url-input').text();
        if (text.startsWith("https://") && text.includes("bandcamp.com")) {
            addToQueue(text);
            closeModals();
        }
    }

    if (e.keyCode == keys.ESCAPE) {
        closeModals();
    }
});

$(".option-tab").click(function () {
    var selectedIndex = $(this).index();
    $(".option-tab").each(function (index) {
        if (index == selectedIndex) {
            $(".option-tab").eq(index).attr("class", "option-tab selected");
            $(".option-selection").eq(index).attr("class", "option-selection");
        } else {
            $(".option-tab").eq(index).attr("class", "option-tab");
            $(".option-selection").eq(index).attr("class", "hidden option-selection");
        }
    });
});

$('#albums-select').scroll(function () {
    const scrollHeight = $('#albums-select').prop('scrollHeight');
    const scrollTop = $('#albums-select').prop('scrollTop');
    const clientHeight = $('#albums-select').height();

    if (Math.abs(scrollHeight - clientHeight - scrollTop) < 1) {
        page += 1;
        extendDiscover(selectedTags, page)
    }
});

$('#volume').on('input', function (e) {
    setVolume($(this).val());
});

$('#volume').on('input', function (e) {
    setVolume($(this).val());
});

$('#discover-heading').keyup(function (e) {
    if (e.keyCode == keys.ENTER) {
        clearDiscover();
        extendDiscover($(this).val().split(" "), page);
    }
});

$(document).keyup(function (e) {
    if ($(e.target).closest("input")[0]) {
        return;
    }

    if (e.keyCode == keys.KEY_S) {
        $("#discover-heading")[0].focus();
    }

    if (e.keyCode == keys.ESCAPE) {
        closeModals();
    }
});

$('#seekbar').on('input', function (e) {
    seek($(this).val());
    updatePlayerInformation();
});

setStateChangeCallback(function () {
    if (isPaused()) {
        $('#play-pause').attr("src", "icons/play.svg");
    } else {
        $('#play-pause').attr("src", "icons/pause.svg");
    }

    $(".track-card").each(function (index) {
        if (index == queuePosition) {
            $(this).attr("class", "track-card current");
        } else {
            $(this).attr("class", "track-card");
        }
    });
});