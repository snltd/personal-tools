use crate::util::types::Config;
use crate::util::{assay, db};
use anyhow::ensure;
use camino::Utf8PathBuf;
use clap::Parser;
use std::fs;
use tracing_subscriber::EnvFilter;

mod test_utils;
mod util;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Path to the config file
    #[arg(short = 'c', long = "config", default_value = "assay.toml")]
    config: Utf8PathBuf,
}

fn load_config(path: &Utf8PathBuf) -> anyhow::Result<Config> {
    ensure!(path.exists(), format!("Config not found: {}", path));

    let raw = fs::read_to_string(path)?;
    let config = toml::from_str(&raw)?;
    Ok(config)
}

fn main() {
    let cli = Cli::parse();

    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .with_ansi(true)
        .init();

    let conf = match load_config(&cli.config) {
        Ok(conf) => conf,
        Err(e) => {
            tracing::error!("error loading config: {e}");
            std::process::exit(1);
        }
    };

    let mut conn = match db::connection(&conf.db_uri) {
        Ok(conn) => conn,
        Err(e) => {
            tracing::error!("ERROR connecting to database: {e}");
            std::process::exit(1);
        }
    };

    match db::records_to_assay(&mut conn, conf.batch_size) {
        Ok(list_of_records) => match assay::assay_list(&mut conn, list_of_records, &conf) {
            Ok(_) => println!("OK"),
            Err(e) => {
                tracing::error!("ERROR assaying: {e}");
                std::process::exit(2);
            }
        },
        Err(e) => {
            tracing::error!("ERROR getting list: {e}");
            std::process::exit(2);
        }
    }
}
