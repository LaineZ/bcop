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
                $('#albums-select')
                    .append(discoverAlbumCard(element.art_id, element.title, element.artist, element.genre));
    
                var card = document.getElementById("art_" + element.art_id);
                setImage(element.art_id, card);
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
}