use crate::components::Button;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component]
pub fn InputButton(props: &Props) -> Html {
    html! {
        <Button
            obj_type="submit"
            class={classes!(props.class.clone())}
            onclick={&props.onclick}
        >
            { props.children.clone() }
        </Button>
    }
}
