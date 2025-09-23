
class Store {
    #state = {};
    #listeners = new Set();

    constructor(initial) {
        this.#state = initial;
    }

    get state() {
        return this.#state;
    }

    setState(patch) {
        this.#state = { ...this.#state, ...patch };
        this.#listeners.forEach(cb => cb(this.#state));
    }

    subscribe(cb) {
        this.#listeners.add(cb);
        return () => this.#listeners.delete(cb);
    }
}

export const store = new Store({ route: "home" });