use std::ops::Deref;
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;
use crate::{AppRoute, State};
use crate::api::types::Post as ApiPost;
use crate::api::Api;


#[derive(Properties, PartialEq)]
pub struct PostProps {
    post_id: i32,
    title: String,
    description: String,
    delete_callback: Callback<i32>
}

#[function_component(Posts)]
pub fn posts() -> Html {
    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<State>();
    let posts = use_state(|| Vec::<ApiPost>::new());

    if state.api.session.is_none() {
        navigator.push(&AppRoute::SignIn);
        return html! {}
    }

    let sign_out = {
        let posts = posts.clone();
        Callback::from(move |_| {
            posts.clone().set(vec![ApiPost {
                post_id: 0,
                title: "Hello".to_owned(),
                description: "I'm a post".to_owned(),
                user_id: 1,
                bookmarked: false,
            }]);
            web_sys::console::log_1(&JsValue::from("Log out this little shit".to_owned()))
        })
    };


    let delete_callback = {
        Callback::from(|id| {
            web_sys::console::log_1(&JsValue::from(id));
        })
    };


    let posts = posts.deref().into_iter().map(|post| {
        html! {
            <Post post_id={post.post_id.clone()} title={post.title.clone()} description={post.description.clone()} delete_callback={delete_callback.clone()}/>
        }
    }).collect::<Html>();
    html! {
        <>
            <div>
                <button onclick={sign_out}>
                    { "Sign Out" }
                </button>
            </div>
            <div class={"posts"}>
                { posts }
            </div>
        </>
    }
}

#[function_component(Post)]
pub fn post(props: &PostProps) -> Html {

    // let delete_callback = {
    //     Callback::from(move |_| {
    //         props.delete_callback.emit(props.post_id)
    //     })
    // };

    let delete_callback = props.delete_callback.clone();
    let id = props.post_id.clone();
    html! {
        <>
            <div class={"post"}>
                <div class={"post_actions"}>
                    <button class={"button"}>{"Bookmark"}</button>
                    <button onclick={move |_| delete_callback.emit(id)} class={"button red"}>{"Delete"}</button>
                </div>
                <div class={"post_content"}>
                    <p class={"post_content_title"}>{ &props.title }</p>
                    <p class={"post_content_description"}>{ &props.description }</p>
                </div>
            </div>
        </>
    }
}