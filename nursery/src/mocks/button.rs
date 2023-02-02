use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockButton() -> Html {
    let onclick = Callback::from(|_| gloo_dialogs::alert("Clicked"));

    html! {
        <Button {onclick}>
            {"Click me!"}
        </Button>
    }
}
