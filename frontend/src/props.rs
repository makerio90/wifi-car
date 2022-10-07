use yew::prelude::*;
#[derive(Properties, PartialEq)]
pub struct ButtonProps {
    pub text: String,
}
#[function_component(Button)]
pub fn button(buttonprops: &ButtonProps) -> Html {
    html! {
        <button type="button" class="btn btn-primary">{ &buttonprops.text }</button>
    }
}
