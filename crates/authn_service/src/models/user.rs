use super::UserDevice;

use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
#[unique(email)]
pub struct User {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub created_at: Timestamp,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub updated_at: Timestamp,

    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    pub is_verified: bool,

    #[schema(example = "user@example.com")]
    pub email: String,

    #[has_many]
    #[serde(skip)]
    pub devices: Vec<UserDevice>,
}
