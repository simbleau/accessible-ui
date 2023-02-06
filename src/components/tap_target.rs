use crate::prelude::*;
use cssugar::prelude::*;
use stylist::css;
use yew::prelude::*;

pub const SIZE: Length = Length::Px(48.0);
const FG_SIZE: Length = Length::Px(24.0);

#[derive(Properties, PartialEq)]
pub struct Props {
    pub mask: IconMask,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub background: Option<Color>,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(TapTarget)]
pub fn view(props: &Props) -> Html {
    let spec = use_spec();

    // Coloring
    let bg = match props.background {
        Some(c) => c,
        None => spec.color.alpha(0.10),
    };
    let fg = match props.color {
        Some(c) => c,
        None => spec.color,
    };

    let style = css! {
        display: inline-block;
        border: 0;
        border-radius: 50%;
        padding: 0;
        align-items: center;
        justify-content: center;
        cursor:pointer;

        width: ${SIZE};
        height: ${SIZE};

        transition: background-color 0.5s;
        background-color: ${bg};


        &:hover {
            background-color: ${bg.darken(0.2)};
        }

        & > i {
            transition: width 0.25s, height 0.25s;
            background: ${fg};
            width: ${FG_SIZE * IconMask::aspect_ratio(props.mask)};
            height: ${FG_SIZE};
        }

        &:hover > i {
            background: ${fg.lighten(0.2)};
            width: ${FG_SIZE * IconMask::aspect_ratio(props.mask) * 1.25};
            height: ${FG_SIZE * 1.25};
        }
    };

    html! {
        <button class={ classes!(style, props.class.clone()) }
                onclick={ props.onclick.clone() }
        >
        <Icon
            mask={ props.mask }
            class={ css!("pointer-events: none;") }
        />
        </button>
    }
}
