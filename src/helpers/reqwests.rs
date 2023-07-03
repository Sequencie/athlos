/// Core crates
use reqwest;
use json::object

pub struct Parameters {
    method: String,
    body: String,
    url: String
}
/// To be used in creating reqwest clients and processing responses.
pub async fn send_get(params: Parameters) -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let response = client.get(params.url).await.expect("HR0-0").text.expect("HR0-1");
    return response;
}
