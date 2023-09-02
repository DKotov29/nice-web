use std::ops::Deref;
use gloo::dialogs::alert;
use wasm_bindgen::JsValue;
use web_sys::{HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yew_hooks::{use_async, use_session_storage};
use yew_router::hooks::use_navigator;
use yewdux::prelude::use_store;
use crate::{AppRoute, State};
use crate::api::types::{CreatePost, Post as ApiPost, Session};
use crate::api::Api;


#[derive(Properties, PartialEq)]
pub struct PostProps {
    post_id: i32,
    title: String,
    description: String,
    is_bookmarked: bool,
    delete_callback: Callback<i32>,
    bookmark_callback: Callback<(i32, bool)>
}

#[function_component(Posts)]
pub fn posts() -> Html {
    let navigator = use_navigator().unwrap();
    let (state, dispatch) = use_store::<State>();

    if state.api.session.is_none() {
        navigator.push(&AppRoute::SignIn);
        return html! {}
    }

    let title = use_node_ref();
    let description = use_node_ref();

    let posts = use_state_eq(|| Vec::<ApiPost>::new());
    let loading = use_state(|| false);
    let navigator = use_navigator().unwrap();
    let stored_session = use_session_storage::<Session>("session".to_owned());

    let load_user_posts = {
        let state = state.clone();
        let posts = posts.clone();
        let loading = loading.clone();
        Callback::from(move |_: ()| {
            let state = state.clone();
            let posts = posts.clone();
            let loading = loading.clone();

            // loading.set(true);

            wasm_bindgen_futures::spawn_local(async move {
                match state.api.get_posts().await {
                    Ok(user_posts) => {
                        posts.set(user_posts);
                    },
                    Err(err) => {
                        alert(&format!("Error: {}", err));
                    }
                }
                // loading.set(false);
            });
        })
    };

    let add_post_callback = {
        let title = title.clone();
        let description = description.clone();
        let state = state.clone();
        let load_user_posts = load_user_posts.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let title = title.cast::<HtmlInputElement>().unwrap();
            let description = description.cast::<HtmlTextAreaElement>().unwrap();

            let state = state.clone();
            let load_user_posts = load_user_posts.clone();

            let create_post = CreatePost {
                title: title.value(),
                description: description.value()
            };

            wasm_bindgen_futures::spawn_local(async move {
                match state.api.create_post(create_post).await {
                    Ok(_) => {
                        load_user_posts.emit(());
                    },
                    Err(err) => alert(&format!("Error: {}", err))
                }
            })
        })
    };

    load_user_posts.emit(());

    let sign_out = {
        let state = state.clone();
        let dispatch = dispatch.clone();
        let stored_session = stored_session.clone();
        let navigator = navigator.clone();
        Callback::from(move |_| {
            let state = state.clone();
            let stored_session = stored_session.clone();
            let navigator = navigator.clone();
            let dispatch = dispatch.clone();
            wasm_bindgen_futures::spawn_local(async move {
                match state.api.sign_out().await {
                    Ok(_) => {
                        stored_session.delete();
                        navigator.push(&AppRoute::SignIn);
                        dispatch.
                    },
                    Err(err) => alert(&format!("Error: {}", err))
                }
            })
        })
    };

    let delete_callback = {
        let state = state.clone();
        let load_user_posts = load_user_posts.clone();
        Callback::from(move |id| {
            let state = state.clone();
            let load_user_posts = load_user_posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match state.api.delete_post(id).await {
                    Ok(_) => {
                        load_user_posts.emit(());
                    },
                    Err(err) => alert(&format!("Error: {}", err))
                }
            });
        })
    };

    let bookmark_callback = {
        let state = state.clone();
        let load_user_posts = load_user_posts.clone();
        Callback::from(move |(id, bookmark)| {
            let state = state.clone();
            let load_user_posts = load_user_posts.clone();

            wasm_bindgen_futures::spawn_local(async move {
                match state.api.bookmark_post(id, bookmark).await {
                    Ok(_) => {
                        load_user_posts.emit(());
                    },
                    Err(err) => alert(&format!("Error: {}", err))
                }
            });
        })
    };


    let posts = posts.deref().into_iter().map(|post| {
        html! {
            <Post
                post_id={post.post_id.clone()}
                title={post.title.clone()}
                description={post.description.clone()}
                delete_callback={delete_callback.clone()}
                bookmark_callback={bookmark_callback.clone()}
                is_bookmarked={post.bookmarked.clone()}
            />
        }
    }).collect::<Html>();

    let loading = loading.clone();
    html! {
        <>
            <div>
                <button onclick={sign_out}>
                    { "Sign Out" }
                </button>
            </div>
            <form onsubmit={add_post_callback}>
                <input ref={title} placeholder={"Title"}/>
                <textarea ref={description} placeholder={"Description"}></textarea>
                <button submit={"true"} class={"button"}>{"Submit"}</button>
            </form>
            if *loading {
                <p>{ "Loading.." }</p>
            } else {
                <div class={"posts"}>
                    { posts }
                </div>
            }
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

    let bookmark_callback = props.bookmark_callback.clone();
    let delete_callback = props.delete_callback.clone();
    let id = props.post_id.clone();
    let is_bookmarked = props.is_bookmarked.clone();
    html! {
        <>
            <div class={"post"}>
                <div class={"post_actions"}>
                    <button onclick={move |_| bookmark_callback.emit((id, !is_bookmarked))} class={"button"}>
                        if is_bookmarked {
                            {"Unset bookmark"}
                        } else {
                            {"Bookmark"}
                        }
                    </button>
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