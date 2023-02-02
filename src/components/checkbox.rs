use std::iter::repeat_with;

use crate::style_context::use_spec;
use cssugar::prelude::*;
use stylist::css;
use stylist::yew::styled_component;
use web_sys::HtmlInputElement;
use yew::prelude::*;

const CHECKBOX_HEIGHT: Length = Length::Px(20.0);
const CHECKBOX_WIDTH: Length = Length::Px(20.0);

#[derive(Properties, PartialEq)]
pub struct CheckBoxProps {
    /// The checkbox field name in the form
    pub name: AttrValue,
    /// The aria-label attribute
    pub label: AttrValue,
    //The status of the checkbox
    pub checked: bool,
}

#[styled_component]
pub fn CheckBox(props: &CheckBoxProps) -> Html {
    let spec = use_spec();
    let checkbox_ref = use_node_ref();

    let style = css!{
        pointer-events: none;
        text-align: center;
        min-width: ${CHECKBOX_WIDTH};
        min-height: ${CHECKBOX_HEIGHT};
        border-style: solid;
        border-color: ${spec.color};
        color: ${spec.color};
        background: none;
        &:hover {
            background: ${spec.color.alpha(0.1)};
        }
    };

    let container_css = css!{
        cursor:pointer;
        display: inline-block;
    };

    let label_css = css! {
        pointer-events: none;
        align-self: center;
    };

    // Focus on hover
    let onmouseover = {
        let checkbox_ref = checkbox_ref.clone();
        move |_: MouseEvent| {
            _ = checkbox_ref
                .cast::<HtmlInputElement>()
                .and_then(|e| e.focus().ok());
        }
    };

    // Toggle 'checked' when clicking anywhere on the span
    let onclick = {
        let checkbox_ref = checkbox_ref.clone();
        move |_: MouseEvent| {
            let checkbox = checkbox_ref
                .cast::<HtmlInputElement>().unwrap();
            let is_checked = checkbox.checked();
            checkbox.set_checked(!is_checked);
        }
    };

    let rand_id: String = repeat_with(fastrand::alphanumeric).take(8).collect();

    html! {
        <span class={container_css} onclick={onclick}>
            <div style="display: flex;">
                <input
                    type="checkbox"
                    name={&props.name}
                    ref={ checkbox_ref }
                    class={ style }
                    onmouseover={ onmouseover }
                    aria-labelledby={rand_id.clone()}
                    checked={props.checked}
                />
                <label id={rand_id} class={label_css}>{" "}{&props.label}</label>
            </div>
        </span>
    }
}