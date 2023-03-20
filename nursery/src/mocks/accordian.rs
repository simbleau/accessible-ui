use accessible_ui::{components::accordian::Accordian};
use yew::prelude::*;

#[function_component]
pub fn MockAccordian() -> Html {
    // let onclick = Callback::from(active = true);
    html! {
    <>
        <Accordian>
            {"A collapsable component!"}
        </Accordian>
    </>
    }
}
