use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockIcon() -> Html {
    html! {
        <>
            <div style="font-size: 3em;">
                <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
            </div>
            <div style="font-size: 2em;">
                <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
            </div>
            <div style="font-size: 1em;">
                <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
            </div>
        </>
    }
}
