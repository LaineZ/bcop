let selectedTags = [];
let player = new Player();
let loading = new LoadingIndicator();
let console = new Console();

let visualizer = new Visualizer(300, 64);

let windowSize;

loading.spawn();

//const genericModal = new Modal("generic-modal");

// MODALS
const clearQueueModal = new Modal("clear-queue-modal");
const optionsModal = new Modal("options-modal");
const albumImportModal = new Modal("album-import-modal");

// VIEWS
const homeView = new View("home-window", "home");
const discoverView = new View("discover-window", "discover");
const nowPlayingView = new View("now-playing-window", "now-playing");

homeView.show();


const clamp = (num, min, max = Number.MAX_SAFE_INTEGER) => Math.min(Math.max(num, min), max);


function update() {
    visualizer.update();
}

optionsModal.modalWindow.addEventListener("closed", (_) => {
    setSettings();
    saveSettings();
});

albumImportModal.modalWindow.addEventListener("closed", (_) => {
    $('#search-results').empty();
});

albumImportModal.modalWindow.addEventListener("open", (_) => {
    if ($("#album-url-input").text()) {
        searchRequest();
    }
});

window.addEventListener('load', function () {
    player.setup();
});

function setupSizeVars() {
    let [w, h] = Window.this.box("dimension");
    let ws = "normal";
    if (w < 1000)
        ws = "small";
    if (windowSize !== ws) {
        Window.this.mediaVars({ "window-size": windowSize = ws });
    }
}

Window.this.on("size", setupSizeVars);
setupSizeVars();

function showErrorModal(message) {
    $("body").append(`<div class="error-modal">${message}</div>`);
    setTimeout(function () {
        $(".error-modal").each(function () {
            $(this)[0].classList.add("closing");
        });
    }, clamp(message.length * 18, 1000));

    setTimeout(function () {
        $(".error-modal").eq(0).remove();
    }, clamp(message.length * 20, 2000));
    loading.destroy();
}

function setTheme(theme) {
    if (themes[theme]) {
        for (const [key, value] of Object.entries(themes[theme])) {
            document.style.variable(key, value);
        }
    } else {
        logWarn("Unable to find theme: " + theme + " reverting to hope_diamond...");
        setTheme("hope_diamond");
    }
}

function createElementFromHTML(html) {
    const placeholder = document.createElement("div");
    placeholder.insertAdjacentHTML("afterbegin", html);
    return placeholder.firstElementChild;
}

setInterval(function () {
    player.updatePlayerInformation();
}, 100);

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

function closeModals() {
    //genericModal.hide();
    clearQueueModal.hide();
    optionsModal.hide();
    albumImportModal.hide();
}

function closeViews() {
    homeView.hide();
    discoverView.hide();
    nowPlayingView.hide();
}

// HANDLERS

$(".no").on("click", function () {
    closeModals();
});

$("#clear-queue-yes").on("click", function () {
    player.clearQueue();
    closeModals();
});

$("#search-results").on("click", ".track-card", function () {
    var value = $(this).prop("value");
    player.addToQueue(value);
    closeModals();
});

document.on("click", function (evt) {
    if (evt.target.id == "modal-dim" || evt.target.id == "album-import-modal" || evt.target.id == "search-results") {
        closeModals();
        return true;
    }
    return false;
});

$(window).on("click", "#discover-queue-menu", "li", function (e) {
    const idx = $(e.target).index();
    const index = $(e.source.parentElement).index();

    // remove track
    if (idx == 0) {
        player.removeTrackAt(index);
    }
    // open track url in browser
    if (idx == 1) {
        openInBrowser(player.queue[index].title_link);
    }
    // copy url
    if (idx == 2) {
        setClipboard(player.queue[index].title_link);
    }
    // revoke track url
    if (idx == 3) {
        player.revokeAudioUrlForAll();
    }
});

$("#queue-select").on("click", ".track-card", function () {
    var idx = $(this).index();
    player.queuePosition = idx;
    player.loadTrack();
});


$("#track-name").on("click", function () {
    log($(".track-card.current").offset());
});

$('#play-pause').on("click", function () {
    if (player.isPaused()) {
        player.setPaused(false);
    } else {
        player.setPaused(true);
    }
});

$('#github').on("click", function () {
    openInBrowser("https://github.com/lainez/bc_rs")
});

$('#sciter-link').on("click", function () {
    openInBrowser("https://sciter.com")
});

$('#bass-link').on("click", function () {
    openInBrowser("http://www.un4seen.com")
});

$('#back').on("click", function () {
    player.previous();
});

$('#forward').on("click", function () {
    player.next();
});

$('#stop').on("click", function () {
    player.stop();
});

$('#clear-queue').on('click', function () {
    clearQueueModal.show();
});

$('#close-settings').on('click', function () {
    closeModals();
});

$('#settings').on('click', function () {
    optionsModal.show();
});

$('#player-queue-toggle').on('click', function () {
    const tags = document.getElementById("queue-select");
    if (tags.classList.contains("closed")) {
        tags.classList.remove("closed");
    } else {
        tags.classList.add("closed");
    }
});

$('#album-import').on('click', function () {
    albumImportModal.show();
});

$('#home').on('click', function () {
    closeViews();
    homeView.show();
});

$('#discover').on('click', function () {
    closeViews();
    discoverView.show();
});

$('#now-playing').on('click', function () {
    closeViews();
    nowPlayingView.show();
});



$('#theme').on('change', function () {
    setTheme(this.value);
});

$('#audio-backend').on('change', function () {
    player.switchBackend(parseInt(this.value));
});

const searchRequest = debounce(function () {
    var text = $('#album-url-input').text();

    $('#search-results').empty();

    loading.spawn();

    httpRequestGet("https://bandcamp.com/api/fuzzysearch/1/autocomplete?q=" + text, function (response) {
        var json = JSON.parse(response);
        if (json.auto && json.auto.results) {
            json.auto.results.forEach(element => {
                if (element.type == "a" || element.type == "t") {
                    const card = createElementFromHTML(
                        searchResultCard(element.name, element.band_name, element.url));

                    $(card).children(function () {
                        if ($(this).prop("className") == "search-img") {
                            setImage(element.art_id, $(this)[0]);
                        }
                    });

                    $('#search-results').append(card);
                }
            });
        }
        loading.destroy();
    }, showErrorModal);
}, 500);

$('#album-url-input').on('input', function () {
    var text = $('#album-url-input').text();
    if ((text.startsWith("https://") || text.startsWith("http://")) && text.includes("bandcamp.com")) {
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
            player.addToQueue(text);
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

$('#volume').on('input', function (e) {
    player.setVolume($(this).val());
});

$(document).keyup(function (e) {
    logDebug(e.keyCode);

    if ($(e.target).closest("input")[0]) {
        return;
    }

    if (debugMode && e.keyCode == keys.F5) {
        Window.this.load(location.href);
    }

    if (e.keyCode == keys.KEY_GRAVE) {
        console.toggle();
    }

    if (e.keyCode == keys.KEY_F) {
        albumImportModal.show();
    }

    if (e.keyCode == keys.ESCAPE) {
        setSettings();
        saveSettings();
        closeModals();
    }
});

$('#seekbar').on('input', function (e) {
    player.seek($(this).val());
    player.updatePlayerInformation();
});

document.on("closerequest", function (evt) {
    if (Window.this.xcall("get_save_queue_on_exit")) {
        player.saveQueue();
    } else {
        deleteFile("queue.json");
    }
    setGeometry();
});