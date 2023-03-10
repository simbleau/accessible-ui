use std::iter::repeat_with;

use crate::style_context::use_spec;
use cssugar::prelude::*;
use stylist::css;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const INPUT_BORDER_WIDTH: Length = Length::Px(1.0);
const INPUT_BORDER_RADIUS: Length = Length::Px(5.0);
const INPUT_PADDING: Length = Length::Px(8.0);
const SPACE_BETWEEN: Length = Length::Rem(0.5);

#[derive(Properties, PartialEq)]
pub struct InputFieldProps {
    /// The aria-required attribute
    #[prop_or_default]
    pub required: bool,
    /// The input field name in the form
    pub name: AttrValue,
    /// The aria-label attribute
    pub label: AttrValue,
    /// Placeholder text in the text field
    pub placeholder: AttrValue,
}

#[styled_component]
pub fn InputField(props: &InputFieldProps) -> Html {
    let spec = use_spec();
    let input_ref = use_node_ref();

    let style = css!(
        cursor:text;
        padding: ${INPUT_PADDING};
        margin-left: ${SPACE_BETWEEN};

        border-radius: ${INPUT_BORDER_RADIUS};
        border-width: ${INPUT_BORDER_WIDTH};
        border-style: solid;
        border-color: ${spec.color};

        color: ${spec.color};
        background: none;
        &:hover {
            background: ${spec.color.alpha(0.1)};
        }
    );

    // Focus on hover
    let onmouseover = {
        let input_ref = input_ref.clone();
        move |_: MouseEvent| {
            _ = input_ref
                .cast::<HtmlInputElement>()
                .and_then(|e| e.focus().ok());
        }
    };

    let rand_id: String = repeat_with(fastrand::alphanumeric).take(8).collect();

    html! {
        <>
            <label id={rand_id.clone()}>{&props.label}{": "}</label>
            <input
                name={&props.name}
                ref={ input_ref }
                class={ style }
                onmouseover={ onmouseover }
                aria-required={ if props.required { "true "} else { "false"} }
                aria-labelledby={rand_id}
                placeholder={&props.placeholder}
            />
        </>
    }
}
