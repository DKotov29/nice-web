use gloo::console::console;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, Html};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let username = use_node_ref();
    let password = use_node_ref();

    let callback = move |e: SubmitEvent| {
        e.prevent_default();
        web_sys::console::log_1(&JsValue::from("it works"));
    };

    html! {
        <>
            <form
            onsubmit={callback}
            >
                <input placeholder={"username"}/>
                <input placeholder={"password"}/>
                <input type={"submit"}/>
            </form>
            <label for="my-input">
                { "My input:" }
                <input
                    id="my-input"
                    type="text"
                />
            </label>
        </>
    }
}
