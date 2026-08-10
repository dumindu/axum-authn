use super::User;

use jiff::Timestamp;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
#[unique(user_id, device_hash)]
#[unique(credential_id)]
pub struct UserDevice {
    #[auto]
    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub created_at: Timestamp,

    #[schema(value_type = String, format = DateTime, examples("2026-07-06T13:38:00Z"))]
    pub last_used_at: Timestamp,

    #[key]
    #[auto]
    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub id: Uuid,

    #[schema(value_type = String, format = Uuid, examples("01bbbbbb-bbbb-7bbb-8bbb-bbbbbbbbbbbb"))]
    pub user_id: Uuid,

    #[serde(skip)]
    pub signature_counter: u8,

    #[schema(example = "iPhone 18 Pro")]
    pub device_name: String,

    #[serde(skip)]
    pub device_hash: Vec<u8>,

    #[serde(skip)]
    pub credential_id: Vec<u8>,

    #[serde(skip)]
    pub public_key: Vec<u8>,

    #[belongs_to(key = user_id, references = id)]
    #[serde(skip)]
    pub user: toasty::Deferred<User>,
}
