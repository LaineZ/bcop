function queuedTrackCard(art_id, title, artist) {
    return `<div class="track-card">
        <img class="queue_art_${art_id}">
        <div class="track-card-desc">
            <h3>${title}</h3>
            <p>${artist}</p>
        </div>
    </div>`;
}

function discoverAlbumCard(art_id, title, artist, genre) {
    return `<div class="album-card">
    <img class=album-image id="art_${art_id}">
        <div class="album-description">
            <h3 title="${title}">${title}</h3>
            <p>${artist} × ${genre}</p>
        </div>
    </div>`;
}

function searchResultCard(art_id, title, artist, url) {
    return `<div class="track-card" value="${url}">
    <img class="search-img" id="search_art_${art_id}" src="icons/audio.jpg">
    <div class="track-card-desc">
        <h3>${title}</h3>
        <p>by <strong>${artist}</strong></p>
    </div>
</div>`;
} 

function DiscoverContextMenu() {
    return <menu.context id="discover-context-menu">
        <li>Add to queue</li>
        <li>Open album page in browser</li>
    </menu>;
}

function QueueContextMenu() {
    return <menu.context id="discover-queue-menu">
        <li>Delete track</li>
    </menu>;
}
