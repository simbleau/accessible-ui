use crate::{
    components::{Icon, IconMask},
    style_context::use_spec,
};
use stylist::yew::styled_component;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExternalLinkProps {
    #[prop_or(AttrValue::Static("#"))]
    pub to: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
}

#[styled_component(ExternalLink)]
pub fn view(props: &ExternalLinkProps) -> Html {
    let spec = use_spec();

    let hitbox_style = css! {
        & {
            display: inline-block;
        }
        & *[data-aui-id="linkicon"] {
            background-color: ${spec.link};
        }
        &:hover *[data-aui-id="linkicon"] {
            background-color: ${spec.link_hover};
        }
    };

    let link_css = css! {
        display: flex;
        flex-direction: row;
        align-items: baseline;
    };

    html! {
        <div class={hitbox_style}>
            <a href={ props.to.clone() } class={link_css} >
                if let Some(mask) = props.icon {
                    <Icon
                        data_aui_id="linkicon"
                        mask={mask}
                        class={classes!(css!("vertical-align: middle; margin-right: 3px;"))}
                    />
                }
                { props.children.clone() }
                <Icon
                    data_aui_id="linkicon"
                    mask={ IconMask::Share }
                    class={classes!(css!("vertical-align: baseline !important; margin-left: 3px;"))}
                    scale={ 0.75 }
                />
            </a>
        </div>
    }
}
