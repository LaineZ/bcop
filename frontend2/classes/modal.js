class Modal {
    constructor(id) {
        this.modalDim = document.getElementById("modal-dim");
        this.modalWindow = document.getElementById(id);
    }

    show() {
        this.modalDim.classList.add("active");
        this.modalWindow.style.display = "block";

        const event = new Event("open");
        this.modalWindow.dispatchEvent(event);
    }

    hide() {
        if (this.modalWindow.style.display != "none") {
            const event = new Event("closed");
            this.modalWindow.dispatchEvent(event);
        } 

        this.modalWindow.style.display = "none";
        this.modalDim.classList.add("closing");

        setTimeout(() => {
            this.modalDim.classList.remove("active");
            this.modalDim.classList.remove("closing");
        }, 200);
    }
}