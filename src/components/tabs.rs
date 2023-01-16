use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;

use crate::style_context::use_spec;

#[derive(Clone, PartialEq)]
pub struct Tab<T> {
    pub id: T,
    pub title: Html,
    pub title_class: Classes,
    pub panel: Html,
    pub panel_class: Classes,
}

#[derive(Clone, PartialEq, Properties)]
pub struct TabsProps<T: Clone + PartialEq> {
    pub selected: T,
    pub tabs: Vec<Tab<T>>,
    #[prop_or_default]
    pub onchange: Callback<T>,
    #[prop_or_default]
    pub class: Classes,
}

fn hash<T: Clone + PartialEq + Hash + 'static>(tab: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    tab.hash(&mut hasher);
    hasher.finish()
}

#[styled_component(Tabs)]
pub fn view<T: Clone + PartialEq + Hash + 'static>(
    props: &TabsProps<T>,
) -> Html {
    let spec = use_spec();

    let tab_refs = Rc::new(
        props
            .tabs
            .iter()
            .map(|x| (hash(&x.id), NodeRef::default()))
            .collect::<HashMap<_, _>>(),
    );

    let tabs = props
        .tabs
        .iter()
        .map(|x| {
            let hash = hash(&x.id);
            let selected = props.selected == x.id;
            (x, hash, selected)
        })
        .collect::<Vec<_>>();

    let tab_click = {
        move |(tab, _id, _selected): (&Tab<T>, &u64, &bool)| {
            let tab_id = tab.id.clone();
            props.onchange.reform(move |_| tab_id.clone())
        }
    };

    // CSS
    let tab_list_css = css! {
        r#"
            list-style:none;
            padding:0;
        "#
    };

    let tab_css = css! {
        r#"
            & {
                position:relative;
                color: ${not_selected};
                cursor:pointer;
                max-width:100%;
                margin: 0 auto;
                padding-top: 0;
                padding-left: 5px;
                padding-right: 5px;
                padding-bottom: 5px;
                transition: color 250ms;
            }
            &[aria-selected="true"], &:hover {
                color: ${selected};
            }

            @supports (text-decoration-thickness: initial) and (text-underline-offset: initial) {
                & {
                    transition: color 250ms, text-decoration-color 250ms;
                    text-decoration: underline solid;
                    text-underline-offset: 5px;
                    text-decoration-thickness: 3px;
                    text-decoration-color: ${not_selected};
                }
                &[aria-selected="true"], &:hover {
                    text-decoration-color: ${selected};
                }
            }

            "#,
        selected = spec.link,
        not_selected = spec.link.alpha(0.4),
    };

    let tab_panel_css = css! {
        & {
            margin-top:20px;
        }
        &[aria-hidden="true"] {
            display:none;
        }
    };

    html! {
        <div id="tabs" class={ props.class.clone() }>
            <ul class={ tab_list_css }>
                {
                    tabs
                        .iter()
                        .map(|(tab, id, selected)| html! {
                            <span
                                class={classes!(
                                    tab_css.clone(),
                                    tab.title_class.clone(),
                                )}
                                aria-expanded={selected.to_string()}
                                aria-selected={selected.to_string()}
                                role="tab"
                                data-tab-id={id.to_string()}
                                onclick={
                                    tab_click((tab, id, selected))
                                }
                                key={*id}
                                ref={ tab_refs[id].clone() }
                            >
                                { tab.title.clone() }
                            </span>
                        })
                        .collect::<Html>()
                }
            </ul>
            {
                tabs
                    .iter()
                    .filter(|(_, _, selected)| *selected )
                    .map(|(props, id, selected)| html! {
                        <div
                            class={classes!(
                                tab_panel_css.clone(),
                                selected.then(|| props.panel_class.clone()),
                            )}
                            role="tabpanel"
                            key={*id}
                        >
                            { props.panel.clone() }
                        </div>
                    })
                    .collect::<Html>()
            }
        </div>
    }
}
