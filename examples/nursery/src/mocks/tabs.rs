use accessible_ui::prelude::*;
use yew::prelude::*;

#[function_component(MockTabs)]
pub fn view() -> Html {
    let selected = use_state(|| TabChoice::A);

    let onchange = {
        let selected = selected.clone();
        Callback::from(move |x| {
            selected.set(x);
        })
    };

    html! {
        <Tabs<TabChoice>
            selected={*selected}
            onchange={ onchange }
            tabs={vec![a(), b()]}
        />
    }
}

#[derive(Debug, Clone, Copy, Hash, PartialEq)]
enum TabChoice {
    A,
    B,
}

fn a() -> Tab<TabChoice> {
    Tab {
        id: TabChoice::A,
        title: html!({ "Tab A" }),
        title_class: classes!(),
        panel: html!(
            <ul>
                <li>{"Something that goes on Tab A"}</li>
            </ul>
        ),
        panel_class: classes!(),
    }
}

fn b() -> Tab<TabChoice> {
    Tab {
        id: TabChoice::B,
        title: html!({ "Tab B" }),
        title_class: classes!(),
        panel: html!(
            <ul>
                <li>{"Something that goes on Tab B"}</li>
            </ul>
        ),
        panel_class: classes!(),
    }
}
