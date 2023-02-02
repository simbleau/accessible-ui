use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockPreformatted() -> Html {
    html! {
        <Preformatted inner={"hello"} />
    }
}
