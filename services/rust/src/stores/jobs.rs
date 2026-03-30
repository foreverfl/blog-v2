use crate::types::ApiError;

const JOB_TTL_SECS: i64 = 86400; // 24h

/// Store a job status in Redis with 24h TTL.
pub async fn set(
    redis: &redis::Client,
    prefix: &str,
    job_id: &str,
    payload: &serde_json::Value,
) -> Result<(), ApiError> {
    let key = format!("import:{prefix}:{job_id}");
    let mut conn = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| ApiError::Internal(format!("redis connection failed: {e}")))?;
    let _: () = redis::cmd("SETEX")
        .arg(&key)
        .arg(JOB_TTL_SECS)
        .arg(payload.to_string())
        .query_async(&mut conn)
        .await
        .map_err(|e| ApiError::Internal(format!("redis write failed: {e}")))?;
    Ok(())
}

/// Best-effort store (for background tasks where we can't propagate errors).
pub async fn set_silent(redis: &redis::Client, prefix: &str, job_id: &str, payload: &serde_json::Value) {
    if let Ok(mut conn) = redis.get_multiplexed_async_connection().await {
        let key = format!("import:{prefix}:{job_id}");
        let _: Result<(), _> = redis::cmd("SETEX")
            .arg(&key)
            .arg(JOB_TTL_SECS)
            .arg(payload.to_string())
            .query_async(&mut conn)
            .await;
    }
}

/// Get a job status from Redis. Tries multiple prefixes in order.
pub async fn get(
    redis: &redis::Client,
    job_id: &str,
    prefixes: &[&str],
) -> Result<Option<serde_json::Value>, ApiError> {
    let mut conn = redis
        .get_multiplexed_async_connection()
        .await
        .map_err(|e| ApiError::Internal(format!("redis connection failed: {e}")))?;

    for prefix in prefixes {
        let key = format!("import:{prefix}:{job_id}");
        let result: Option<String> = redis::cmd("GET")
            .arg(&key)
            .query_async(&mut conn)
            .await
            .map_err(|e| ApiError::Internal(format!("redis read failed: {e}")))?;
        if let Some(data) = result {
            let value: serde_json::Value = serde_json::from_str(&data)
                .map_err(|e| ApiError::Internal(format!("failed to parse job data: {e}")))?;
            return Ok(Some(value));
        }
    }

    Ok(None)
}
