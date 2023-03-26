use accessible_ui::{components::tooltip::Tooltip};
use yew::prelude::*;

#[function_component]
pub fn MockTooltip() -> Html {
    html! {
        <>
            <br/>
            <br/>
            <br/>
            <Tooltip class={stylist::css!("color: black; background-color: pink;")}>
                {"I should only be visible if it's not collapsed!"}
            </Tooltip>
            <br/>
            <br/>
            <br/>
            <Tooltip class={stylist::css!("color: black; background-color: pink;")}>
                {"I shouldnt be displayed if another is up"}
            </Tooltip>
        </>
    }
}
