use deno_bindgen::deno_bindgen;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Subscribe to tracing events, i.e. turn on logs.
#[deno_bindgen]
pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_env("MATRIX_LOG"))
        .init();
}