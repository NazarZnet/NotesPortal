use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::alert::AlertComponent;
use crate::components::auth::form::AuthorizationForm;
use crate::components::auth::types::FormType;
use crate::components::posts::PostsList;
use crate::components::switchbutton::SwitchButton;
use crate::components::auth::logout::LogOut;
use crate::components::posts::AddPostForm;


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
    #[at("/posts/add")]
    AddPost,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <div class="container">
                    <div class="flex-container">
                        <h1>{ "Home" }</h1>
                        <SwitchButton text={"Sign Up"} route={Route::SignUp}/>
                    </div>
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
                    <div class="flex-container">
                        <SwitchButton text={"Home"} route={Route::Home}/>
                        <LogOut/>

                        <SwitchButton text={"Add Post"} route={Route::AddPost}/>
                    </div>
                    <PostsList/>
                </div>
            }
        }
        Route::AddPost=> {
            html! {
                <div class="container">
                    <AddPostForm>
                        <AlertComponent message="Pos added successfully!" route={Route::Posts}/>
                    </AddPostForm>
                </div>
            }
        }
        Route::NotFound => html! { 
            <div class="container">
                <AlertComponent message="404! Page not found!" route={Route::Home}/>
            </div>
         },
    }
}
