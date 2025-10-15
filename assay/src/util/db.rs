use crate::util::types::{Price, RecordToAssay};
use mysql::prelude::*;
use mysql::{Pool, PooledConn};

pub fn connection(uri: &str) -> anyhow::Result<PooledConn> {
    tracing::debug!("connecting to {uri}");
    Ok(Pool::new(uri)?.get_conn()?)
}

pub fn update_api_price(
    conn: &mut PooledConn,
    id: u32,
    price: Option<Price>,
) -> anyhow::Result<()> {
    if let Some(suggested_price) = price {
        tracing::info!("updating {id} with suggested price {:.2}", suggested_price);
        conn.exec_drop(
            "UPDATE rec SET discogs_vgplus_value = ?, discogs_updated = NOW() WHERE id = ?",
            (suggested_price, id),
        )?;
    } else {
        tracing::info!("{id} has no suggested price");
        conn.exec_drop("UPDATE rec SET discogs_updated = NOW() WHERE id = ?", (id,))?;
    }

    Ok(())
}

pub fn update_www_price(
    conn: &mut PooledConn,
    id: u32,
    field: Option<&str>,
    price: Option<Price>,
) -> anyhow::Result<()> {
    if let Some(cheapest_price) = price
        && let Some(field) = field
    {
        let field = format!("discogs_cheapest_{field}");

        tracing::info!(
            "updating {id} {field} with cheapest price {:.2}",
            cheapest_price
        );

        conn.exec_drop(
            format!(
                "UPDATE rec SET {} = ?, discogs_updated = NOW() WHERE id = ?",
                field
            ),
            (cheapest_price, id),
        )?;
    } else {
        tracing::info!("{id} has no cheapest price");
        conn.exec_drop("UPDATE rec SET discogs_updated = NOW() WHERE id = ?", (id,))?;
    }

    Ok(())
}

pub fn records_to_assay(conn: &mut PooledConn, limit: usize) -> anyhow::Result<Vec<RecordToAssay>> {
    tracing::debug!("fetching {limit} records from database");
    let result: Vec<RecordToAssay> = conn.exec_map(
        "SELECT id, discogs_uri, title FROM rec ORDER BY discogs_updated LIMIT ?",
        (limit,),
        |(id, discogs_uri, title)| RecordToAssay {
            id,
            discogs_uri,
            title,
        },
    )?;

    Ok(result)
}
