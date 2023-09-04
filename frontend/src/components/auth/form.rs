use crate::api::api_authorization_request;
use crate::components::list_erors::ListErrors;

use log;
use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;

use yew::prelude::*;
use yew::{function_component, html, Html, Properties};
use yew_router::prelude::Link;

use super::types::{FormData, FormSettings, FormType};
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub formtype: FormType,
    pub children: Children,
}

#[function_component(AuthorizationForm)]
pub fn authorization_form(props: &Props) -> Html {
    let settings: FormSettings = props.formtype.into();

    let form_data = use_state(FormData::default);
    let validation_error = use_state(String::new);

    let api_request = {
        let form_data = form_data.clone();
        let request_type = props.formtype.clone();
        use_async(async move {
            let info = (*form_data).clone();
            api_authorization_request(info, request_type.to_string()).await
        })
    };

    {
        use_effect_with_deps(
            move |api_request| {
                if let Some(user_info) = &api_request.data {
                    // user_ctx.login(user_info.user.clone());
                    log::debug!("Userlogged in, {:?}", user_info);
                }
                || ()
            },
            api_request.clone(),
        );
    }

    let oninput_username = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*form_data).clone();
            info.username = input.value();
            form_data.set(info);
        })
    };

    let oninput_password = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*form_data).clone();
            info.password = input.value();
            form_data.set(info);
        })
    };

    let onsubmit = {
        let api_request = api_request.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default(); /* Prevent event propagation */
            api_request.run();
        })
    };

    html! {
        <div class="form_container">
            {
                if let Some(_data)=&api_request.data{
                    html!(
                        for props.children.clone()
                    )
                }else{
                    html!()
                }
            }
        <div class="form_box">

            <header class="form_box_title">
                <h2>{settings.title.clone()}</h2>
            </header>
            <ListErrors error={api_request.error.clone()} />
            <form {onsubmit} class="form">
                {
                    match props.formtype{
                        FormType::SignUp=>html!(
                            <div class="form_route">
                                <Link<Route> to={Route::LogIn}>
                                    { "Have an account?" }
                                </Link<Route>>
                            </div>),
                        FormType::LogIn=>html!(
                            <div class="form_route">
                                <Link<Route> to={Route::SignUp}>
                                    { "Don't have an account?" }
                                </Link<Route>>
                            </div>)
                    }
                }
                <div class="form_errors">{validation_error.to_string()}</div>
                <div class="form_field">
                    <label for="username">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M13.468 12.37C12.758 11.226 11.195 10 8 10s-4.757 1.225-5.468 2.37A6.987 6.987 0 008 15a6.987 6.987 0 005.468-2.63z"></path><path fill-rule="evenodd" d="M8 9a3 3 0 100-6 3 3 0 000 6z" clip-rule="evenodd"></path><path fill-rule="evenodd" d="M8 1a7 7 0 100 14A7 7 0 008 1zM0 8a8 8 0 1116 0A8 8 0 010 8z" clip-rule="evenodd"></path></svg>
                        {"Username:"}
                    </label>
                    <input
                        type="text"
                        name="username"
                        oninput={oninput_username}
                        class=""
                        placeholder="AdminPro"
                        required=true
                    />
                </div>
                <div class="form_field">
                    <label for="password">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><g><path fill="none" d="M0 0h24v24H0z"></path><path d="M18 8h2a1 1 0 0 1 1 1v12a1 1 0 0 1-1 1H4a1 1 0 0 1-1-1V9a1 1 0 0 1 1-1h2V7a6 6 0 1 1 12 0v1zm-2 0V7a4 4 0 1 0-8 0v1h8zm-5 6v2h2v-2h-2zm-4 0v2h2v-2H7zm8 0v2h2v-2h-2z"></path></g></svg>
                        {"Password:"}
                    </label>
                    <input
                        type="password"
                        oninput={oninput_password}
                        class=""
                        name="password"
                        placeholder="SecretPas196"
                        required=true
                    />
                </div>
                <button type="submit" class="form_btn">
                    {settings.value.clone()}
                </button>

            </form>
        </div>
    </div>
    }
}
