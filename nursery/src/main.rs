use accessible_ui::prelude::*;
use component_container::ComponentContainer;
use nav::Navigation;
use stylist::yew::{styled_component, Global};
use yew::prelude::*;
mod component_container;
mod mocks;
mod nav;

#[styled_component]
pub fn Root() -> Html {
    let no_margins = css!("html,body {margin: 0;}");
    html! {
        <ContextProvider<AuiSpec> context={ AuiSpec::default() } >
            <Global css={no_margins} />
            <App />
        </ContextProvider<AuiSpec>>
    }
}

#[styled_component]
pub fn App() -> Html {
    let selected = use_state(|| html!("Nothing selected"));

    let app_css = css! {
        margin: auto;
        max-width: 1100px;
        min-height: 100vh;
    };

    let wrap_css = css! {
        display: flex;
    };

    html! {
        <div id="app" class={app_css}>
            <div id="wrap" class={wrap_css}>
                <Navigation selected={selected.clone()} />
                <ComponentContainer {selected} />
            </div>
        </div>
    }
}

pub fn main() {
    yew::Renderer::<Root>::new().render();
}
