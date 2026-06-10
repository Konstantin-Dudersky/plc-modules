use reqwest::{blocking::Client, header};
use serde::Deserialize;
use tracing::warn;

use crate::{Part, SUPPLIER_LCSC, error::Error};

pub fn req_sku(
    client: &Client,
    part: Part,
    inventree_token: &str,
    inventree_url: &str,
) -> Result<Part, Error> {
    let url = format!("{inventree_url}/company/part/");

    let response = client
        .get(url)
        .header(header::AUTHORIZATION, inventree_token)
        .query(&[("part", &part.pk)])
        .query(&[("supplier", SUPPLIER_LCSC)])
        .send()
        .map_err(|e| Error::Request {
            source: e,
            part_ipn: part.part_ipn.to_string(),
        })?;

    let r = response
        .json::<Vec<ResponsePart>>()
        .map_err(|e| Error::PartPkDeser {
            source: e,
            part_ipn: part.part_ipn.to_string(),
        })?;

    let response_part = match r.first() {
        Some(v) => v,
        None => {
            warn!("Для детали {} не найден поставщик LCSC", part.part_ipn);
            return Ok(part);
        }
    };

    let part = Part {
        lcsc_sku: response_part.SKU.clone(),
        ..part
    };

    Ok(part)
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct ResponsePart {
    pub SKU: String,
}
