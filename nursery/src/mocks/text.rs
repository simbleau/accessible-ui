use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component]
pub fn MockText() -> Html {
    html! {
        <>
        <div style="width: 150px; height: 20px">
            <Text ellipsize={true}>
                {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Purus non enim praesent elementum facilisis leo vel fringilla est. Ac ut consequat semper viverra nam libero justo laoreet sit. Viverra adipiscing at in tellus integer feugiat scelerisque. Lobortis elementum nibh tellus molestie nunc non blandit massa. Nunc eget lorem dolor sed viverra ipsum nunc aliquet. Dis parturient montes nascetur ridiculus mus. Iaculis at erat pellentesque adipiscing commodo elit at. Vel eros donec ac odio tempor. Viverra adipiscing at in tellus integer feugiat scelerisque varius morbi. Sed viverra tellus in hac habitasse platea. Fermentum iaculis eu non diam phasellus vestibulum lorem sed risus. Ut venenatis tellus in metus vulputate eu scelerisque."}
            </Text>
        </div>
        <Text ellipsize={false}>
            {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Purus non enim praesent elementum facilisis leo vel fringilla est. Ac ut consequat semper viverra nam libero justo laoreet sit. Viverra adipiscing at in tellus integer feugiat scelerisque. Lobortis elementum nibh tellus molestie nunc non blandit massa. Nunc eget lorem dolor sed viverra ipsum nunc aliquet. Dis parturient montes nascetur ridiculus mus. Iaculis at erat pellentesque adipiscing commodo elit at. Vel eros donec ac odio tempor. Viverra adipiscing at in tellus integer feugiat scelerisque varius morbi. Sed viverra tellus in hac habitasse platea. Fermentum iaculis eu non diam phasellus vestibulum lorem sed risus. Ut venenatis tellus in metus vulputate eu scelerisque."}
        </Text>
        <Text inline={true} style="color: red">
            {"inline text 1"}
        </Text>
        <Text inline={true} style="color: blue">
            {"inline text 2"}
        </Text>
        </>
    }
}
