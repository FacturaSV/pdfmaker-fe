use axum::{Router, routing::post};
use crate::routes::Pdf::generate_invoice;

pub fn create_app() -> Router {
    // Router::new().route("/generate-invoice", post(generate_invoice)),
    Router::new().route("/generate-fe", post(generate_invoice))
}
