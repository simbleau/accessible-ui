use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockCheckbox() -> Html {
    html! {
        <>
            <CheckBox
                name="checkbox"
                label="Default True"
                checked={true}
            />
            <CheckBox
                name="checkbox"
                label="Default False"
                checked={false}
            />
        </>
    }
}
