/** Progress loading indicator */
class LoadingIndicator {
    constructor() {
        this.loading = document.getElementById("loading");
    }

    spawn() {
        this.loading.style.display = "block";
    }

    destroy() {
        this.loading.classList.add("closing");

        var me = this;
        setTimeout(function() {
            me.loading.style.display = "none";
            me.loading.classList.remove("closing");
        }, 200);
    }
}