use crate::mocks::*;
use accessible_ui::prelude::*;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct NavigationProps {
    pub selected: UseStateHandle<Html>,
}

#[function_component]
pub fn Navigation(props: &NavigationProps) -> Html {
    let mut items = vec![];
    items.push(("Button", html!(<button::MockButton />)));
    items.push(("External Link", html!(<external_link::MockExternalLink />)));
    items.push(("Form", html!(<form::MockForm />)));
    items.push(("Icons", html!(<icons::MockIcon />)));
    items.push(("Internal Link", html!(<internal_link::MockInternalLink />)));
    items.push(("Preformatted", html!(<preformatted::MockPreformatted />)));
    items.push(("Spinner", html!(<spinner::MockSpinner />)));
    items.push(("Tabs", html!(<tabs::MockTabs />)));
    items.push(("TapTargets", html!(<tap_targets::MockTapTarget />)));
    items.push(("Radio Button", html!(<radio_button::MockRadioButton />)));
    items.push(("Checkbox", html!(<checkbox::MockCheckbox />)));

    let change_selected = {
        let selected = props.selected.clone();
        move |new_selection: Html| {
            selected.set(new_selection);
        }
    };

    html! {
        {
            items.into_iter().map(|(name, item)| {
                let change_selected = change_selected.clone();
                html!{
                    <RadioButton
                        name={"selection"}
                        label={name}
                        onchange={Callback::from(
                            move |_e| {
                                change_selected(item.clone());
                            }
                        )}
                    />
                }
            }).collect::<Html>()
        }
    }
}