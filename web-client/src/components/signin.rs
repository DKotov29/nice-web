use gloo::dialogs::alert;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, Html};
use yew_hooks::use_session_storage;
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;
use yewdux::prelude::use_store;

use crate::api::types::{Credentials, Session};
use crate::{AppRoute, State};

#[function_component(SignIn)]
pub fn sign_in() -> Html {
    let _navigator = use_navigator().unwrap();
    // let mut status_message = Mutex::new(String::new());

    let (state, dispatch) = use_store::<State>();

    let stored_session = use_session_storage::<Session>("session".to_owned());

    let username = use_node_ref();
    let password = use_node_ref();

    let state1 = state.clone();
    let dispatch1 = dispatch.clone();
    let callback = {
        let username = username.clone();
        let password = password.clone();
        let stored_session = stored_session.clone();
        move |e: SubmitEvent| {
            e.prevent_default();
            let state = state1.clone();
            let dispatch = dispatch1.clone();
            let stored_session = stored_session.clone();

            let username = username.clone().cast::<HtmlInputElement>().unwrap();
            let password = password.clone().cast::<HtmlInputElement>().unwrap();
            wasm_bindgen_futures::spawn_local(async move {
                match state
                    .api
                    .sign_in(Credentials {
                        username: username.value(),
                        password: password.value(),
                    })
                    .await
                {
                    Ok(session) => {
                        dispatch.reduce_mut(|state| state.api.session = Some(session.clone()));
                        stored_session.set(session);
                    }
                    Err(err) => {
                        alert(&format!("{:?}", err));
                    }
                }
            });
        }
    };

    html! {
        <>
            if let Some(_session) = &state.api.session {
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
