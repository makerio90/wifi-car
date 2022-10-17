use gloo_net::http::Request;
use log::info;
use log::Level;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;
// Define the possible messages which can be sent to the component
pub enum Msg {
    Enable,
    Disable,
    Login(String),
}

pub struct App;

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Enable => {
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/api/enable").send().await;
                });
            }
            Msg::Disable => {
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/api/disable").send().await;
                });
            }
            Msg::Login(s) => {
                wasm_bindgen_futures::spawn_local(async move {
                    Request::post("/auth/login")
                        .header("Authorization", &format!("{:X}", Sha256::digest(s)))
                        .send()
                        .await;
                });
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let onkeypress = ctx.link().batch_callback(|e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input: InputElement = e.target_unchecked_into();
                let value = input.value();
                input.set_value("");
                Some(Msg::Login(value))
            } else {
                None
            }
        });
        html! {
        <>
            <nav class="navbar bg-light">
              <div class="container-fluid">
                <a class="navbar-brand" >{ "wificar" }</a>
                <div class="d-flex">
                    <input class="form-control" type="text" {onkeypress} placeholder="password" />
                </div>
              </div>
            </nav>
            <div class="card mx-auto" style="width: 280px;">
                <div class="card-header">
                    <button class="btn btn-success" id="enable" onclick={ctx.link().callback(|_| Msg::Enable)}>{"enable"}</button>
                    <button class="btn btn-danger" id="disable" onclick={ctx.link().callback(|_| Msg::Disable)}>{"disable"}</button>
                </div>
                <div class="container text-center mx-auto">
                    <div class="row">
                        <div class="col align-self-center">
                            <button class="btn btn-outline-secondary" id="fwd">{"fwd"}</button><br />
                        </div>
                    </div>

                    <div class="row">
                        <div class="col">
                            <button class="btn btn-outline-secondary" id="left">{"left"}</button>
                        </div>
                        <div class="col">
                            <button class="btn btn-outline-secondary" id="bkwd">{"bkwd"}</button>
                        </div>
                        <div class="col">
                            <button class="btn btn-outline-secondary" id="right">{"right"}</button>
                        </div>
                    </div>
                </div>
            </div>
        </>
                    }
    }
}

fn main() {
    console_log::init_with_level(Level::Debug);
    yew::Renderer::<App>::new().render();
}
