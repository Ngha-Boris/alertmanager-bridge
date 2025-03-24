mod alert;
mod config;
mod discord;

use crate::alert::AlertmanagerPayload;
use crate::config::get_discord_webhook_url;
use crate::discord::send_discord_message;
use actix_web::{App, HttpResponse, HttpServer, Responder, web};

async fn handle_webhook(payload: web::Json<AlertmanagerPayload>) -> impl Responder {
    let message = build_discord_message(&payload);
    let discord_webhook_url = get_discord_webhook_url();
    match send_discord_message(&discord_webhook_url, &message).await {
        Ok(_) => HttpResponse::Ok().body("OK"),
        Err(e) => {
            eprintln!("Error sending to Discord: {}", e);
            HttpResponse::InternalServerError().body("Error sending to Discord")
        }
    }
}

fn build_discord_message(payload: &AlertmanagerPayload) -> String {
    let mut message = String::from("Alerts:\n");
    for alert in &payload.alerts {
        let alert_name = &alert.labels.alertname;
        let status = &alert.status;
        let summary = alert.annotations.summary.as_deref().unwrap_or("No summary");
        let description = alert
            .annotations
            .description
            .as_deref()
            .unwrap_or("No description");
        message.push_str(&format!(
            "- {}: {}\nSummary: {}\nDescription: {}\n",
            alert_name, status, summary, description
        ));
    }
    message
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::JsonConfig::default().limit(20000))
            .route("/webhook", web::post().to(handle_webhook))
    })
    .bind("0.0.0.0:9094")?
    .run()
    .await
}
