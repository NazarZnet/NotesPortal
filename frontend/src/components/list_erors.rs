use yew::prelude::*;

use common::{ErrorResponse, ErrorTypes};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: Option<ErrorResponse>,
}

/// The `ListErrors` components renders a list of error messages.
///
/// Arguments:
///
/// * `error`: Optinal argument that has type `ErrorResponse`. Shows only Auth and Validation erros
///
/// Returns:
///
/// The `ListErros` function returns a HTML element.
#[function_component(ListErrors)]
pub fn list_errors(props: &Props) -> Html {
    if let Some(error) = &props.error {
        html! {
            <ul class="error_messages">
                {
                    match error.error_type{
                        ErrorTypes::Auth(_)=>html!(
                         <li> {error.message.clone()}</li>
                        ),
                        ErrorTypes::ValidationError=>html!(
                            <li> {error.message.clone()}</li>
                           ),
                        _=>html!()
                    }
                }
            </ul>
        }
    } else {
        html!()
    }
}
