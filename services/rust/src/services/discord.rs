use crate::types::ApiError;

const DISCORD_API: &str = "https://discord.com/api/v10";

#[derive(serde::Deserialize)]
struct DmChannel {
    id: String,
}

/// Send a direct message to a Discord user via a bot token.
/// Opens (or reuses) the bot↔user DM channel, then posts `content` to it.
pub async fn send_dm(bot_token: &str, user_id: &str, content: &str) -> Result<(), ApiError> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| ApiError::Internal(format!("failed to build http client: {e}")))?;

    let auth = format!("Bot {bot_token}");

    // 1. Open the DM channel with the recipient.
    let channel: DmChannel = client
        .post(format!("{DISCORD_API}/users/@me/channels"))
        .header("Authorization", &auth)
        .json(&serde_json::json!({ "recipient_id": user_id }))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("discord dm channel request failed: {e}")))?
        .error_for_status()
        .map_err(|e| ApiError::Internal(format!("discord dm channel error: {e}")))?
        .json()
        .await
        .map_err(|e| ApiError::Internal(format!("discord dm channel decode failed: {e}")))?;

    // 2. Post the message to that channel.
    client
        .post(format!("{DISCORD_API}/channels/{}/messages", channel.id))
        .header("Authorization", &auth)
        .json(&serde_json::json!({ "content": content }))
        .send()
        .await
        .map_err(|e| ApiError::Internal(format!("discord message request failed: {e}")))?
        .error_for_status()
        .map_err(|e| ApiError::Internal(format!("discord message error: {e}")))?;

    Ok(())
}
