use reqwasm::http;

pub async fn api_authorization_request(form_data:String, uri:String) -> Result<String, String> {
    log::debug!("Make request with data: {} and uri: {}", form_data, uri);
    let response = match http::Request::post(&format!("http://localhost:8000/auth/{}",uri))
        .header("Content-Type", "application/json")
        .body(form_data)
        .send()
        .await
    {
        Ok(res) => res,
        Err(e) => return Err(e.to_string()),
    };
    log::debug!("Response: {:?}", response);

    Ok("ok".to_string())
}