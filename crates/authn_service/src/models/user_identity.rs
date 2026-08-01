use super::User;

use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct UserIdentity {
    #[key]
    pub id: Uuid,
    pub user_id: Uuid,
    pub created_at: Timestamp,

    pub provider: String,
    pub provider_sub_id: String,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,
}
