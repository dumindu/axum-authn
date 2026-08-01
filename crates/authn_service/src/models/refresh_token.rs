use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
#[unique(token_hash)]
#[index(token_family_id)]
pub struct RefreshToken {
    #[key]
    pub id: Uuid,

    pub user_id: Uuid,
    pub device_id: Uuid,
    pub token_family_id: Uuid,

    #[auto]
    pub created_at: Timestamp,
    pub expires_at: Timestamp,

    pub is_revoked: bool,
    pub token_hash: Vec<u8>,
}
