use std::sync::Arc;

use crate::{
    cache::Cache,
    config::Config,
};

pub struct AppState {
    config: Arc<Config>,
    redis: Cache,
}

impl AppState {
    pub fn new(
        config: Arc<Config>,
        redis: Cache,
    ) -> Self {
        Self {
            config,
            redis,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub fn redis(&self) -> &Cache {
        &self.redis
    }
}
