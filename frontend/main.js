let selectedTags = [];
let player = new Player();
let discover = new Discover();

const tagSelector = document.getElementById("tags-select");
const loading = document.getElementById("loading");

loading.style.display = "block";

getTags(function (done) {
    done.split("\n").forEach(element => {
        tagSelector.innerHTML += `<option class="tag">${element}</option>`
    });
    loading.style.display = "none";
});

player.setup();

setupVolume($("#volume")[0]);

setInterval(function () {
    player.updatePlayerInformation();
}, 500);

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

    setTimeout(function () {
        modalDim.classList.remove("active");
        modalDim.classList.remove("closing");
    }, 200);
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
    discover.clearDiscover();
    selectedTags = [$(this).val()];
    $('#discover-heading').text(selectedTags);
    for (let index = 0; index < 3; index++) {
        discover.extend(selectedTags)
    }
});

$("#albums-select").on("click", ".album-card", function () {
    var idx = $(this).index();
    player.addToQueue(discover.discover[idx]);
});

$("#albums-select").on("click", ".album-card", function () {
    var idx = $(this).index();
    player.addToQueue(discover.discover[idx]);
});

$("#search-results").on("click", ".track-card", function () {
    var value = $(this).prop("value");
    player.addToQueue(value);
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
        player.addToQueue(discover.discover[index]);
    }

    // open in browser
    if (idx == 1) {
        const index = $(e.source.parentElement).index();
        openInBrowser(discover.discover[index]);
    }
});

$(window).on("click", "#discover-queue-menu", "li", function (e) {
    const idx = $(e.target).index();

    // remove track
    if (idx == 0) {
        const index = $(e.source.parentElement).index();
        player.removeTrackAt(index);
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
})

$('#sciter-link').on("click", function () {
    openInBrowser("https://sciter.com")
})

$('#back').on("click", function () {
    if (player.queuePosition > 0) {
        player.queuePosition -= 1;
        player.loadTrack();
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
        player.loadTrack();
    }
});

$('#stop').on("click", function () {
    player.stop();
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

const searchRequest = debounce(function () {
    var text = $('#album-url-input').text();

    $('#search-results').empty();

    httpRequestGet("https://bandcamp.com/api/fuzzysearch/1/autocomplete?q=" + text, function (response) {
        var json = JSON.parse(response);
        if (json.auto && json.auto.results) {
            json.auto.results.forEach(element => {
                if (element.type == "a" || element.type == "t") {
                    $('#search-results').append(
                        searchResultCard(element.art_id, element.name, element.band_name, element.url));

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

$('#albums-select').scroll(function () {
    const scrollHeight = $('#albums-select').prop('scrollHeight');
    const scrollTop = $('#albums-select').prop('scrollTop');
    const clientHeight = $('#albums-select').height();

    if (Math.abs(scrollHeight - clientHeight - scrollTop) < 1) {
        discover.extend(selectedTags)
    }
});

$('#volume').on('input', function (e) {
    player.setVolume($(this).val());
    storeVolume($("#volume")[0]);
});

$('#discover-heading').keyup(function (e) {
    if (e.keyCode == keys.ENTER) {
        discover.clearDiscover();
        discover.extend($(this).val().split(" "));
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
    player.seek($(this).val());
    player.updatePlayerInformation();
});

document.on("closerequest", function (evt) {
    saveSettings()
});