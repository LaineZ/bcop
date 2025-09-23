import { HomePage } from "../pages/HomePage";
import { store } from "../store.js";

export class RouterView extends Element {
    activeRoute = "home";

    this(props) {
        this.props = props;
    }

    componentDidMount() {
        this.activeRoute = "home";
        this.unsubscribe = store.subscribe(state => {
            this.componentUpdate({ activeRoute: state.route || "home" });
        });
    }

    componentWillUnmount() {
        if (this.unsubscribe) this.unsubscribe();
    }

    render() {
        switch (this.activeRoute) {
            case "nowplaying":
                return <section>
                        <h1>Now playing</h1>
                    </section>;
            case "home":
                return <section>
                        <HomePage></HomePage>
                    </section>;
            default:
                console.error("No route found for: " + this.activeRoute);
        }
    }
}