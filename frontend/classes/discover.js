/** Discover managment */
class Discover {
    constructor() {
        this.discover = [];
        this.page = 1;
    }

    extend(tags) {
        const json = {
            filters: {
                format: "all",
                location: 0,
                sort: "pop",
                tags: tags
            },
            page: this.page
        };

        var me = this;

        loading.style.display = "block";

        httpRequestPost("https://bandcamp.com/api/hub/2/dig_deeper", JSON.stringify(json), function (response) {
            const jsonRes = JSON.parse(response);
            jsonRes.items.forEach(element => {
                me.discover.push(element.tralbum_url);
                const node = createElementFromHTML(discoverAlbumCard(element.title, element.artist, element.genre));

                $(node).children(function() {
                    if ($(this).prop("className") == "album-image") {
                        setImage(element.art_id, $(this)[0]);
                    }
                });

                $('#albums-select').append(node);
            });
            loading.style.display = "none";
            me.page += 1;
        });
    }

    clearDiscover() {
        this.page = 1;
        this.discover = [];
        $('#albums-select').empty();
    }
} ``