use super::User;

use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
pub struct UserDevice {
    // Initial order for postgres columns by alignment(16-byte, 8-byte, 4-byte, 2-byte, 1-byte, then variable-length types)
    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub user_id: Uuid,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub created_at: Timestamp,

    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub updated_at: Timestamp,

    #[schema(example = "iPhone 18 Pro")]
    pub device_name: String,

    #[serde(skip)]
    pub device_hash: Vec<u8>,

    #[serde(skip)]
    pub credential_id: Vec<u8>,

    #[serde(skip)]
    pub public_key: Vec<u8>,

    #[serde(skip)]
    pub signature_counter: i32,

    #[belongs_to(key = user_id, references = id)]
    #[serde(skip)]
    pub user: toasty::Deferred<User>,
}
