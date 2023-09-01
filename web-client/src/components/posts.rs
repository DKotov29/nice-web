use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;
use crate::{AppRoute, State};
use crate::api::Api;

#[function_component(Posts)]
pub fn posts() -> Html {
    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<State>();

    if state.api.session.is_none() {
        navigator.push(&AppRoute::SignIn);
        return html! {}
    }

    let session_id = format!("{:?}", state.api.session);

    // let posts = use_async(async move {
    //     state.api.get_posts()
    // });

    // let api: &Api = &state.api;

    html! {
        <>
            { session_id }
        </>
    }
}