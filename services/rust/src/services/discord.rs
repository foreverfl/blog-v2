use crate::types::ApiError;

/// Post `content` to a channel via an incoming webhook URL.
/// No bot token or DM channel needed — the URL itself carries the auth.
#[tracing::instrument(name = "discord.send_webhook", skip_all)]
pub async fn send_webhook(webhook_url: &str, content: &str) -> Result<(), ApiError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(format!("failed to build http client: {e}")))?;

    client
        .post(webhook_url)
        .json(&serde_json::json!({ "content": content }))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("discord webhook request failed: {e}")))?
        .error_for_status()
        .map_err(|e| ApiError::Internal(format!("discord webhook error: {e}")))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    // Live check — posts to the real channel, so ignored by default.
    // Run: DISCORD_COMMENTS_WEBHOOK=<url> cargo test send_webhook -- --ignored
    #[tokio::test]
    #[ignore]
    async fn send_webhook_posts_to_channel() {
        let url = std::env::var("DISCORD_COMMENTS_WEBHOOK")
            .expect("set DISCORD_COMMENTS_WEBHOOK to run this test");
        super::send_webhook(&url, "webhook self-check: hello from blog-rust-api")
            .await
            .expect("webhook post failed");
    }
}
