use accessible_ui::{components::accordian::Accordian};
use yew::prelude::*;

#[function_component]
pub fn MockAccordian() -> Html {
    // let onclick = Callback::from(active = true);
    html! {
    <>
        <Accordian title={"A collapsable component!"} class={stylist::css!("color: black; background-color: pink;")}>
            {"I should only be visible if it's not collapsed!"}
        </Accordian>
    </>
    }
}
