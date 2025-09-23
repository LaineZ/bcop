import { HomePage } from "../pages/HomePage";
import { store } from "../store.js";

export class SidebarMenuComponent extends Element {
    render() {
        return <ul class="sidebar-menu">
            <li class="scale-active-anim" title="Home" onClick={() => store.setState({ route: "home" })}><icon|home /></li>
            <li class="scale-active-anim" title="Now playing" onClick={() => store.setState({ route: "nowplaying" })}><icon|video /></li>
            <li class="scale-active-anim" title="Discover" onClick={() => store.setState({ route: "discover" })}><icon|tag /></li>
            <li class="scale-active-anim" title="Search" onClick={() => store.setState({ route: "search" })}><icon|search /></li>
            <li class="scale-active-anim" title="Settings"><icon|cog /></li>
        </ul>;
    }
}