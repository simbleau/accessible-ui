use crate::prelude::*;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Properties, PartialEq)]
pub struct RouterLinkProps<Route>
where
    Route: Routable,
{
    pub to: Route,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub icon: Option<IconMask>,
    #[prop_or_default]
    pub class: Classes,
}

#[styled_component(InternalLink)]
pub fn view<Route>(props: &RouterLinkProps<Route>) -> Html
where
    Route: Routable + 'static,
{
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
        <div class={classes!(hitbox_style, props.class.clone())}>
            <Link<Route> to={ props.to.clone() } classes={link_css} >
                if let Some(mask) = props.icon {
                    <Icon
                        data_aui_id="linkicon"
                        mask={mask}
                        class={classes!(css!("align-self: middle; margin-right: 3px;"))}
                    />
                }
                { props.children.clone() }
            </Link<Route>>
        </div>
    }
}
