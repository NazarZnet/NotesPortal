use crate::api::request;
use crate::components::list_erors::ListErrors;
use common::{PostsFormData, ResponsePost};
use reqwasm::http::Method;

use web_sys::HtmlInputElement;
use yew_hooks::prelude::*;

use yew::prelude::*;
use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}
/// Component that renders a form for adding a new post, including
/// fields for title and description.
///
/// Arguments:
///
/// * `children`: Components that will be rendered when adding post requests is successful.
/// Returns:
///
/// Returns:
///
/// The `AddPostForm` component returns a HTML element that represents a form for adding a new post.
/// The form includes input fields for the title and description of the post, as well as a submit
/// button. The function also includes logic for handling user input and making an API request to add
/// the post.

#[function_component(AddPostForm)]
pub fn add_post_form(props: &Props) -> Html {
    let form_data = use_state(PostsFormData::default);

    let api_request = {
        let form_data = form_data.clone();
        use_async(async move {
            let data = (*form_data).clone();
            request::<PostsFormData, ResponsePost>(Method::POST, "/posts".to_owned(), Some(data))
                .await
        })
    };

    let oninput_title = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*form_data).clone();
            info.title = input.value();
            form_data.set(info);
        })
    };

    let oninput_description = {
        let form_data = form_data.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*form_data).clone();
            info.description = Some(input.value());
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
                <h2>{"Add new post!"}</h2>
            </header>
            <ListErrors error={api_request.error.clone()} />
            <form {onsubmit} class="form">
                <div class="form_field">
                    <label for="title">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M14 9a1 1 0 100-2 1 1 0 000 2zm0 1a2 2 0 100-4 2 2 0 000 4zM2 9a1 1 0 100-2 1 1 0 000 2zm0 1a2 2 0 100-4 2 2 0 000 4z" clip-rule="evenodd"></path><path fill-rule="evenodd" d="M1.5 2.5A1.5 1.5 0 013 1h10a1.5 1.5 0 011.5 1.5v4h-1v-4A.5.5 0 0013 2H3a.5.5 0 00-.5.5v4h-1v-4zm1 7v4a.5.5 0 00.5.5h10a.5.5 0 00.5-.5v-4h1v4A1.5 1.5 0 0113 15H3a1.5 1.5 0 01-1.5-1.5v-4h1z" clip-rule="evenodd"></path><path d="M11.434 4H4.566L4.5 5.994h.386c.21-1.252.612-1.446 2.173-1.495l.343-.011v6.343c0 .537-.116.665-1.049.748V12h3.294v-.421c-.938-.083-1.054-.21-1.054-.748V4.488l.348.01c1.56.05 1.963.244 2.173 1.496h.386L11.434 4z"></path></svg>
                        {"Title:"}
                    </label>
                    <input
                        type="text"
                        name="title"
                        oninput={oninput_title}
                        class=""
                        placeholder="Have to create Actix Web API server"
                        required=true
                    />
                </div>
                <div class="form_field">
                    <label for="description">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="1em" width="1em" xmlns="http://www.w3.org/2000/svg"><path d="M14 2H6c-1.1 0-1.99.9-1.99 2L4 20c0 1.1.89 2 1.99 2H18c1.1 0 2-.9 2-2V8l-6-6zm2 16H8v-2h8v2zm0-4H8v-2h8v2zm-3-5V3.5L18.5 9H13z"></path></svg>
                        {"Description:"}
                    </label>
                    <textarea
                        type="text"
                        oninput={oninput_description}
                        class=""
                        rows="6"
                        name="description"
                        placeholder="If you want to use Rust..."
                    />
                </div>
                <button type="submit" class="form_btn">
                    {"Add Post"}
                </button>

            </form>
        </div>
    </div>
    }
}
