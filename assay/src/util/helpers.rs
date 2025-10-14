use regex::Regex;

pub fn release_id(discogs_uri: &str) -> Option<String> {
    let re = Regex::new(r"/release/(\d+)").unwrap();
    re.captures(discogs_uri)
        .and_then(|cap| cap.get(1).map(|m| m.as_str().to_owned()))
}

pub fn price_from_str(raw: &str) -> Option<f64> {
    let re = Regex::new(r"£(\d+\.\d+)").unwrap();
    let m = re
        .captures(raw)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str());

    if let Some(num_str) = m {
        num_str.parse::<f64>().ok()
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_release_id() {
        assert_eq!(
            "1618085".to_owned(),
            release_id("/Quintron-IF-001-011/release/1618085").unwrap()
        );
        assert_eq!(
            "31522313".to_owned(),
            release_id("/release/31522313-Melt-Banana-35").unwrap()
        );
    }

    #[test]
    fn test_price_from_str() {
        assert_eq!(None, price_from_str("no price"));
        assert_eq!(Some(12.34), price_from_str("£12.34"));
        assert_eq!(Some(9.99), price_from_str(" about  £9.99 "));
    }
}
