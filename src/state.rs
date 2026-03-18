// src/state.rs
use std::sync::Arc;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<Config>,
    // pub db: PgPool,
}

impl AppState {
    /// Initialises application state.
    ///
    /// # Panics
    ///
    /// Panics if required environment variables
    /// are missing.
    pub async fn new() -> Self {
        let config = Arc::new(Config::from_env());

        Self { config }
    }
}