use std::ops::Deref;
use std::rc::Rc;
use std::sync::Mutex;
use gloo::console::console;
use gloo::dialogs::alert;
use yew::prelude::*;
use wasm_bindgen::{JsCast, JsValue, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::{function_component, Html};
use yew_hooks::use_async;
use yew_router::hooks::use_navigator;
use yewdux::prelude::{Dispatch, use_store};
use crate::api::types::Credentials;
use crate::{AppRoute, State};
use yew_router::prelude::Link;

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let navigator = use_navigator().unwrap();
    // let mut status_message = Mutex::new(String::new());

    let (state, dispatch) = use_store::<State>();

    let username = use_node_ref();
    let password = use_node_ref();

    let state1 = state.clone();
    let dispatch1 = dispatch.clone();
    let callback = {
        let username = username.clone();
        let password = password.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let state = state1.clone();
            let dispatch = dispatch1.clone();

            let username = username.clone().cast::<HtmlInputElement>().unwrap();
            let password = password.clone().cast::<HtmlInputElement>().unwrap();
            wasm_bindgen_futures::spawn_local(async move {

                match state.api.sign_in(Credentials {
                    username: username.value(),
                    password: password.value()
                }).await {
                    Ok(session) => {
                        dispatch.reduce_mut(|state| state.api.session = Some(session));
                    },
                    Err(err) => {
                        alert(&format!("{:?}", err));
                    }
                }
            });
        }
    };

    html! {
        <>
            if let Some(session) = &state.api.session {
                <p>{ "You’ve been signed in successfully!" }</p>
                <Link<AppRoute> to={AppRoute::Posts}>
                    { "Show My Posts" }
                </Link<AppRoute>>
            } else {
                <form
                onsubmit={callback}
                >
                    <input ref={username} placeholder={"username"}/>
                    <input ref={password} placeholder={"password"}/>
                    <button submit={"true"}>{"Sign In"}</button>
                </form>
            }
        </>
    }
}
