use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::alert::AlertComponent;
use crate::components::auth::form::AuthorizationForm;
use crate::components::auth::types::FormType;
use crate::components::posts_list::PostsList;
use crate::components::switchbutton::SwitchButton;

// use crate::components::page::AuthPage;
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
                <div class="container">
                    <h1>{ "Home" }</h1>
                    <SwitchButton text={"Sign Up"} route={Route::SignUp}/>
                </div>
            }
        }
        Route::SignUp => {
            html! {
                <div class="container">
                    <AuthorizationForm formtype={FormType::SignUp}>
                        <AlertComponent message="You’ve been signed up successfully!" route={Route::LogIn} />
                    </AuthorizationForm>
                </div>
            }
        }
        Route::LogIn => {
            html! {
                <div class="container">
                    <AuthorizationForm formtype={FormType::LogIn}>
                        <AlertComponent message="You’ve been signed in successfully!" route={Route::Posts}/>
                    </AuthorizationForm>
                </div>
            }
        }
        Route::Posts => {
            html! {
                <div class="container">
                    <h1>{ "Posts" }</h1>
                    <SwitchButton text={"Home"} route={Route::Home}/>
                    <PostsList/>
                </div>
            }
        }
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}
