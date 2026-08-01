use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct WebauthnChallenge {
    pub created_at: Timestamp,
    pub expires_at: Timestamp,

    #[key]
    pub challenge_token: Vec<u8>,

    pub user_id: Option<Uuid>,
    pub device_id: Option<Uuid>,
    pub challenge_type: String,
}
