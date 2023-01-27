function queuedTrackCard(title, artist) {
    return `<div class="track-card">
        <img class="track-img">
        <div class="track-card-desc">
            <h3>${title}</h3>
            <p>${artist}</p>
        </div>
    </div>`;
}

function discoverAlbumCard(title, artist, genre) {
    return `<div class="album-card">
    <img class=album-image>
        <div class="album-description">
            <h3 title="${title}">${title}</h3>
            <p>${artist} × ${genre}</p>
        </div>
    </div>`;
}

function searchResultCard(title, artist, url) {
    return `<div class="track-card" value="${url}">
    <img class="search-img" src="icons/audio.jpg">
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
