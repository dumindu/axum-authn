use super::User;

use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct UserPassword {
    #[key]
    pub user_id: Uuid,

    #[auto]
    pub created_at: Timestamp,

    #[auto]
    pub updated_at: Timestamp,

    pub password_hash: String,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,
}
