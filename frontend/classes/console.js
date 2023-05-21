class Console {
    constructor() {
        this.console = $("#console");
        this.consoleContents = $("#console-contents");
        this.consoleInput = $("#console-input");
        this.hidden = true;
        this.history = [];
        this.selectedHistory = 0;

        this.hide();

        $('*').on('click', (e) => {
            if (!this.hidden) {
                e.preventDefault();
                this.#selectElement(e.target);
                this.consoleInput[0].focus();
                e.stopImmediatePropagation();
            }
        });

        this.consoleInput.keyup((e) => {
            if (e.keyCode == keys.ENTER) {
                this.executeCommand(this.consoleInput.val());
                this.history.push(this.consoleInput.val());
                this.consoleInput.val("");
                this.consoleInput[0].focus();
            }

            if (this.history.length > 0) {
                if (e.keyCode == 264) {
                    this.selectedHistory = clamp(this.selectedHistory + 1, 0, this.history.length - 1);
                    this.consoleInput.val(this.history[this.selectedHistory]);
                }

                if (e.keyCode == 265) {
                    this.selectedHistory = clamp(this.selectedHistory - 1, 0, this.history.length - 1);
                    this.consoleInput.val(this.history[this.selectedHistory]);
                }
            }

            if (e.keyCode == 96) {
                this.toggle();
            }
        });
    }

    #selectElement(element) {
        $("#dom-element").text(element);
        this.selectedDomElement = element;
    }

    executeCommand(input) {
        this.print("> " + input);
        try {
            // register functions
            let command = input.split(" ");
            const remove = (element) => {
                if (!element) {
                    this.print("DOM element is missing!");
                    return;
                };

                element.remove();
            }

            const disable = (element) => {
                if (!element) {
                    this.print("DOM element is missing!");
                    return;
                };

                element.attr("disabled", true);
            }

            const enable = (element) => {
                if (!element) {
                    this.print("DOM element is missing!");
                    return;
                };

                element.attr("disabled", false);
            }

            const box = (element) => {
                if (!element) {
                    this.print("DOM element is missing!");
                    return;
                };

                $(element).css("border", "1px solid red");
                $(element).find("*").css("border", "1px solid red");
            }

            const rbox = (element) => {
                if (!element) {
                    this.print("DOM element is missing!");
                    return;
                };

                $(element).css("border", "none");
                $(element).find("*").css("border", "none");
            }

            let result = eval(input);

            if (typeof (result) == "object") {
                this.print(JSON.stringify(result, null, 2));
                return;
            }

            if (typeof (result) == "function") {
                let element = this.selectedDomElement;

                if (command.length > 1) {
                    element = $(command[1]);
                }

                result(element, command.splice(0, 2));
                return;
            }

            this.print(result);
        } catch (error) {
            this.print(error);
        }
    }

    hide() {
        this.console[0].classList.add("close");
        this.hidden = true;
    }

    show() {
        this.consoleInput[0].focus();
        this.console[0].classList.remove("close");
        this.hidden = false;
        this.consoleInput.val("");
    }

    toggle() {
        log(this.hidden);
        if (this.hidden) {
            this.show();
        } else {
            this.hide();
        }
    }

    print(...any) {
        this.consoleContents.append(`${any}<br>`);
    }
}