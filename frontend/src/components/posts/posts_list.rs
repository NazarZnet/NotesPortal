use common::PostsUpdateData;
use yew::prelude::*;
use yew_hooks::{use_async, use_effect_once};
use yew_router::prelude::use_navigator;

use super::PostItem;
use crate::{api::{get_posts_request, update_post_request}, routes::Route,components::list_erors::ListErrors};

#[function_component(PostsList)]
pub fn posts_list() -> Html {
    let update_post_data=use_state(PostsUpdateData::default);

    //update post important field request
    let update_api_request = {
        let data=update_post_data.clone();
        use_async(async move {
            let update_post=(*data).clone();
            update_post_request(update_post, "/posts/update".to_owned()).await
        })
    };

    //main api requst 
    let api_request = { use_async(async move { get_posts_request("/posts".to_string()).await }) };
   
    //navigate to login page 
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
            || log::debug!("Get posts request started!")
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


    //main update callback that moves to children
    let update_post={
        let api_request=api_request.clone();
        Callback::from(move |post:PostsUpdateData|{
            //set new data for update request
            update_post_data.set(post);
            //run api update request
            update_api_request.run();
            //run main api request to get updated posts list
            api_request.run();
        })
    };

   
    

    html! {
        <div>
            <ListErrors error={api_request.error.clone()} />
           {
            if let Some(posts)=&api_request.data{
                html!(
                {for posts.iter().map(|post|html!(<PostItem post={post.clone()} update_callback={update_post.clone()}/>))}
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
