use reqwest;
use serde_json::json;

pub async fn send_discord_message(webhook_url: &str, content: &str) -> Result<(), reqwest::Error> {
    let payload = json!({
        "content": content,
        "username": "Alertmanager"
    });
    reqwest::Client::new()
        .post(webhook_url)
        .json(&payload)
        .send()
        .await?;
    Ok(())
}
