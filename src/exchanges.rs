use log::warn;
use serde::{Deserialize, Serialize};

use super::{env_eodhd_token, EODHDError};

const BASE_URL: &str = "https://eodhistoricaldata.com/api/";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EODHDExchange {
    // Name. e.g. Toronto Exchange
    pub name: String,
    // Code. e.g. TO
    pub code: String,
    // Currency. e.g. CAD
    pub currency: String,
    // Full country name. e.g. Canada
    pub country: String,
    // Two-letter country abbreviation. e.g. CA
    #[serde(rename = "CountryISO2")]
    pub country_iso2: String,
    // Three-letter country abbreviation. e.g. CAN
    #[serde(rename = "CountryISO3")]
    pub country_iso3: String,
}

pub async fn get_exchanges() -> Result<Vec<EODHDExchange>, EODHDError> {
    let url = format!(
        "{base_url}/exchanges-list/?api_token={token}&fmt=json",
        base_url = BASE_URL,
        token = env_eodhd_token()
    );
    let request = reqwest::get(url).await;

    if request.is_err() {
        let description: &str = "request failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(request.err().unwrap()),
        });
    }

    let res = request.unwrap().json::<Vec<EODHDExchange>>().await;
    if res.is_err() {
        let description: &str = "parsing exchanges failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(res.err().unwrap()),
        });
    }
    Ok(res.unwrap())
}
