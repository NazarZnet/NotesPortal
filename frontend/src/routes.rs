use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::alert::AlertComponent;
use crate::components::auth::form::AuthorizationForm;
use crate::components::auth::logout::LogOut;
use crate::components::auth::types::FormType;
use crate::components::posts::AddPostForm;
use crate::components::posts::PostsList;

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
/// The `switch` function takes a `Route` enum as input and returns corresponding HTML based on the
/// route.
///
/// Arguments:
///
/// * `routes`: The `routes` parameter is of type `Route`, which is an enum representing different
/// routes in the application. The `switch` function takes this parameter and returns an HTML element
/// based on the value of the `routes` parameter. The function uses pattern matching to determine which
/// route is currently active and renders
///
/// Returns:
///
/// The `switch` function returns an `Html` element based on the value of the `routes` parameter. The
/// returned `Html` element represents the corresponding view for the given route.

pub fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! {
                <div class="container">
                    <div class="flex-container">
                        <h1>{ "Home" }</h1>
                        <button class="link">
                            <Link<Route> to={Route::SignUp}>
                                    { "Sign Up" }
                            </Link<Route>>
                        </button>
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
                        <button class="link">
                        <Link<Route> to={Route::Home}>
                                { "Home" }
                        </Link<Route>>
                        </button>

                        <LogOut/>

                        <button class="link">
                        <Link<Route> to={Route::AddPost}>
                                { "Add Post" }
                        </Link<Route>>
                        </button>
                    </div>
                    <PostsList/>
                </div>
            }
        }
        Route::AddPost => {
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
