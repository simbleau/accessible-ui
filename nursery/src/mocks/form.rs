use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockForm() -> Html {
    let onsubmit = Callback::from(|_| gloo_dialogs::alert("form submitted"));

    html! {
        <form id="myform" {onsubmit}>
            <InputField
                name="email"
                label="Email"
                placeholder="email"
                required={true}
            />
            <InputButton>{"Submit"}</InputButton>
        </form>
    }
}
