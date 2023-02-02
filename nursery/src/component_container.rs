use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ComponentContainerProps {
    pub selected: UseStateHandle<Html>,
}

#[function_component]
pub fn ComponentContainer(props: &ComponentContainerProps) -> Html {
    html! {
        <div style="display: inline-block">
            {(*props.selected).clone()}
        </div>
    }
}
