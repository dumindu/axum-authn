use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, toasty::Model, Serialize, ToSchema)]
pub struct UserDevice {
    #[key]
    #[auto]
    #[serde(skip)]
    pub id: Uuid,

    #[serde(skip)]
    pub user_id: Uuid,

    /// SHA-256 output of device_id + salt
    #[schema(value_type = String, format = Binary, example = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")]
    pub device_hash: Vec<u8>,

    #[schema(example = "iPhone 18 Pro")]
    pub device_name: String,

    /// Base64URL string (without padding) of WebAuthn Credential ID
    #[schema(value_type = String, format = Binary, example = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")]
    pub credential_id: Vec<u8>,

    /// Public key DER/COSE binary configuration slice validated directly via aws-lc-rs
    #[schema(value_type = String, format = Binary, example = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA")]
    pub public_key: Vec<u8>,

    /// Anti-cloning monotonic chip validation transaction execution tracking sequence counter
    #[schema(example = 42)]
    pub signature_counter: i32,

    /// Registration timestamp marker
    #[schema(value_type = String, example = "2026-07-29T13:30:00Z")]
    pub created_at: Timestamp,

    /// Last active runtime cryptographic usage handshake occurrence timestamp
    #[schema(value_type = String, example = "2026-07-29T13:30:00Z")]
    pub last_used_at: Timestamp,

    #[belongs_to(key = user_id, references = id)]
    #[serde(skip)]
    pub user: toasty::Deferred<User>,
}