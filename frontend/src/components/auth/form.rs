use crate::api::api_authorization_request;
use crate::components::list_erors::ListErrors;


use log;
use yew_hooks::prelude::*;
use web_sys::HtmlInputElement;

use yew::prelude::*;
use yew_router::prelude::Link;
use yew::{function_component, html, Html, Properties};

use crate::routes::Route;
use super::types::{FormType,FormData,FormSettings};


#[derive(Properties, PartialEq)]
pub struct Props {
    pub formtype:FormType,
    pub children:Children
}

#[function_component(AuthorizationForm)]
pub fn authorization_form(props:&Props)->Html {
    let settings:FormSettings=props.formtype.into();

    let form_data=use_state(FormData::default);
    let validation_error=use_state(String::new);

    let api_request = {
        let form_data = form_data.clone();
        let request_type=props.formtype.clone();
        use_async(async move {
            let info=(*form_data).clone(); 
            api_authorization_request(info,request_type.to_string()).await
        })
    };
    

      
    {
        use_effect_with_deps(
            move |api_request| {
                if let Some(user_info) = &api_request.data {
                    // user_ctx.login(user_info.user.clone());
                    log::debug!("Userlogged in, {:?}",user_info);
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
        <div class="">
            {
                if let Some(_data)=&api_request.data{
                    html!(
                        for props.children.clone()
                    )
                }else{
                    html!()
                }
            }
            <header class="">
                <h2 class="">{settings.title.clone()}</h2>
            </header>
            <ListErrors error={api_request.error.clone()} />
            <form {onsubmit}>
                {
                    match props.formtype{
                        FormType::SignUp=>html!(
                            <div> 
                                <Link<Route> to={Route::LogIn}>
                                    { "Have an account?" }
                                </Link<Route>>
                            </div>),
                        FormType::LogIn=>html!(
                            <div> 
                                <Link<Route> to={Route::SignUp}>
                                    { "Don't have an account?" }
                                </Link<Route>>
                            </div>)
                    }
                }
                <div class="form_errors">{validation_error.to_string()}</div>
                <div class="">
                    <input
                        type="text"
                        oninput={oninput_username}
                        class=""
                        placeholder="username"
                        required=true
                    />
                </div>
                <div class="">
                    <input
                        type="password"
                        oninput={oninput_password}
                        class=""
                        placeholder="password"
                        required=true
                    />
                </div>
                <button type="submit" class="">
                    {settings.value.clone()}
                </button>
                
            </form>
        </div>
    }
}
