use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::*;

use crate::routes;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    #[prop_or(routes::Route::Home)]
    pub route: routes::Route,
}

#[function_component(SwitchButton)]
pub fn switch_button(props: &Props) -> Html {
    let navigator = use_navigator().unwrap();

    let route = props.route.clone();
    let onclick = Callback::from(move |_| navigator.push(&route));
    html! {
        <div>
            <button {onclick}>{ props.text.clone() }</button>
        </div>
    }
}
