use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::NaiveDate;

use crate::config::AppState;
use crate::types::ApiError;

use super::model::{ListOverallQuery, RankingResponse};
use super::store;

const DEFAULT_LIMIT: i64 = 100;
const MAX_LIMIT: i64 = 1000;

pub fn router() -> Router<AppState> {
    Router::new().route("/overall", get(list_overall))
}

// GET /github/overall?date=YYYY-MM-DD&limit=
//
// Response: 200 { date, repos } with the day's 'overall' repos by rank (latest day when date is
// absent, limit 100 by default, capped at 1000); 400 on a malformed date; 404 when that day has no rows.
async fn list_overall(
    State(state): State<AppState>,
    Query(query): Query<ListOverallQuery>,
) -> Result<Json<RankingResponse>, ApiError> {
    let date = parse_date(query.date.as_deref())?;
    let limit = clamp_limit(query.limit);
    let rows = store::list_ranking(&state.db, "overall", date, limit).await?;
    let Some(first) = rows.first() else {
        return Err(ApiError::NotFound);
    };
    let snapshot_date = first.snapshot_date;
    let repos = rows.into_iter().map(|row| row.repo).collect();
    Ok(Json(RankingResponse { date: snapshot_date, repos }))
}

/// Parses ?date= as YYYY-MM-DD; None when absent, BadRequest when malformed.
fn parse_date(raw: Option<&str>) -> Result<Option<NaiveDate>, ApiError> {
    let Some(raw) = raw else {
        return Ok(None);
    };
    NaiveDate::parse_from_str(raw, "%Y-%m-%d")
        .map(Some)
        .map_err(|_| ApiError::BadRequest(format!("date must be YYYY-MM-DD: {raw}")))
}

/// Turns ?limit= into a row count between 1 and MAX_LIMIT, DEFAULT_LIMIT when absent.
fn clamp_limit(raw: Option<i64>) -> i64 {
    raw.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_absent_is_none() {
        assert_eq!(parse_date(None).unwrap(), None);
    }

    #[test]
    fn parse_date_reads_the_day() {
        let expected = NaiveDate::from_ymd_opt(2026, 11, 3);
        assert_eq!(parse_date(Some("2026-11-03")).unwrap(), expected);
    }

    #[test]
    fn clamp_limit_caps_at_max() {
        assert_eq!(clamp_limit(Some(5000)), 1000);
    }
}
