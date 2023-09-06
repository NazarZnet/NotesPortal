
use common::ResponsePost;
use yew::prelude::*;
use common::PostsUpdateData;


#[derive(Properties, PartialEq)]
pub struct Props {
    pub post: ResponsePost,
    pub update_callback:Callback<PostsUpdateData>
}

#[function_component(PostItem)]
pub fn post_item(props: &Props) -> Html {
    let update_callback = props.update_callback.clone();
    let post_id = props.post.id;
    let mut post_important = props.post.important;
    
    let onclick = {
        //change important propertie
        post_important= !post_important;
        Callback::from(move |_e:MouseEvent| { 
            let update_data=PostsUpdateData{id:post_id,important:post_important};
            
            update_callback.emit(update_data);
            log::debug!("User clicked to the post");
        })
    };



    html! {
      <div class="post">
          <div class="post-header">
              <h2>{&props.post.title.clone()}</h2>
              {
                if props.post.important{
                    html!(<svg {onclick}stroke="currentColor" fill="yellow" stroke-width="0" viewBox="0 0 1024 1024" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M908.1 353.1l-253.9-36.9L540.7 86.1c-3.1-6.3-8.2-11.4-14.5-14.5-15.8-7.8-35-1.3-42.9 14.5L369.8 316.2l-253.9 36.9c-7 1-13.4 4.3-18.3 9.3a32.05 32.05 0 0 0 .6 45.3l183.7 179.1-43.4 252.9a31.95 31.95 0 0 0 46.4 33.7L512 754l227.1 119.4c6.2 3.3 13.4 4.4 20.3 3.2 17.4-3 29.1-19.5 26.1-36.9l-43.4-252.9 183.7-179.1c5-4.9 8.3-11.3 9.3-18.3 2.7-17.5-9.5-33.7-27-36.3z"></path></svg>)
                }else{
                    html!(<svg {onclick} stroke="currentColor" fill="white" stroke-width="0" viewBox="0 0 16 16" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M2.866 14.85c-.078.444.36.791.746.593l4.39-2.256 4.389 2.256c.386.198.824-.149.746-.592l-.83-4.73 3.523-3.356c.329-.314.158-.888-.283-.95l-4.898-.696L8.465.792a.513.513 0 00-.927 0L5.354 5.12l-4.898.696c-.441.062-.612.636-.283.95l3.523 3.356-.83 4.73zm4.905-2.767l-3.686 1.894.694-3.957a.565.565 0 00-.163-.505L1.71 6.745l4.052-.576a.525.525 0 00.393-.288l1.847-3.658 1.846 3.658a.525.525 0 00.393.288l4.052.575-2.906 2.77a.564.564 0 00-.163.506l.694 3.957-3.686-1.894a.503.503 0 00-.461 0z" clip-rule="evenodd"></path></svg>)
                }
              }
          </div >
          {
              if let Some(desc)=&props.post.description{
                  html!(<p class="post-desc">{desc.clone()}</p>)
              }else{
                  html!(<p class="post-desc">{"No description!"}</p>)
              }
          }

          <p class="post-date">{&props.post.created_at.date()}</p>
      </div>
    }
}
