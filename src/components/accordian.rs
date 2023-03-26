use crate::style_context::use_spec;
use stylist::css;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    pub title: AttrValue,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub obj_type: Option<AttrValue>,
}

#[function_component(Accordian)]
pub fn view(props: &Props) -> Html {
    let spec = use_spec();
    let active = use_state(|| false);

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
    );

    // let content =css!(
    //     padding: 0 18px;
    //     display: none;
    //     overflow: hidden;
    //     background-color: #f1f1f1;
    // );

    /* Add a background color to the button if it is clicked on (add the .active class with JS), and when you move the mouse over it (hover) */
    let active_style = css! (
        background-color: #f00;

        &:after {
            content: "-"; /* Unicode character for "plus" sign (+) */
            font-size: 13px;
            color: white;
            float: right;
            margin-left: 5px;
        }
    );
    let inactive_style = css! (
        background-color: #f0f;

        &:after {
            content: "+"; /* Unicode character for "plus" sign (+) */
            font-size: 13px;
            color: white;
            float: right;
            margin-left: 5px;
        }
    );

    let on_click_handler = {
        let active = active.clone();
        Callback::from(move |_| {
            active.set(!*active);
        })
    };

    let classes = classes!(
        props.class.clone(),
        collapsible,
        if *active {
            active_style
        } else {
            inactive_style
        }
    );

    html! {
        <div id="accordian-group">
            <button
                type={&props.obj_type}
                class={classes}
                onclick={&on_click_handler}
            >
            { &props.title }
            </button>
            if *active {
                <div class={classes!(props.class.clone())}>
                    { props.children.clone() }
                </div>
            }
        </div>
    }
}
