use std::{
    collections::{HashMap, HashSet},
    fs::File,
    path::Path,
};

use csv::Reader;
use serde::Deserialize;

use crate::{Part, error::Error};

pub fn read_bom_file(bom_file: &str) -> Result<Vec<Part>, Error> {
    let path = Path::new(bom_file);
    let file = File::open(path).map_err(|e| Error::OpenCsv {
        source: e,
        file_path: path.to_string_lossy().into_owned(),
    })?;
    let mut rdr: Reader<File> = csv::Reader::from_reader(file);

    let mut records = vec![];

    for result in rdr.deserialize() {
        let record: CsvItem = result.map_err(Error::CsvDeser)?;
        records.push(record);
    }

    let records = records
        .iter()
        .map(|v| {
            let hash: HashMap<u32, HashSet<String>> = match v.option.is_empty() {
                true => HashMap::new(),
                false => parse_option(&v.option, &v.part_ipn)?,
            };

            Ok(Part {
                part_ipn: v.part_ipn.clone(),
                quantity: v.quantity,
                option: hash,
                ..Default::default()
            })
        })
        .collect::<Result<Vec<Part>, Error>>()?;

    Ok(records)
}

fn parse_option(option: &str, part_ipn: &str) -> Result<HashMap<u32, HashSet<String>>, Error> {
    let options = option.split(",");
    let mut hash: HashMap<u32, HashSet<String>> = HashMap::new();
    for o in options {
        let (opt_index, opt_value) = o.split_once(':').ok_or(Error::ParseOptions {
            part_ipn: part_ipn.to_string(),
            options: option.to_string(),
        })?;

        let opt_index: u32 = opt_index.parse().map_err(|_| Error::ParseOptions {
            part_ipn: part_ipn.to_string(),
            options: option.to_string(),
        })?;

        hash.entry(opt_index)
            .or_default()
            .insert(opt_value.to_string());
    }
    Ok(hash)
}

#[derive(Debug, Deserialize)]
struct CsvItem {
    part_ipn: String,
    quantity: u32,
    option: String,
}
