use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use reqwasm::http::Method;
use crate::{api::request, routes::Route};

#[function_component(LogOut)]
pub fn logout_button() -> Html {
    let api_request = { use_async(async move { request::<(),()>(Method::GET,"/auth/logout".to_string(),None).await }) };
    let onclick={
        let api_request=api_request.clone();
        Callback::from(move|_|api_request.run())
    };
    let navigator = use_navigator();
    
    {
        use_effect_with_deps(
            //check if response is Auth(Authorization) or Auth(Authentication) error then navigate to login page
            move |request| {
                if let Some(_) = &request.data {
                   if let Some(navigatio)=&navigator{
                    navigatio.push(&Route::Home)
                   }
                }
            },
            api_request.clone(),
        )
    }
    

    html! {
        <div>
            <button class="link" {onclick}>{"Log Out"}</button>
        </div>
    }
}