use actix_web::{post, web, HttpResponse, Error};
use log::{error, info};
use reqwest::{Client, header};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct UrlForm {
    pub url: String,
}

#[post("/convert")]
pub async fn convert_url(form: web::Json<UrlForm>) -> Result<HttpResponse, Error> {
    let url = &form.url;

    info!("Received URL: {}", url);

    // Create an async reqwest client with default headers
    let client = Client::builder()
        .default_headers(get_default_headers())
        .build()
        .unwrap();

    let response = match client.get(url).send().await {
        Ok(resp) => resp,
        Err(e) => {
            error!("Error fetching URL: {}", e);
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error fetching URL: {}", e)));
        }
    };

    let html_content = match response.text().await {
        Ok(text) => text,
        Err(e) => {
            error!("Error reading response text: {}", e);
            return Ok(HttpResponse::InternalServerError()
                .body(format!("Error reading response text: {}", e)));
        }
    };

    // Convert HTML to Markdown
    let md_content = html2md::parse_html(&html_content);

    // Return the Markdown content as JSON
    Ok(HttpResponse::Ok().json(serde_json::json!({ "markdown": md_content })))
}

fn get_default_headers() -> header::HeaderMap {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) \
            Chrome/93.0.4577.82 Safari/537.36"),
    );
    headers.insert(
        header::ACCEPT_LANGUAGE,
        header::HeaderValue::from_static("en-US,en;q=0.9"),
    );
    headers.insert(
        header::ACCEPT,
        header::HeaderValue::from_static("text/html,application/xhtml+xml,\
            application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8"),
    );
    headers
}