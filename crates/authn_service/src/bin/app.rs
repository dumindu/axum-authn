use authn_service::{AppConf, AppState, routes};
use tracing::info;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow::anyhow!("CRITICAL: Failed to bind AWS-LC-RS platform provider."))?;

    // 2. Provision WebAuthn server identity profile
    let rp_id = "secure.yourdata.service"; // Change to your primary production domain
    let rp_origin = url::Url::parse("https://yourdata.service")?;

    let webauthn_instance = WebauthnBuilder::new(rp_id, &rp_origin)?
        .allow_subdomains(false)
        .build()?;
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        "info,tower_http=warn,axum=warn,toasty=warn,tokio_postgres=warn".into()
    });

    tracing_subscriber::registry()
        .with(filter)
        .with(tracing_subscriber::fmt::layer().json().with_target(false).with_current_span(false))
        .init();

    let conf = AppConf::init();

    let db = toasty::Db::builder()
        .models(toasty::models!())
        .connect(&conf.db.to_database_url())
        .await?;

    let server_conf = conf.server.clone();
    let state = AppState { db, server_conf };

    let addr = conf.server.to_addr();
    info!(addr = %addr, "Starting server");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    let app = routes::init(state);
    axum::serve(listener, app).await?;

    Ok(())
}
