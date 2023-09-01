use reqwest;
use common::{ResponseUser, ErrorResponse};
use serde::{Serialize,de::DeserializeOwned};

use once_cell::sync::Lazy;

static API_ROOT: Lazy<String> = Lazy::new(|| {
    dotenv::var("API_ROOT").unwrap_or("http://127.0.0.1:8000".to_string())
});

pub async fn api_authorization_request<T:Serialize+std::fmt::Debug>(form_data:T, uri:String) -> Result<ResponseUser, ErrorResponse> {
    log::debug!("Make request with data: {:?} and uri: {}", form_data, uri);
    //make post request to the API server
    let response_data=request(reqwest::Method::POST, uri, form_data).await;
    log::debug!("Response data: {:?}", response_data);
    response_data
}


pub async fn request<B, T>(method: reqwest::Method, uri: String, body: B) -> Result<T, ErrorResponse>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    let url = format!("{}{}", API_ROOT.clone(), uri);
    log::debug!("uri{}",url);
    let builder = reqwest::Client::new()
        .request(method, url)
        .header("Content-Type", "application/json")
        .json(&body);


    let response = builder.send().await;

    match response {
        Ok(data)=>{
            if data.status().is_success() {
                //deserialize object or return error
                data.json::<T>().await.map_err(|e|{
                    log::error!("Failder to deserialize response!");
                    ErrorResponse::new(Some(e.to_string()),Some("Failed to deserialize response".to_string()),common::ErrorTypes::DeserializeError)
                })
            } else {
                //deserialize to ErrorResponse
                log::debug!("Response status is not 200!");
                let error=data.json::<ErrorResponse>().await.unwrap_or_default();
                Err(error)
            }
        },
        Err(e) =>{
            log::debug!("Error sending the request!");
            Err(ErrorResponse::new(Some(e.to_string()),Some("Failed to make request".to_string()),common::ErrorTypes::RequestError))
        }
    }
    
}