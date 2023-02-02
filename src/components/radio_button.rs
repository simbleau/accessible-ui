use std::iter::repeat_with;

use crate::style_context::use_spec;
use cssugar::prelude::*;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

const RADIO_HEIGHT: Length = Length::Px(20.0);
const RADIO_WIDTH: Length = Length::Px(20.0);

#[derive(Properties, PartialEq)]
pub struct RadioButtonProps {
    /// The radiobutton name for forms
    pub name: AttrValue,
    /// The aria-label attribute
    pub label: AttrValue,
    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[styled_component]
pub fn RadioButton(props: &RadioButtonProps) -> Html {
    let spec = use_spec();
    let radiobutton_ref = use_node_ref();

    let style = css! {
        cursor: pointer;
        text-align: center;
        min-width: ${RADIO_WIDTH};
        min-height: ${RADIO_HEIGHT};
        border-style: solid;
        border-color: ${spec.color};
        color: ${spec.color};
        background: none;
        &:hover {
            background: ${spec.color.alpha(0.1)};
        }
    };

    let container_css = css! {
        display: inline-block;
    };

    let label_css = css! {
        align-self: center;
    };

    let rand_id: String = repeat_with(fastrand::alphanumeric).take(8).collect();

    html! {
        <span class={container_css}>
            <div style="display: flex;">
                <input
                    type="radio"
                    name={&props.name}
                    ref={ radiobutton_ref }
                    class={ style }
                    aria-labelledby={rand_id.clone()}
                    onchange={props.onchange.clone()}
                />
                <label id={rand_id} class={label_css}>{" "}{&props.label}</label>
            </div>
        </span>
    }
}
