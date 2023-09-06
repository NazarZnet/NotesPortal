use reqwasm::http;

use common::ErrorResponse;
use serde::{de::DeserializeOwned, Serialize};

use once_cell::sync::Lazy;

//get root part of request address from .env file
static API_ROOT: Lazy<String> =
    Lazy::new(|| dotenv::var("API_ROOT").unwrap_or("http://127.0.0.1:8000".to_string()));

/// The `request` function in Rust is an asynchronous function that sends an HTTP request, handles the
/// response, and returns either the deserialized response object or an error response.
///
/// Arguments:
///
/// * `method`: The `method` parameter is the HTTP method to be used for the request, such as GET, POST,
/// PUT, DELETE, etc.
/// * `uri`: The `uri` parameter is a string that represents the path or endpoint of the API you want to
/// make a request to. It is appended to the `API_ROOT` value to form the full URL address for the
/// request.
/// * `body`: The `body` parameter is an optional parameter that represents the request body. It can be
/// of any type that implements the `Serialize` trait, which allows it to be converted into a JSON
/// string. The `Serialize` trait is typically implemented for structs or enums using the `serde`
/// library.
///
/// Returns:
///
/// a `Result` type, where the success case contains a value of type `T` (which must implement the
/// `DeserializeOwned` trait, be `'static`, and implement `std::fmt::Debug`), and the error case
/// contains an `ErrorResponse` object.
pub async fn request<B, T>(
    method: http::Method,
    uri: String,
    body: Option<B>,
) -> Result<T, ErrorResponse>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    //create full url address
    let url = format!("{}{}", API_ROOT.clone(), uri);
    log::debug!("Make request: {}", url);

    let mut builder = http::Request::new(&url)
        .method(method)
        .credentials(http::RequestCredentials::Include) //add cookies
        .header("Content-Type", "application/json");

    //add data if exists
    if let Some(data) = body {
        let data = serde_json::to_string(&data).unwrap_or_default();
        builder = builder.body(data);
    }

    let response = builder.send().await;

    match response {
        Ok(data) => {
            if data.status() == 200 {
                //deserialize object or return error
                data.json::<T>().await.map_err(|e| {
                    log::error!("Failder to deserialize response!");
                    ErrorResponse::new(
                        Some(e.to_string()),
                        Some("Failed to deserialize response".to_string()),
                        common::ErrorTypes::DeserializeError,
                    )
                })
            } else {
                //deserialize to ErrorResponse
                log::debug!("Response status is not 200!");
                let error = data.json::<ErrorResponse>().await.unwrap_or_default();
                Err(error)
            }
        }
        Err(e) => {
            log::debug!("Error sending the request!");
            Err(ErrorResponse::new(
                Some(e.to_string()),
                Some("Failed to make request".to_string()),
                common::ErrorTypes::RequestError,
            ))
        }
    }
}
