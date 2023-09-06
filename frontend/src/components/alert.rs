use yew::prelude::*;
use yew_router::prelude::Link;

use crate::routes::Route;
#[derive(Properties, PartialEq)]
pub struct Props {
    pub message: String,
    pub route: Route,
}

/// The `AlertComponent` function renders an alert message with a
/// button that links to different routes based on the `route` argument.
///
/// Arguments:
///
/// * `route`: The route when user will redirect after clicking the button
///
/// * `message`: The message that alert shows
///
/// Returns:
///
/// The `AlertComponent` returns a HTML element.
#[function_component(AlertComponent)]
pub fn alert_component(props: &Props) -> Html {
    html! {
        <div class="alert">
            <div class="alert_message">
                <h1>
                    {props.message.clone()}
                </h1>
            </div>
            <button class="alert_btn">
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
                        Route::Home=>html!(
                            <Link<Route> to={props.route.clone()}>
                                { "Home" }
                            </Link<Route>>
                        ),
                        _=>html!()
                    }
                }
            </button>

        </div>
    }
}
