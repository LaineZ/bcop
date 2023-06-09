class View {
    constructor(id, buttonId, data) {
        this.view = document.getElementById(id);
        this.button = document.getElementById(buttonId);
        this.data = data;
        this.hide();
    }

    show() {
        this.view.style.display = "block";
        this.view.classList.add("slide-animation");
        const event = new CustomEvent("open", {
            detail: this.data
        });
        this.view.dispatchEvent(event);
        this.button.classList.add("selected");
    }

    hide() {
        if (this.view.style.display != "none") {
            const event = new CustomEvent("closed", {
                detail: this.data
            });
            this.view.dispatchEvent(event);
            this.view.classList.remove("slide-animation");
            this.view.style.display = "none";
            this.button.classList.remove("selected");
        }
    }
}