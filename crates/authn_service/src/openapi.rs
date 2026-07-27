use utoipa::{OpenApi, openapi::OpenApi as OpenApiDoc};

#[derive(OpenApi)]
#[openapi(
    info(title = "Authn Service API", version = "1.0.0"),
    servers(
        (url = "http://localhost:3000", description = "Development")
    ),
    components(schemas(
        crate::errors::ErrorResponse,
        crate::app::shared::ValidationErrorResponse
    )),
)]
pub struct ApiDoc;

pub fn generate_doc() -> OpenApiDoc {
    let openapi = ApiDoc::openapi();
    openapi
}
