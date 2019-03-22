use serde::{Deserialize, Serialize};
use serde_json::json;
use azure_functions::{
    bindings::{HttpRequest, HttpResponse},
    func,
};
// Get a static file that has preprocessed the data from the url below:
// https://www.belastingdienst.nl/wps/wcm/connect/nl/douane_voor_bedrijven/content/hulpmiddel-wisselkoersen
static CURRENCY_FILE: &'static str = include_str!("./data/currency.json");

#[derive(Serialize, Deserialize)]
struct CurrencyData {
    name: String,
    code: String,
    euro: f64,
    foreign: f64
}

#[func]
pub fn currency(req: HttpRequest) -> HttpResponse {
    // Parse the input.
    let from = match req.query_params().get("from") {
        Some(s) => s.clone(),
        _ => return currency_error(),
    };
    let to = match req.query_params().get("to") {
        Some(s) => s.clone(),
        _ => return currency_error(),
    };
    // Parse the data.
    let currencies : Vec<CurrencyData> = match serde_json::from_str(CURRENCY_FILE){
        Ok(c) => c,
        Err(_e) => return currency_list_error(),
    };
    // Get the exchange rate.
    let from_currency : &CurrencyData = match currencies.iter().find(|e | e.code == from) {
        Some(c) => c,
        _ => return currency_not_found(),
    };
    let to_currency : &CurrencyData = match currencies.iter().find(|e | e.code == to) {
        Some(c) => c,
        _ => return currency_not_found(),
    };
    // Calculate!
    let ratio = to_currency.euro / from_currency.euro;
    // Output!
    json!(format!("{{\"ratio\":\"{:.5}\"}}", ratio)).into()
}

fn currency_error() -> HttpResponse {
    json!("{\"ratio\":\"Currency error\"}").into()
}
fn currency_not_found() -> HttpResponse {
    json!("{\"ratio\":\"Currency not found\"}").into()
}
fn currency_list_error() -> HttpResponse {
    json!("{\"ratio\":\"Currency list could not be obtained\"}").into()
}