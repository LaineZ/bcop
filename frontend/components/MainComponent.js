import { HomePage } from "../pages/HomePage";
import { SidebarMenuComponent } from "./SidebarMenuComponent";
import { RouterView } from "./RouterView";


export class MainComponent extends Element {
    this(props) {
        this.props = props;
    }

    render() {
        return <main>
            <SidebarMenuComponent></SidebarMenuComponent>
            <RouterView />
        </main>;
    }
}