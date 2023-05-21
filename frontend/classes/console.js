class Console {
    constructor() {
        this.console = $("#console");
        this.consoleContents = $("#console-contents");
        this.consoleInput = $("#console-input");
        this.hidden = true;

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
                this.consoleInput.val("");
                this.consoleInput[0].focus();
            }

            if (e.keyCode == 96) {
                this.toggle();
            }
        });
    }

    #domCommand(command, name, action) {
        if (command[0].toLowerCase() == name) {

            let element = this.selectedDomElement;

            if (command.length > 1) {
                element = $(command[1]);
            }

            if (!element) {
                this.print("DOM element is missing!");
                return;
            }

            action();
        }
    }

    #selectElement(element) {
        $("#dom-element").text(element);
        this.selectedDomElement = element;
    }

    executeCommand(input) {
        try {
            let result = eval(input);
            this.print(result);
        } catch (error) {
            let command = input.split(" ");
            this.print("> " + input);
            this.#domCommand(command, "remove", () => {
                $(this.selectedDomElement).remove();
            });
    
            this.#domCommand(command, "text", () => {
                $(this.selectedDomElement).text(command[1]);
            });
    
            this.#domCommand(command, "disable", () => {
                $(this.selectedDomElement).attr("disabled", true);
            });
    
            this.#domCommand(command, "enable", () => {
                $(this.selectedDomElement).attr("disabled", false);
            });
    
            this.#domCommand(command, "box", () => {
                $(this.selectedDomElement).css("border", "1px solid red");
                $(this.selectedDomElement).find("*").css("border", "1px solid red");
            });
    
            this.#domCommand(command, "rbox", () => {
                $(this.selectedDomElement).css("border", "none");
                $(this.selectedDomElement).find("*").css("border", "none");
            });
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
        this.consoleContents.append(`<p>${any}</p>`);
    }
}