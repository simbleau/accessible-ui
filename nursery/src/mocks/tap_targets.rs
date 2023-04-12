use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockTapTarget() -> Html {
    let onclick = Callback::from(|_| gloo_dialogs::alert("Clicked"));

    html! {
    <>
        <TapTarget mask={IconMask::Mastodon} onclick={onclick.clone()} />
        <TapTarget mask={IconMask::GitHub} onclick={onclick.clone()} />
        <TapTarget mask={IconMask::LinkedIn} {onclick} />
    </>
    }
}
