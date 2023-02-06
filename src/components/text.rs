use stylist::yew::styled_component;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[derive(Clone, PartialEq, Properties)]
pub struct TextProps {
    #[prop_or_default]
    pub ellipsize: bool,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    /// Wrap text in `span` instead of `div`.
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub title: Option<AttrValue>,
    #[prop_or_default]
    pub style: Option<AttrValue>,
}

#[styled_component]
pub fn Text(props: &TextProps) -> Html {
    let ellipsize_css = css! {
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
        word-wrap: normal;
    };

    html! {
        <@{if props.inline { "span" } else { "div"}}
            class={classes!(
                props.ellipsize.then_some(ellipsize_css),
                props.class.clone(),
            )}
            style={&props.style}
            title={&props.title}
        >
            {props.children.clone()}
        </@>
    }
}
