use std::fmt;

use crate::providers::{MCUtils, ServerJarProvider};

mod providers;

#[derive(Debug, Clone)]
pub struct FetchError(String);

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub async fn get_available_servers() -> anyhow::Result<(), FetchError> {
    MCUtils::get_available_server_brands().await
}

pub async fn get_versions(brand_id: &str) -> anyhow::Result<(), FetchError> {
    MCUtils::get_available_versions(brand_id).await
}
