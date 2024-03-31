#![feature(async_fn_in_trait)]

pub use mcutils::*;

use crate::FetchError;

mod mcutils;

pub(crate) trait ServerJarProvider {
    async fn get_available_server_brands() -> Result<(), FetchError>;
    async fn get_available_versions<T: Into<String>>(brand_id: T) -> Result<(), FetchError>;
}
