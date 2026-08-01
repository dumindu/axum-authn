use jiff::Timestamp;
use uuid::Uuid;

#[derive(Debug, toasty::Model)]
pub struct OauthChallenge {
    #[key]
    pub state: Uuid,

    #[auto]
    pub created_at: Timestamp,

    pub expires_at: Timestamp,

    pub pkce_code_verifier: String,
    pub client_redirect_uri: String,

    pub provider: String,
    pub flow_type: String,
}
