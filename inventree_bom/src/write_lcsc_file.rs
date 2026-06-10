use std::path::Path;

use csv::Writer;
use serde::Serialize;

use crate::{Part, error::Error};

pub fn write_lcsc_file(parts: &[Part], output_dir: &Path) -> Result<(), Error> {
    let path = output_dir.join("lcsc.csv");
    let mut writer = Writer::from_path(path).map_err(|e| Error::CreateOutputCsv { source: e })?;

    let records = parts.iter().map(|part| FileRecord {
        part_ipn: part.part_ipn.clone(),
        quantity: part.quantity,
        lcsc_sku: part.lcsc_sku.clone(),
    });

    for record in records {
        writer
            .serialize(record)
            .map_err(|e| Error::WriteToCsv { source: e })?;
    }

    writer.flush().map_err(|e| Error::FlushFile { source: e })?;

    Ok(())
}

#[derive(Debug, Default, Serialize)]
pub struct FileRecord {
    pub part_ipn: String,

    #[serde(rename = "Quantity")]
    pub quantity: u32,

    #[serde(rename = "LCSC Part Number")]
    pub lcsc_sku: String,
}
