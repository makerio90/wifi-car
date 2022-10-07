mod props;

use props::Button;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Hello World: " } <Button text="hello" /></h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
