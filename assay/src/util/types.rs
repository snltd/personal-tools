use serde::Deserialize;

pub type Price = f64; // Because I don't care about accuracy at all.

#[derive(Debug)]
pub(crate) struct RecordToAssay {
    pub(crate) id: u32,
    pub(crate) discogs_uri: Option<String>,
    pub(crate) title: String,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub db_uri: String,
    pub batch_size: usize,
    pub condition: String,
    pub discogs_token: String,
}
