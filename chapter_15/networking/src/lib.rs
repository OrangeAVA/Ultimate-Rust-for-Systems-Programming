use reqwest;
use wasm_bindgen::prelude::*;

async fn perform_get_request(url: &str) -> Result<String, reqwest::Error> {
    let response = reqwest::get(url).await?;
    let body = response.text().await?;
    Ok(body)
}

#[wasm_bindgen]
pub async fn get_request(url: &str) -> JsValue {
    match perform_get_request(url).await {
        Ok(response_body) => JsValue::from_str(&response_body),
        Err(error) => JsValue::from_str(&format!("Error: {:?}", error)),
    }
}
