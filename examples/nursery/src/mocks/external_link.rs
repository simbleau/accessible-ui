use accessible_ui::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MockExternalLink)]
pub fn view() -> Html {
    html! {
        <ExternalLink to={"https://github.com"} icon={IconMask::GitHub}>
            {"External Link (GitHub)"}
        </ExternalLink>
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Nowhere,
}
