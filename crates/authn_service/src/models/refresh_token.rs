use super::User;

use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct RefreshToken {
    #[key]
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_id: Uuid,
    pub created_at: Timestamp,
    pub expires_at: Timestamp,

    pub token_family_id: Uuid,
    pub is_revoked: bool,
    pub token_hash: Vec<u8>,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,
}
