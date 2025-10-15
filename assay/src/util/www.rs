use crate::util::helpers;
use anyhow::bail;
use scraper::{Html, Selector};

pub fn cheapest_available(id: &str) -> anyhow::Result<(Option<f64>, Option<String>)> {
    let uk_url = uk_marketplace_url(id);
    tracing::info!("fetching UK prices from {uk_url}");
    let uk_raw = raw_page(&uk_url)?;

    if let Some(uk_price) = extract_price_uk(&uk_raw)? {
        tracing::debug!("found UK price of {uk_price}");
        return Ok((Some(uk_price), Some("uk".to_owned())));
    }

    tracing::info!("{id} has no UK price");
    let world_url = world_marketplace_url(id);
    tracing::info!("fetching world prices from {world_url}");
    let world_raw = raw_page(&world_url)?;

    if let Some(world_price) = extract_price_world(&world_raw)? {
        tracing::info!("found world price of {world_price}");
        return Ok((Some(world_price), Some("world".to_owned())));
    }

    Ok((None, None))
}

fn raw_page(url: &str) -> anyhow::Result<String> {
    let body = ureq::get(url).call()?.body_mut().read_to_string()?;
    Ok(body)
}

fn uk_marketplace_url(id: &str) -> String {
    format!("https://www.discogs.com/sell/release/{id}?ships_from=United+Kingdom")
}

fn world_marketplace_url(id: &str) -> String {
    format!("https://www.discogs.com/sell/release/{id}")
}

// Gets the cheapest price from the given HTML page
fn extract_price(html: &str, selector_str: &str) -> anyhow::Result<Option<f64>> {
    tracing::debug!("parsing HTML");
    let document = Html::parse_document(html);

    if let Ok(selector) = Selector::parse(selector_str) {
        if let Some(element) = document.select(&selector).next() {
            Ok(helpers::price_from_str(
                &element.text().collect::<Vec<_>>().join(" "),
            ))
        } else {
            tracing::debug!("selector matched nothing");
            Ok(None)
        }
    } else {
        bail!("Error parsing document");
    }
}

fn extract_price_uk(html: &str) -> anyhow::Result<Option<f64>> {
    extract_price(
        html,
        "tr.shortcut_navigable:nth-child(1) > td:nth-child(5) > span:nth-child(1)",
    )
}

fn extract_price_world(html: &str) -> anyhow::Result<Option<f64>> {
    extract_price(
        html,
        "#pjax_container > table > tbody > tr:nth-child(1) > td.item_price.hide_mobile > span.converted_price",
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::test_utils::spec_helper::load_fixture;

    #[test]
    fn test_extract_price_world() {
        assert_eq!(
            17.44,
            extract_price_world(&load_fixture("31853303-world.html"))
                .unwrap()
                .unwrap(),
        );
    }

    #[test]
    fn test_extract_price_uk() {
        assert_eq!(
            21.00,
            extract_price_uk(&load_fixture("31853303-uk.html"))
                .unwrap()
                .unwrap(),
        );
    }
}
