use std::time::{Duration, SystemTime};

use aws_sdk_s3::presigning::PresigningConfig;

use crate::types::ApiError;

/// How long a signed link stays valid. Two hours so a link minted at the very
/// end of its window still has an hour of life left.
const LIFETIME: Duration = Duration::from_secs(2 * 60 * 60);

/// The window the signing time is snapped to.
const WINDOW: u64 = 60 * 60;

/// Round a time down to the start of its window.
///
/// Signing from the exact current time would mint a different URL every second,
/// and the browser caches media by URL — the same clip would be downloaded again
/// on every request. Snapping makes the URL stable within the window.
fn window_start(now: SystemTime) -> SystemTime {
    let secs = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    SystemTime::UNIX_EPOCH + Duration::from_secs(secs - secs % WINDOW)
}

/// Build a temporary signed link to one private R2 object.
///
/// The signature travels inside the URL, so nothing is stored server side.
/// Callers within the same hour get byte-identical URLs.
///
/// `bucket` is the physical bucket name, `key` the object key.
/// Returns the signed URL, or an error when signing fails.
#[tracing::instrument(name = "signed_url.get_object", skip(s3))]
pub async fn get_object(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
) -> Result<String, ApiError> {
    get_object_at(s3, bucket, key, SystemTime::now()).await
}

/// Same as [`get_object`], with the clock passed in so tests can pin it.
pub async fn get_object_at(
    s3: &aws_sdk_s3::Client,
    bucket: &str,
    key: &str,
    now: SystemTime,
) -> Result<String, ApiError> {
    let presigning = PresigningConfig::builder()
        .start_time(window_start(now))
        .expires_in(LIFETIME)
        .build()
        .map_err(|e| ApiError::Internal(format!("bad presigning config: {e}")))?;

    let request = s3
        .get_object()
        .bucket(bucket)
        .key(key)
        .presigned(presigning)
        .await
        .map_err(|e| ApiError::Internal(format!("failed to sign {key}: {e}")))?;

    Ok(request.uri().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snaps_to_the_top_of_the_hour() {
        let at = |secs| SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        // 10:00:00, 10:00:01 and 10:59:59 all snap to 10:00:00
        assert_eq!(window_start(at(36_000)), at(36_000));
        assert_eq!(window_start(at(36_001)), at(36_000));
        assert_eq!(window_start(at(39_599)), at(36_000));
        // 11:00:00 moves to the next window
        assert_eq!(window_start(at(39_600)), at(39_600));
    }

    fn test_client() -> aws_sdk_s3::Client {
        let credentials = aws_sdk_s3::config::Credentials::new(
            "test-access-key",
            "test-secret-key",
            None,
            None,
            "test",
        );
        let config = aws_sdk_s3::config::Builder::new()
            .behavior_version(aws_sdk_s3::config::BehaviorVersion::latest())
            .region(aws_sdk_s3::config::Region::new("auto"))
            .credentials_provider(credentials)
            .endpoint_url("https://example.r2.cloudflarestorage.com")
            .force_path_style(true)
            .build();
        aws_sdk_s3::Client::from_conf(config)
    }

    #[tokio::test]
    async fn same_hour_gives_the_same_url() {
        let s3 = test_client();
        let at = |secs| SystemTime::UNIX_EPOCH + Duration::from_secs(secs);
        let sign = |now| get_object_at(&s3, "anime-clips", "feed/a.mp4", now);

        let early = sign(at(36_001)).await.unwrap();
        let late = sign(at(39_599)).await.unwrap();
        let next_hour = sign(at(39_600)).await.unwrap();

        assert!(early.contains("X-Amz-Signature"), "not a signed url: {early}");
        assert_eq!(early, late, "same hour must reuse the url so caching works");
        assert_ne!(early, next_hour, "the next hour must sign afresh");
    }
}
