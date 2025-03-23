pub fn get_discord_webhook_url() -> String {
    std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL must be set")
}
