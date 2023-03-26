use crate::{style_context::use_spec};
use stylist::css;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,

}

#[function_component(Tooltip)]
pub fn view(props: &Props) -> Html {
    let spec = use_spec();
    let parent = css! {
        &:hover #child{
            visibility: visible;
            opacity: 1;
            background: ${spec.color.alpha(0.1)};
            //font-size: 50%;
        }
    };

    let child = css! {
        font-size: 50%;
        visibility: hidden;
        display: inline-block;
        width: fit-content;
        background-color: ${spec.color.alpha(0.1)};
        //color: #f0f;
        text-align: center;
        padding: 5px 0;
        border-radius: 6px;
        z-index: 1;
        content: "";
        position: relative;
        border-width: 5px;
        border-style: solid;
        //border-color:  transparent transparent transparent;
        opacity: 0;
        transition: opacity 1.0s;
        &:after{
            content: "";
            position: absolute;
            top: 100%;
            left: 50%;
            margin-left: -5px;
            border-width: 5px;
            border-style: solid;
            border-color: #555 transparent transparent transparent;
        }


    };




    html! {
        <div >

            <div id="parent" class= {parent.clone()}>
                <div id="child" class = {child.clone()}>
                    { props.children.clone() }
                </div>
                <div>{"Now you see me"}</div>
             </div>
        </div>
    }
}
