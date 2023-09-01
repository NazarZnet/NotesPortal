use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::Route;
#[derive(Properties,PartialEq)]
pub struct Props {
    pub message: String,
    pub route: Route
}

#[function_component(AlertComponent)]
pub fn alert_component(props: &Props) -> Html {
    html! {
        <div class="">
            <div>
                <p class="">
                    {props.message.clone()}
                </p>
            </div>
            <button >
                {
                    match props.route{
                        Route::LogIn=>html!(
                            <Link<Route> to={props.route.clone()}>
                                { "Log In" }
                            </Link<Route>>),
                        Route::Posts=>html!(
                            <Link<Route> to={props.route.clone()}>
                                { "Go to posts" }
                            </Link<Route>>),
                        _=>html!()
                    }
                }
            </button>

        </div>
    }
}
