import { MainComponent } from "./components/MainComponent";
import { Theme } from "./themeManager";

const velement = <MainComponent name="world"></MainComponent>;

function App() {
    return <body>
        <MainComponent name="govno"></MainComponent>
    </body>;
}

const theme = new Theme("hope_diamond");

document.body.patch(<App />);