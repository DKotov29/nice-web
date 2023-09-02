use yew::prelude::*;
use yew_router::prelude::*;

use crate::AppRoute;

#[function_component(Nav)]
pub fn nav() -> Html {
    html! {
        <div>
            <Link<AppRoute> to={AppRoute::SignUp}>
                { "Sign up" }
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::SignIn}>
                { "Sign in" }
            </Link<AppRoute>>
            <Link<AppRoute> to={AppRoute::Posts}>
                { "Show user's page" }
            </Link<AppRoute>>
        </div>
    }
}
