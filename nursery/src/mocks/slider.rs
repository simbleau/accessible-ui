use accessible_ui::{components::slider::Slider};
use yew::prelude::*;

#[function_component]
pub fn MockSlider() -> Html {
    html! {
    <>
        <Slider
            name="Slider"
            label="Option 1"
            max="200"
            min="1"
            defaultValue="1"
            step = "1"
            valueLabelDisplay= "auto"
            orient="vertical"
            
        />
    </>
    }
}
