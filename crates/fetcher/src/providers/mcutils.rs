use crate::providers::ServerJarProvider;
use crate::FetchError;

pub struct MCUtils;

impl MCUtils {
    const API_URL: &'static str = "https://mcutils.com/api/server-jars";
}

impl ServerJarProvider for MCUtils {
    async fn get_available_server_brands() -> Result<(), FetchError> {
        let response = reqwest::get(Self::API_URL)
            .await
            .map_err(|e| FetchError(format!("Could not fetch data from {}", Self::API_URL)))?;

        if !response.status().is_success() {
            return Err(FetchError(format!(
                "Could not fetch data from {}",
                Self::API_URL
            )));
        }

        // let servers: Vec<_> = response
        //     .json()
        //     .await
        //     .map_err(|e| "Could not parse response")?;
        // Ok(servers)
        Ok(())
    }

    async fn get_available_versions<T: Into<String>>(brand_id: T) -> Result<(), FetchError> {
        let url = format!("{}/{}", Self::API_URL, brand_id.into());
        let response = reqwest::get(&url)
            .await
            .map_err(|e| FetchError(format!("Could not fetch data from {}", url)))?;

        Ok(())
    }
}
