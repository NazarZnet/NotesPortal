use crate::api::api_authorization_request;

use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yew::{function_component, html, Html, Properties};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub title: String,
    pub btn_value: String,
    pub uri:String
}

#[function_component(AuthorizationForm)]
pub fn authorization_form(props:&Props)->Html {
    let username = use_state(String::new);
    let password = use_state(String::new);

    let username_input_ref = use_node_ref();
    let password_input_ref = use_node_ref();

    let handle_username = {
        let username = username.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            username.set(value);
        })
    };

    let handle_password = {
        let password = password.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target().unwrap();
            let value = target.unchecked_into::<HtmlInputElement>().value();
            password.set(value);
        })
    };

    let on_submit = {
        let username_ref = username_input_ref.clone();
        let password_ref = password_input_ref.clone();
        let uri=props.uri.clone();
        
        Callback::from(move |event: SubmitEvent| {
            let uri=uri.clone();
            let username=username.clone();
            let password=password.clone();

            let username_ref = username_ref.clone();
            let password_ref = password_ref.clone();

            event.prevent_default();

            let form_data = serde_json::json!({
                "username": username.to_string(),
                "password": password.to_string()
            });

            spawn_local(async move {
                let username_input = username_ref.cast::<HtmlInputElement>().unwrap();
                username_input.set_value("");
                username.set(String::new());

                let password_input = password_ref.cast::<HtmlInputElement>().unwrap();
                password_input.set_value("");
                password.set(String::new());
                

                let response = api_authorization_request(form_data.to_string(),uri).await;

                match response {
                    Ok(_) => {
                        log::info!("Success request!");
                    }
                    Err(e) => {
                        log::error!("Error request {}", e);
                    }
                }
            });
        })
    };

    html! {
        <div class="">
            <header class="">
                <h2 class="">{props.title.clone()}</h2>
            </header>
            <form onsubmit={on_submit}>
                <div class="">
                    <input
                        type="text"
                        ref={username_input_ref}
                        oninput={handle_username}
                        class=""
                        placeholder="username"
                    />
                </div>
                <div class="">
                    <input
                        type="password"
                        ref={password_input_ref}
                        oninput={handle_password}
                        class=""
                        placeholder="password"
                    />
                </div>
                <button
                    type="submit"
                    class=""
                    >
                    {props.btn_value.clone()}
                </button>
                
            </form>
        </div>
    }
}
