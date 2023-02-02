use accessible_ui::prelude::*;
use gloo_dialogs::alert;
use yew::prelude::*;
mod mocks;

#[function_component(Root)]
pub fn view() -> Html {
    html! {
        <ContextProvider<AuiSpec> context={ AuiSpec::default() } >
            <App />
        </ContextProvider<AuiSpec>>
    }
}

#[derive(Properties, PartialEq)]
struct WrapperProps {
    children: Children,
}

#[function_component(Wrapper)]
fn view(props: &WrapperProps) -> Html {
    html! {
        <div style="display: inline-block">
            {props.children.clone()}
        </div>
    }
}

#[function_component(App)]
pub fn view() -> Html {
    let onsubmit = Callback::from(|_| gloo_dialogs::alert("form submitted"));

    html! {
        <div style="display: flex; flex-direction: column;" align="center">
            <Wrapper>
                <span style="font-size: 3em;">
                    <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
                </span>
            </Wrapper>
            <Wrapper>
                <span style="font-size: 2em;">
                    <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
                </span>
            </Wrapper>
            <Wrapper>
                <span style="font-size: 1em;">
                    <Icon mask={IconMask::Sun} /> {"Icons will inherit their computed size"}
                </span>
            </Wrapper>
            <Wrapper>
                <Button onclick={Callback::from(|_| alert("Clicked"))}>{"Click me!"}</Button>
            </Wrapper>
            <Wrapper>
                <Preformatted inner={"hello"} />
            </Wrapper>
            <Wrapper>
                <Spinner />
            </Wrapper>
            <Wrapper>
                <TapTarget mask={IconMask::Twitter} onclick={Callback::from(|_| alert("Clicked"))} />
                <TapTarget mask={IconMask::GitHub} onclick={Callback::from(|_| alert("Clicked"))} />
                <TapTarget mask={IconMask::LinkedIn} onclick={Callback::from(|_| alert("Clicked"))} />
            </Wrapper>
            <Wrapper>
                <mocks::tabs::MockTabs />
            </Wrapper>
            <Wrapper>
                {"This text is baseline: "}
                <mocks::external_link::MockExternalLink />
                {"..."}
            </Wrapper>
            <Wrapper>
                {"This text is baseline: "}
                <mocks::internal_link::MockInternalLink />
                {"..."}
            </Wrapper>
            <Wrapper>
                <form id="myform" {onsubmit}>
                    <InputField
                        name="email"
                        label="Email"
                        placeholder="email"
                        required={true}
                    />
                    <InputButton>{"Submit"}</InputButton>
                </form>
            </Wrapper>
            <Wrapper>
                <form id="myform">
                    <CheckBox
                        name="checkbox"
                        label="Default True"
                        checked={true}
                    />
                    <CheckBox
                        name="checkbox2"
                        label="Default False"
                        checked={false}
                    />
                </form>
            </Wrapper>
        </div>
    }
}

pub fn main() {
    yew::Renderer::<Root>::new().render();
}
