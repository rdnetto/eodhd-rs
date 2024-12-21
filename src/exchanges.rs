use log::warn;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DefaultOnError};

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
    let request = reqwest::get(url).await.and_then(|res| res.error_for_status());

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EODHDTickerType {
    #[serde(alias = "Common Stock")]
    CommonStock,
    #[serde(alias = "Preferred Stock")]
    PreferredStock,
    // Both common + preferred
    Stock,
    ETF,
    Fund
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EODHDExchangeTicker {
    // Ticker code. e.g. CBA
    pub code: String,
    // Ticker name. e.g. Commonwealth Bank Of Austrlia
    pub name: String,
    // Full country name
    pub country: String,
    // Exchange code
    pub exchange: String,
    // Exchange currency
    pub currency: String,
    // Ticker type. None if of unknown type.
    #[serde_as(deserialize_as = "DefaultOnError")]
    #[serde(rename = "Type")]
    pub ticker_type: Option<EODHDTickerType>,
    // ISIN - uniquely identifies ticker across exchanges.
    pub isin: Option<String>,
}

pub async fn get_tickers(
    exchange: &str,
    // If specified, only return tickers of the given type.
    ticker_type: Option<EODHDTickerType>
    ) -> Result<Vec<EODHDExchangeTicker>, EODHDError> {
    let mut url = format!(
        "{base_url}/exchange-symbol-list/{exchange}/?api_token={token}&fmt=json",
        base_url = BASE_URL,
        exchange = exchange,
        token = env_eodhd_token()
    );

    if ticker_type.is_some() {
        url = format!(
            "{url}&type={typ}",
            url = url,
            typ = match ticker_type.unwrap() {
                EODHDTickerType::CommonStock => "common_stock",
                EODHDTickerType::PreferredStock => "preferred_stock",
                EODHDTickerType::Stock => "stock",
                EODHDTickerType::ETF => "etf",
                EODHDTickerType::Fund => "fund"
            }
        );
    }

    let request = reqwest::get(url).await.and_then(|res| res.error_for_status());

    if request.is_err() {
        let description: &str = "request failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(request.err().unwrap()),
        });
    }

    let res = request.unwrap().json::<Vec<EODHDExchangeTicker>>().await;
    if res.is_err() {
        let description: &str = "parsing tickers failed";
        warn!("{}", description);
        return Err(EODHDError {
            description: description.to_string(),
            inner_error: Box::new(res.err().unwrap()),
        });
    }
    Ok(res.unwrap())
}
