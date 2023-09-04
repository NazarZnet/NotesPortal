use yew::prelude::*;
use yew_hooks::use_async;

use crate::api::get_posts_request;

#[function_component(PostsList)]
pub fn posts_list() -> Html {
    let api_request = { use_async(async move { get_posts_request("/posts".to_string()).await }) };
    let onclick = {
        let api_request = api_request.clone();
        Callback::from(move |_| {
            api_request.run();
        })
    };
    html! {
        <div>
            <button {onclick}>{"Get Posts"}</button>
            {
                if let Some(posts)=&api_request.data{
                    html!(
                        <div>
                            { for posts.iter().map(|post| html!(<div>{post.title.clone()}</div>) )}
                        </div>)
                }else{
                    html!()
                }
            }
        </div>
    }
}
