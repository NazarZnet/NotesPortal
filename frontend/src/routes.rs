use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::switchbutton::SwitchButton;
use crate::components::authorizationform::AuthorizationForm;
#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/signup")]
    SignUp,
    #[at("/login")]
    LogIn,
    #[at("/posts")]
    Posts,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { 
                <div>
                    <h1>{ "Home" }</h1>
                    <SwitchButton text={"Sign Up"} route={Route::SignUp}/> 
                </div>
            }
        }
        Route::SignUp => {
            html! { 
                <div>
                    <h1>{ "Sign Up" }</h1>
                    <AuthorizationForm title={"Create new account"} btn_value={"SignUp"} uri={"signup"}/>
                    <SwitchButton text={"Log In"} route={Route::LogIn}/> 
                </div>
            }
        }
        Route::LogIn => {
            html! { 
                <div>
                    <h1>{ "LogIn" }</h1>
                    <AuthorizationForm title={"Log to your account"} btn_value={"LogIn"} uri={"login"}/>
                    <SwitchButton text={"Posts"} route={Route::Posts}/> 
                </div>
            }
        }
        Route::Posts => {
            html! { 
                <div>
                    <h1>{ "Posts" }</h1>
                    <SwitchButton text={"Home"} route={Route::Home}/> 
                </div>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
