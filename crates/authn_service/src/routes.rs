use std::{str::FromStr, time::Duration};

use axum::{
    Router,
    extract::DefaultBodyLimit,
    http::{HeaderName, HeaderValue, Method, Request, Response, StatusCode},
    response::IntoResponse,
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::{
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    trace::TraceLayer,
};
use tracing::{Span, info, info_span};

use crate::{state::AppState};

const REQUEST_ID_HEADER: &str = "x-request-id";

pub fn init(state: AppState) -> Router {
    let x_request_id = HeaderName::from_static(REQUEST_ID_HEADER);

    let middleware = ServiceBuilder::new()
        .layer(SetRequestIdLayer::new(x_request_id.clone(), MakeRequestUuid))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let request_id = request
                        .headers()
                        .get(REQUEST_ID_HEADER)
                        .and_then(|value| value.to_str().ok())
                        .unwrap_or("unknown");

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        uri_path = request.uri().path(),
                        request_id,
                        status_code = tracing::field::Empty,
                        latency = tracing::field::Empty
                    )
                })
                .on_response(|response: &Response<_>, latency: Duration, span: &Span| {
                    span.record("status_code", tracing::field::display(response.status()));
                    span.record("latency", latency.as_millis());

                    span.in_scope(|| {
                        info!("request completed");
                    });
                }),
        )
        .layer(PropagateRequestIdLayer::new(x_request_id))
        .layer(cors_layer(&state));

    Router::new()
        // .route("/livez", get(livez))
        // .route("/login", post(login::login))
        // .route("/reset-password", post(password::trigger_reset_password).put(password_reset::reset_password))
        // .route("/social/state", post(state::state))// can state::state reexport as state
        // .route("/social/google/login", post(google::login))// can google::login reexport as login_with_google
        // .route("/social/apple/login", post(apple::login))// can apple::login reexport as login_with_google
        //
        // .route("/passkey/challenge", post(passkey::challenge))
        // .route("/passkey/register", post(passkey::register))
        // .route("/passkey/authenticate", post(passkey::authenticate))
        //
        // // PROTECTED
        // .route("/refresh", post(refresh::refresh))
        // .route("/change-password", put(password::change_password))
        // .route("/set-password", post(password::set_password))
        // .route("/social/google/connect", post(google::connect))// can google::connect reexport as connect_to_google
        // .route("/social/apple/connect", post(apple::connect))// can apple::connect reexport as connect_to_apple
        // .route("/logout", post(logout::logout))
        .layer(DefaultBodyLimit::max(state.server_conf.default_body_limit))
        .layer(middleware)
        .with_state(state)
}

// src/features/authn/mod.rs
use axum::{routing::{get, post, put}, Router};

pub fn router() -> Router<()> {
    Router::new()
        .route("/login", post(login::login))
        .route("/reset-password", post(password::trigger_reset_password).put(password_reset::reset_password))
        .route("/social/state", post(state::state))// can state::state reexport as state
        .route("/social/google/login", post(google::login))// can google::login reexport as login_with_google
        .route("/social/apple/login", post(apple::login))// can apple::login reexport as login_with_google

        .route("/passkey/challenge", post(passkey::challenge))
        .route("/passkey/register", post(passkey::register))
        .route("/passkey/authenticate", post(passkey::authenticate))

        // PROTECTED
        .route("/refresh", post(refresh::refresh))
        .route("/change-password", put(password::change_password))
        .route("/set-password", post(password::set_password))
        .route("/social/google/connect", post(google::connect))// can google::connect reexport as connect_to_google
        .route("/social/apple/connect", post(apple::connect))// can apple::connect reexport as connect_to_apple
        .route("/logout", post(logout::logout))
}


async fn livez() -> impl IntoResponse {
    StatusCode::OK
}

fn cors_layer(state: &AppState) -> CorsLayer {
    let origins: Vec<HeaderValue> = state
        .server_conf
        .allowed_origins
        .split(',')
        .map(|s| HeaderValue::try_from(s.trim()).expect("Invalid origin header"))
        .collect();

    let methods: Vec<Method> = state
        .server_conf
        .allowed_methods
        .split(',')
        .map(|s| Method::from_str(s.trim()).expect("Invalid HTTP method"))
        .collect();

    let headers: Vec<HeaderName> = state
        .server_conf
        .allowed_headers
        .split(',')
        .map(|s| HeaderName::from_str(s.trim()).expect("Invalid HTTP header name"))
        .collect();

    CorsLayer::new().allow_origin(origins).allow_methods(methods).allow_headers(headers)
}
