use crate::style_context::use_spec;
use cssugar::prelude::*;
use stylist::css;
use yew::prelude::*;

const BUTTON_BORDER_WIDTH: Length = Length::Px(1.0);
const BUTTON_BORDER_RADIUS: Length = Length::Px(5.0);
const BUTTON_PADDING: Length = Length::Px(8.0);

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
    let spec = use_spec();

    let style = css!(
        cursor:pointer;
        padding: ${BUTTON_PADDING};
        text-align: center;

        border-radius: ${BUTTON_BORDER_RADIUS};
        border-width: ${BUTTON_BORDER_WIDTH};
        border-style: solid;
        border-color: ${spec.color};

        color: ${spec.color};
        background: none;
        &:hover {
            background: ${spec.color.alpha(0.1)};
        }
    );

    html! {
        <button
            type="submit"
            class={classes!(props.class.clone(), style)}
            onclick={&props.onclick}
        >
        { props.children.clone() }
        </button>
    }
}
