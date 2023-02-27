use std::iter::repeat_with;

use crate::style_context::use_spec;
use stylist::css;
use stylist::yew::styled_component;
use yew::prelude::*;


#[derive(Properties, PartialEq)]
pub struct SliderProps {
    /// The slider name 
    pub name: AttrValue,
    /// The aria-label attribute
    pub label: AttrValue,
    //min number for slider
    #[prop_or_default]
    pub min: AttrValue,
    //max number for slider
    #[prop_or_default]
    pub max: AttrValue,
    //default postion of slider
    #[prop_or_default]
    pub defaultValue: AttrValue,#[prop_or_default]

    #[prop_or_default]
    pub step: AttrValue,#[prop_or_default]

    #[prop_or_default]
    pub valueLabelDisplay: AttrValue,#[prop_or_default]

    #[prop_or_default]
    pub orient: AttrValue,#[prop_or_default]

    #[prop_or_default]
    pub onchange: Callback<Event>,
}

#[styled_component]
pub fn Slider(props: &SliderProps) -> Html {
    let spec = use_spec();
    let slider_ref = use_node_ref();

    let style = css! {
        -webkit-appearance: none;
        width: 100%;
        height: 5px;
        border-radius: 3px;
        background: #d3d3d3;
        outline: none;
        // opacity: 0.7;
        &:hover {
            background: ${spec.color.alpha(0.3)};
            
        }
        ::-webkit-slider-thumb {
            appearance: none;
            width: 10px;
            height: 10px;
            border-radius: 22px;
            background: #5283ff;
            cursor: pointer;
        }
        ::-webkit-slider-thumb :: &:hover{
            background: ${spec.color.alpha(0.9)};
            border-radius: 50px;
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
                    type="range"
                    name={&props.name}
                    ref={ slider_ref }
                    class={ style }
                    aria-labelledby={rand_id.clone()}
                    onchange={props.onchange.clone()}
                />
                <label id={rand_id} class={label_css}>{" "}{&props.label}</label>
            </div>
        </span>
    }
}
