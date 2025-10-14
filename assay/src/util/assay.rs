use crate::util::types::{Config, RecordToAssay};
use crate::util::{api, db, helpers, www};
use mysql::PooledConn;

pub fn assay_list(
    conn: &mut PooledConn,
    list_of_records: Vec<RecordToAssay>,
    conf: &Config,
) -> anyhow::Result<()> {
    for record in list_of_records {
        assay_record(conn, record, conf)?;
    }

    Ok(())
}

fn assay_record(conn: &mut PooledConn, record: RecordToAssay, conf: &Config) -> anyhow::Result<()> {
    if let Some(discogs_uri) = &record.discogs_uri {
        tracing::info!("considering {} [{}]", record.title, record.id);

        if let Some(id) = helpers::release_id(discogs_uri) {
            let api_uri = format!("https://api.discogs.com/marketplace/price_suggestions/{id}");
            db::update_api_price(conn, record.id, api::marketplace_suggestion(api_uri, conf)?)?;
            db::update_www_price(conn, record.id, www::cheapest_available(&id)?)?;
        } else {
            tracing::error!("Could not get ID");
        }
    } else {
        tracing::warn!("No discogs_uri for {} [{}]", record.title, record.id);
    }

    Ok(())
}
