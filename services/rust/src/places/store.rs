use sqlx::PgPool;
use uuid::Uuid;

use crate::types::ApiError;

use super::model::{Place, PlaceVisit};

/// Lists places with their ITS details, filtered by category and/or subcategory when given.
#[tracing::instrument(name = "places.list", skip(pool))]
pub async fn list(
    pool: &PgPool,
    category: Option<&str>,
    subcategory: Option<&str>,
) -> Result<Vec<Place>, ApiError> {
    let places = sqlx::query_as::<_, Place>(
        r#"
        SELECT p.id, p.name, p.category, p.subcategory, p.address, p.lat, p.lng,
               p.visitable, p.visited_on, p.note, p.url, p.image_url,
               d.kind, d.seasons, d.prefecture, d.city, d.room_types,
               d.min_party, d.detail_url, d.market_price_yen, d.closed
        FROM public.places p
        LEFT JOIN public.its_facility_details d ON d.place_id = p.id
        WHERE ($1::text IS NULL OR p.category = $1)
          AND ($2::text IS NULL OR p.subcategory = $2)
        ORDER BY p.name
        "#,
    )
    .bind(category)
    .bind(subcategory)
    .fetch_all(pool)
    .await?;

    Ok(places)
}

/// Lists every place_visits row of one user, oldest first.
#[tracing::instrument(name = "places.list_visits", skip(pool))]
pub async fn list_visits(pool: &PgPool, user_id: Uuid) -> Result<Vec<PlaceVisit>, ApiError> {
    let visits = sqlx::query_as::<_, PlaceVisit>(
        r#"
        SELECT place_id, applied_count, last_applied_on, visited_on, visited_with, review
        FROM public.place_visits
        WHERE user_id = $1
        ORDER BY created_at
        "#,
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(visits)
}
