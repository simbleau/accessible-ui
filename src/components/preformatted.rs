use crate::style_context::use_spec;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Properties, PartialEq)]
pub struct PreformattedProps {
    #[prop_or(AttrValue::Static("monospace"))]
    pub font_family: AttrValue,
    pub inner: String,
}

#[styled_component(Preformatted)]
pub fn view(props: &PreformattedProps) -> Html {
    let spec = use_spec();

    let preformatted_css = css! {
        font-family: ${props.font_family};
        white-space: pre-wrap;
        border-radius: 3px;
        background-color: ${spec.link.alpha(0.10)};
        color: ${spec.link};
    };

    html! {
        <span class={preformatted_css}>
            { props.inner.clone() }
        </span>
    }
}
