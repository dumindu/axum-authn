use authn_service::DbConf;
use authn_service::models::*;
use toasty_cli::{Config, ToastyCli};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_url = DbConf::init().to_database_url();
    let config = Config::load()?;
    let db = toasty::Db::builder()
        .models(toasty::models!(
            OauthChallenge,
            RefreshToken,
            SecurityAuditLog,
            User,
            UserDevice,
            UserIdentity,
            UserPassword,
            WebauthnChallenge
        ))
        .connect(&db_url)
        .await?;

    let cli = ToastyCli::with_config(db, config);

    cli.parse_and_run().await?;

    Ok(())
}
