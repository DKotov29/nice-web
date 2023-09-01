mod api;
mod components;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use components::{
    SignIn,
    Nav,
    Posts
};
use crate::api::Api;


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
        _ => html! { <h1>{ "Home" }</h1> }
    }
}

#[function_component(App)]
fn app() -> Html {
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
