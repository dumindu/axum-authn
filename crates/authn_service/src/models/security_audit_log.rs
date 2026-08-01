use super::{User, UserDevice};
use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct SecurityAuditLog {
    #[key]
    #[auto]
    pub id: Uuid,

    #[auto]
    pub created_at: Timestamp,

    pub user_id: Option<Uuid>,
    pub device_id: Option<Uuid>,

    pub ip_address: String,
    pub user_agent: String,
    pub location: Option<String>,

    pub event_type: String,
    pub context: Option<String>,

    #[belongs_to(key = user_id, references = id)]
    pub user: toasty::Deferred<User>,

    #[belongs_to(key = device_id, references = id)]
    pub device: toasty::Deferred<Option<UserDevice>>
}
