use gloo::dialogs::alert;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew::{function_component, Html};
use yew_router::hooks::use_navigator;
use yew_router::prelude::Link;
use yewdux::prelude::use_store;

use crate::api::types::Credentials;
use crate::{AppRoute, State};

#[function_component(SignUp)]
pub fn sign_up() -> Html {
    let _navigator = use_navigator().unwrap();
    let signed_up = use_state(|| false);

    // let mut status_message = Mutex::new(String::new());

    let (state, _dispatch) = use_store::<State>();

    let username = use_node_ref();
    let password = use_node_ref();

    let callback = {
        let signed_up = signed_up.clone();

        let username = username.clone();
        let password = password.clone();
        let state = state.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let username = username.clone().cast::<HtmlInputElement>().unwrap();
            let password = password.clone().cast::<HtmlInputElement>().unwrap();
            let state = state.clone();
            let signed_up = signed_up.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match state
                    .api
                    .sign_up(Credentials {
                        username: username.value(),
                        password: password.value(),
                    })
                    .await
                {
                    Ok(_) => {
                        signed_up.set(true);
                    }
                    Err(err) => {
                        alert(&format!("{:?}", err));
                    }
                }
            });
        })
    };

    // let state1 = state.clone();
    // let dispatch1 = dispatch.clone();
    // let callback = {
    //     let username = username.clone();
    //     let password = password.clone();
    //     let signed_callback = signed_callback.clone();
    //     move |e: SubmitEvent| {
    //         e.prevent_default();
    //         let state = state1.clone();
    //         let dispatch = dispatch1.clone();
    //
    //         let username = username.clone().cast::<HtmlInputElement>().unwrap();
    //         let password = password.clone().cast::<HtmlInputElement>().unwrap();
    //         wasm_bindgen_futures::spawn_local(async move {
    //
    //             match state.api.sign_up(Credentials {
    //                 username: username.value(),
    //                 password: password.value()
    //             }).await {
    //                 Ok(_) => {
    //                     signed_callback.emit(true);
    //                 },
    //                 Err(err) => {
    //                     alert(&format!("{:?}", err));
    //                 }
    //             }
    //         });
    //     }
    // };

    html! {
        <>
            if *signed_up {
                <p>{ "You’ve been signed up successfully!" }</p>
                <Link<AppRoute> to={AppRoute::SignIn}>
                    { "Sign In" }
                </Link<AppRoute>>
            } else {
                <form
                onsubmit={callback}
                >
                    <input ref={username} placeholder={"username"}/>
                    <input ref={password} placeholder={"password"} type={"password"}/>
                    <button submit={"true"}>{"Sign Up"}</button>
                </form>
            }
        </>
    }
}
