use crate::util::types::{Config, Price};
use serde_json::Value;

const USER_AGENT: &str = "Assay/1.0 +https://tech.id264.net";

// Gives us the price Discogs thinks a CONDITION copy of this thing is worth. It's
// probably miles out.
pub fn marketplace_suggestion(uri: String, conf: &Config) -> anyhow::Result<Option<Price>> {
    tracing::info!("fetching {uri}");
    let body = ureq::get(uri)
        .header(
            "Authorization",
            format!("Discogs token={}", conf.discogs_token),
        )
        .header("user_agent", USER_AGENT)
        .call()?
        .body_mut()
        .read_to_string()?;

    let json: Value = serde_json::from_str(&body)?;

    if let Some(value) = json.get(&conf.condition).and_then(|u| u.get("value")) {
        tracing::debug!("marketplace suggestion: {value}");
        Ok(value.as_f64())
    } else {
        Ok(None)
    }
}
