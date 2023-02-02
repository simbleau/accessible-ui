use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockRadioButton() -> Html {
    html! {
    <>
        <RadioButton
            name="RadioButton"
            label="Option 1"
        />
        <RadioButton
            name="RadioButton"
            label="Option 2"
        />
        <RadioButton
            name="RadioButton"
            label="Option 3"
        />
    </>
    }
}
