use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// One row of public.places joined to its_facility_details.
///
/// The ITS-only fields (kind … closed) are null for every row that is not an
/// ITS facility. Doubles as the DB row and the API response.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Place {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub subcategory: Option<String>,
    pub address: Option<String>,
    pub lat: f64,
    pub lng: f64,
    pub visitable: Option<bool>,
    pub visited_on: Option<NaiveDate>,
    pub note: Option<String>,
    pub url: Option<String>,
    pub image_url: Option<String>,
    pub kind: Option<String>,
    pub seasons: Option<Vec<String>>,
    pub prefecture: Option<String>,
    pub city: Option<String>,
    pub room_types: Option<String>,
    pub min_party: Option<i16>,
    pub detail_url: Option<String>,
    pub market_price_yen: Option<i32>,
    pub closed: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ListPlacesQuery {
    pub category: Option<String>,
    pub subcategory: Option<String>,
}

/// One row of public.place_visits for the logged-in user.
///
/// applied_count and last_applied_on only mean something for ITS resorts;
/// they stay 0 / null for other places.
#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct PlaceVisit {
    pub place_id: Uuid,
    pub applied_count: i16,
    pub last_applied_on: Option<NaiveDate>,
    pub visited_on: Option<NaiveDate>,
    pub visited_with: Option<String>,
    pub review: Option<String>,
}

/// Body of PUT /places/{id}/mine: the whole record, replaced as one.
#[derive(Debug, Deserialize)]
pub struct SaveVisitRequest {
    pub applied_count: i16,
    pub last_applied_on: Option<NaiveDate>,
    pub visited_on: Option<NaiveDate>,
    pub visited_with: Option<String>,
    pub review: Option<String>,
}
