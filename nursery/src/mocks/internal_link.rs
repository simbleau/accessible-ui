use accessible_ui::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn MockInternalLink() -> Html {
    html! {
        <BrowserRouter>
            <InternalLink<Route> to={Route::Nowhere} icon={IconMask::GitHub}>
                {"Internal Link"}
            </InternalLink<Route>>
        </BrowserRouter>
    }
}

#[derive(Routable, Clone, PartialEq)]
pub enum Route {
    #[at("/")]
    Nowhere,
}
