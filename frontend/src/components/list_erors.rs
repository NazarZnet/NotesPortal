use yew::prelude::*;

use common::{ErrorTypes, ErrorResponse};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub error: Option<ErrorResponse>,
}

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
