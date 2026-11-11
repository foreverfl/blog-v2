use axum::extract::{Query, State};
use axum::routing::get;
use axum::{Json, Router};
use chrono::NaiveDate;

use crate::config::AppState;
use crate::types::ApiError;

use super::model::{ListOverallQuery, RankingResponse};
use super::store;

const DEFAULT_LIMIT: i32 = 100;
const MAX_LIMIT: i32 = 1000;

pub fn router() -> Router<AppState> {
    Router::new().route("/overall", get(list_overall))
}

// GET /github/overall?date=YYYY-MM-DD&limit=&offset=
//
// Response: 200 { date, repos } with the day's overall ranking in rank order,
// skipping `offset` rows, the latest day when date is missing; 400 for a
// malformed date; 404 when offset is 0 and the day has no rows, an empty
// repos past the end. Public read-only — no auth.
async fn list_overall(
    State(state): State<AppState>,
    Query(query): Query<ListOverallQuery>,
) -> Result<Json<RankingResponse>, ApiError> {
    let date = parse_date(query.date.as_deref())?;
    let ranking = store::list_overall(
        &state.db,
        date,
        clamp_limit(query.limit),
        clamp_offset(query.offset),
    )
    .await?;
    Ok(Json(ranking))
}

/// None when no date was given; 400 when it is not YYYY-MM-DD.
fn parse_date(raw: Option<&str>) -> Result<Option<NaiveDate>, ApiError> {
    raw.map(|s| {
        NaiveDate::parse_from_str(s, "%Y-%m-%d")
            .map_err(|_| ApiError::BadRequest(format!("date must be YYYY-MM-DD: {s}")))
    })
    .transpose()
}

/// 100 when missing; kept within 1..=1000 so one call never pulls the whole 10000-row day.
fn clamp_limit(limit: Option<i32>) -> i64 {
    limit.unwrap_or(DEFAULT_LIMIT).clamp(1, MAX_LIMIT).into()
}

/// 0 when missing or negative.
fn clamp_offset(offset: Option<i32>) -> i64 {
    offset.unwrap_or(0).max(0).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_date_reads_missing_and_given_dates() {
        assert_eq!(parse_date(None).unwrap(), None);
        assert_eq!(
            parse_date(Some("2026-11-03")).unwrap(),
            NaiveDate::from_ymd_opt(2026, 11, 3)
        );
        assert!(parse_date(Some("2026/11/03")).is_err());
    }

    #[test]
    fn clamp_limit_defaults_and_caps() {
        assert_eq!(clamp_limit(None), 100);
        assert_eq!(clamp_limit(Some(5000)), 1000);
    }

    #[test]
    fn clamp_offset_defaults_and_floors_at_zero() {
        assert_eq!(clamp_offset(None), 0);
        assert_eq!(clamp_offset(Some(-5)), 0);
        assert_eq!(clamp_offset(Some(100)), 100);
    }
}
