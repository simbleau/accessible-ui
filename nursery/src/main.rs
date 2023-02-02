use accessible_ui::prelude::*;
use component_container::ComponentContainer;
use nav::Navigation;
use stylist::yew::styled_component;
use yew::prelude::*;
mod component_container;
mod mocks;
mod nav;

#[function_component]
pub fn Root() -> Html {
    html! {
        <ContextProvider<AuiSpec> context={ AuiSpec::default() } >
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

    html! {
        <div id="app" class={app_css}>
            <Navigation selected={selected.clone()} />
            <ComponentContainer {selected} />
        </div>
    }
}

pub fn main() {
    yew::Renderer::<Root>::new().render();
}
