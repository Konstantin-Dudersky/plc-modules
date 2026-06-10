use reqwest::{blocking::Client, header};
use serde::Deserialize;
use tracing::{debug, warn};

use crate::{Part, error::Error};

pub fn req_part_pk(
    client: &Client,
    part: Part,
    inventree_token: &str,
    inventree_url: &str,
) -> Result<Option<Part>, Error> {
    let url = format!("{inventree_url}/part/");

    let response = client
        .get(url)
        .header(header::AUTHORIZATION, inventree_token)
        .query(&[("IPN", &part.part_ipn)])
        .send()
        .map_err(|e| Error::Request {
            source: e,
            part_ipn: part.part_ipn.to_string(),
        })?;

    if response.status() != reqwest::StatusCode::OK {
        warn!("Response status: {}", response.status());
        warn!("Response status: {:?}", response.text());
        return Ok(None);
    }

    debug!("Response: {:?}", response);

    let r = response
        .json::<Vec<ResponsePart>>()
        .map_err(|e| Error::PartPkDeser {
            source: e,
            part_ipn: part.part_ipn.to_string(),
        })?;

    let response_part = match r.first() {
        Some(v) => v,
        None => {
            warn!("В базе Inventree не найдена деталь с IPN {}", part.part_ipn);
            return Ok(None);
        }
    };

    let part = Part {
        pk: response_part.pk,
        pricing_min: response_part.pricing_min.unwrap_or_default(),
        pricing_max: response_part.pricing_max.unwrap_or_default(),
        ..part
    };

    Ok(Some(part))
}

#[derive(Deserialize)]
struct ResponsePart {
    pub pk: u32,
    pub pricing_min: Option<f32>,
    pub pricing_max: Option<f32>,
}
