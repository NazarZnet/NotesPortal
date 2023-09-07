use common::ApiResponse;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::use_navigator;

use crate::{api::request, routes::Route};
use reqwasm::http::Method;

/// Component that renders a "Log Out" button and handles the
/// logout functionality by making an API request and navigating to the login page if the request is
/// successful.
#[function_component(LogOut)]
pub fn logout_button() -> Html {
    let api_request = {
        use_async(async move {
            request::<(), ApiResponse>(Method::GET, "/auth/logout".to_string(), None).await
        })
    };
    let onclick = {
        let api_request = api_request.clone();
        Callback::from(move |_| api_request.run())
    };
    let navigator = use_navigator();

    {
        use_effect_with_deps(
            //check if response is Auth(Authorization) or Auth(Authentication) error then navigate to login page
            move |request| {
                if request.data.is_some() {
                    if let Some(navigatio) = &navigator {
                        log::debug!("Log out");
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
