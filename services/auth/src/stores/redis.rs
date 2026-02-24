use redis::AsyncCommands;
use uuid::Uuid;

use crate::types::AuthError;

pub type RedisConn = redis::aio::MultiplexedConnection;

// --- Refresh tokens ---

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

pub async fn get_refresh_token_user(
    conn: &mut RedisConn,
    token_hash: &str,
) -> Result<Option<Uuid>, AuthError> {
    let val: Option<String> = conn.get(format!("refresh:{token_hash}")).await?;
    Ok(val.and_then(|v| v.parse().ok()))
}

pub async fn delete_refresh_token(
    conn: &mut RedisConn,
    token_hash: &str,
) -> Result<(), AuthError> {
    let _: () = conn.del(format!("refresh:{token_hash}")).await?;
    Ok(())
}

// --- OAuth state (CSRF protection) ---

pub async fn store_oauth_state(
    conn: &mut RedisConn,
    state: &str,
    provider: &str,
    ttl_secs: u64,
) -> Result<(), AuthError> {
    let _: () = conn
        .set_ex(format!("oauth_state:{state}"), provider, ttl_secs)
        .await?;
    Ok(())
}

pub async fn get_and_delete_oauth_state(
    conn: &mut RedisConn,
    state: &str,
) -> Result<Option<String>, AuthError> {
    let key = format!("oauth_state:{state}");
    let val: Option<String> = conn.get(&key).await?;
    if val.is_some() {
        let _: () = conn.del(&key).await?;
    }
    Ok(val)
}