use std::time::Duration;

use redis::AsyncCommands;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::types::AuthError;

pub type RedisConn = redis::aio::ConnectionManager;

// Background heartbeat: pings Redis on a fixed interval and logs a single
// line on each transition between healthy and unreachable. ConnectionManager
// retries internally, so this loop is observability only.
pub async fn run_health_loop(mut conn: RedisConn, interval: Duration) {
    let mut healthy = true;
    let mut ticker = tokio::time::interval(interval);
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        ticker.tick().await;
        let result = redis::cmd("PING")
            .query_async::<redis::Value>(&mut conn)
            .await;

        match result {
            Ok(_) if !healthy => {
                tracing::info!("redis connection recovered");
                healthy = true;
            }
            Ok(_) => {}
            Err(err) if healthy => {
                tracing::warn!("redis connection lost: {err}");
                healthy = false;
            }
            Err(err) => {
                tracing::debug!("redis still unreachable: {err}");
            }
        }
    }
}

// --- Refresh tokens ---

#[tracing::instrument(name = "redis.store_refresh_token", skip_all)]
pub async fn store_refresh_token(
    conn: &mut RedisConn,
    token_hash: &str,
    user_id: Uuid,
    ttl_secs: u64,
) -> Result<(), AuthError> {
    let _: () = conn
        .set_ex(
            format!("refresh:{token_hash}"),
            user_id.to_string(),
            ttl_secs,
        )
        .await?;
    Ok(())
}

#[tracing::instrument(name = "redis.get_refresh_token_user", skip_all)]
pub async fn get_refresh_token_user(
    conn: &mut RedisConn,
    token_hash: &str,
) -> Result<Option<Uuid>, AuthError> {
    let val: Option<String> = conn.get(format!("refresh:{token_hash}")).await?;
    Ok(val.and_then(|v| v.parse().ok()))
}

#[tracing::instrument(name = "redis.delete_refresh_token", skip_all)]
pub async fn delete_refresh_token(
    conn: &mut RedisConn,
    token_hash: &str,
) -> Result<(), AuthError> {
    let _: () = conn.del(format!("refresh:{token_hash}")).await?;
    Ok(())
}

// --- OAuth state (CSRF protection) ---
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthState {
    pub provider: String,
    #[serde(default)]
    pub is_cli: bool,
}

#[tracing::instrument(name = "redis.store_oauth_state", skip_all)]
pub async fn store_oauth_state(
    conn: &mut RedisConn,
    state: &str,
    provider: &str,
    is_cli: bool,
    ttl_secs: u64,
) -> Result<(), AuthError> {
    let payload = serde_json::to_string(&OAuthState {
        provider: provider.to_string(),
        is_cli,
    })
    .map_err(|e| AuthError::Internal(e.to_string()))?;
    let _: () = conn
        .set_ex(format!("oauth_state:{state}"), payload, ttl_secs)
        .await?;
    Ok(())
}

#[tracing::instrument(name = "redis.get_and_delete_oauth_state", skip_all)]
pub async fn get_and_delete_oauth_state(
    conn: &mut RedisConn,
    state: &str,
) -> Result<Option<OAuthState>, AuthError> {
    let key = format!("oauth_state:{state}");
    let val: Option<String> = conn.get(&key).await?;
    let Some(raw) = val else {
        return Ok(None);
    };
    let _: () = conn.del(&key).await?;

    let parsed = serde_json::from_str::<OAuthState>(&raw).unwrap_or(OAuthState {
        provider: raw,
        is_cli: false,
    });
    Ok(Some(parsed))
}
