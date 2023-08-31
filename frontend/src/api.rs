use reqwasm::http;
use common::{ResponseUser, ErrorResponse};

pub async fn api_authorization_request(form_data:String, uri:String) -> Result<ResponseUser, ErrorResponse> {
    log::debug!("Make request with data: {} and uri: {}", form_data, uri);
    let response = match http::Request::post(&format!("http://localhost:8000/auth/{}",uri))
        .header("Content-Type", "application/json")
        .body(form_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(ErrorResponse::new(Some(e.to_string()),Some("Failed to make request".to_string()),common::ErrorTypes::RequestError))
    };
    log::debug!("Response: {:?}", response);
    if response.status()!=200{
        let error=response.json::<ErrorResponse>().await.unwrap_or_default();
        return Err(error);
    }

    let response_data=response.json::<ResponseUser>().await.map_err(|e|{
        log::error!("Failder to deserialize response!");
        ErrorResponse::new(Some(e.to_string()),Some("Failed to deserialize response".to_string()),common::ErrorTypes::RequestError)
    });
    log::debug!("Response data: {:?}", response_data);
    response_data
}