use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::use_navigator;

use super::PostItem;
use crate::{api::get_posts_request, routes::Route,components::list_erors::ListErrors};

#[function_component(PostsList)]
pub fn posts_list() -> Html {
    let api_request = { use_async(async move { get_posts_request("/posts".to_string()).await }) };
    let navigator = use_navigator();
    let go_to_login = {
        Callback::from(move |_| {
            if let Some(navigation) = &navigator {
                navigation.push(&Route::LogIn);
            } else {
                log::error!("Navigator doesn't work!");
            }
        })
    };
    //sand api request when page is loading
    {
        let api_request = api_request.clone();
        use_effect_once(move || {
            api_request.run();
            || log::debug!("Request started!")
        })
    }
    {
        use_effect_with_deps(
            //check if response is Auth(Authorization) or Auth(Authentication) error then navigate to login page
            move |request| {
                if let Some(error) = &request.error {
                    match &error.error_type {
                        common::ErrorTypes::Auth(_e) => {
                            log::error!("User not authorized!");
                            go_to_login.emit(())
                        },
                        _ => {}
                    }
                }
            },
            api_request.clone(),
        )
    }

    html! {
        <div>
            <ListErrors error={api_request.error.clone()} />
           {
            if let Some(posts)=&api_request.data{
                html!(
                {for posts.iter().map(|post|html!(<PostItem post={post.clone()}/>))}
                )
            }else{
                html!(
                    <div>
                    <h1>{"Haven't gotten any posts yet!"}</h1>
                    </div>
                )
            }
           }
        </div>
    }
}
