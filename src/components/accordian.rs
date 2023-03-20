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
    #[prop_or_default]
    pub active: bool,
    #[prop_or_default]
    pub obj_type: Option<AttrValue>,
}

#[function_component(Accordian)]
pub fn view(props: &Props) -> Html {
    let spec = use_spec();

    let collapsible  = css!(
        background-color: #eee;
        color: #444;
        cursor: pointer;
        padding: 18px;
        width: 100%;
        border: none;
        text-align: left;
        outline: none;
        font-size: 15px;
        &:hover {
            background: ${spec.color.alpha(0.1)};
        }
        &:after {
            content: "+"; /* Unicode character for "plus" sign (+) */
            font-size: 13px;
            color: white;
            float: right;
            margin-left: 5px;
        }
    );

    let content =css!(
        padding: 0 18px;
        display: none;
        overflow: hidden;
        background-color: #f1f1f1;
    );

    /* Add a background color to the button if it is clicked on (add the .active class with JS), and when you move the mouse over it (hover) */
    let active = css! (
        background-color: #ccc;
        &:active:after {
            content: "-"; /* Unicode character for "minus" sign (-) */
          }
    );

    html! {
        <button
            type={&props.obj_type}
            class={classes!(props.class.clone(), collapsible )}
            onclick={&props.onclick}
        >
        { props.children.clone() }
        </button>
    }
}
