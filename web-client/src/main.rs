mod api;
mod components;

use yew::prelude::*;
use yew_hooks::use_local_storage;
use yew_router::prelude::*;
use yewdux::prelude::*;

use yew_hooks::use_session_storage;
use crate::api::types::Session;

use components::{
    SignIn,
    Nav,
    Posts
};
use crate::api::Api;
use crate::components::SignUp;


#[derive(Default, Clone, PartialEq, Eq, Store)]
pub struct State {
    api: Api
}

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    #[at("/sign_up")]
    SignUp,
    #[at("/sign_in")]
    SignIn,
    #[at("/")]
    Posts,
}

fn switch(routes: AppRoute) -> Html {
    match routes {
        AppRoute::SignIn => html! { <SignIn/> },
        AppRoute::Posts => html! { <Posts/> },
        _ => html! { <SignUp/> }
    }
}

#[function_component(App)]
fn app() -> Html {
    let (state, dispatch) = use_store::<State>();

    let session = use_session_storage::<Session>("session".to_owned());

    if let Some(session) = &*session {
        dispatch.reduce_mut(|state| state.api.session = Some(session.clone()));
    }

    html! {
        <div>
            <HashRouter>
                <Nav/>
                <Switch<AppRoute> render={switch} />
            </HashRouter>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
