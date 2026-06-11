use std::{
    collections::{BTreeMap, HashMap, HashSet},
    f32,
    path::Path,
};

use csv::Writer;
use serde::{Serialize, Serializer};
use tracing::info;

use crate::{Part, error::Error};

pub fn write_price_files(parts: &[Part], output_dir: &Path) -> Result<(), Error> {
    // Находим все возможные пары "название опции - значение опции"
    let mut all_option_pairs: HashSet<(u32, String)> = HashSet::new();
    for part in parts {
        for (key, values) in part.option.iter() {
            for opt_value in values {
                let pair = (*key, opt_value.clone());
                all_option_pairs.insert(pair);
            }
        }
    }

    info!("Обнаружены опции: {:?}", all_option_pairs);

    // создаём файл с базовой версией без опций
    create_file(parts, "base", |option| option.is_empty(), output_dir)?;

    // создаём файлы для каждой пары ключ-значение
    for (opt_name, opt_value) in all_option_pairs.iter() {
        let file_name = format!("{opt_name}_{opt_value}");
        create_file(
            parts,
            &file_name,
            |option| {
                let opt_values = option.get(opt_name);
                let opt_values = match opt_values {
                    Some(v) => v,
                    None => return false,
                };
                opt_values.contains(opt_value)
            },
            output_dir,
        )?;
    }

    Ok(())
}

fn create_file<FnFilter>(
    parts: &[Part],
    file_name: &str,
    filter: FnFilter,
    output_dir: &Path,
) -> Result<(), Error>
where
    FnFilter: Fn(&HashMap<u32, HashSet<String>>) -> bool,
{
    let path = output_dir.join(format!("price_{file_name}.csv"));
    let mut writer = Writer::from_path(path).map_err(|e| Error::CreateOutputCsv { source: e })?;

    let filtered_parts = parts
        .iter()
        .filter(|p| filter(&p.option))
        .cloned()
        .collect::<Vec<Part>>();

    let mut hash: BTreeMap<String, Part> = BTreeMap::new();

    for part in filtered_parts {
        hash.entry(part.part_ipn.clone())
            .and_modify(|p| p.quantity += part.quantity)
            .or_insert(part);
    }

    let mut records = hash
        .values()
        .map(|p| FileRecord {
            part_ipn: p.part_ipn.clone(),
            quantity: p.quantity,
            pricing_min: p.pricing_min,
            pricing_max: p.pricing_max,
            pricing_min_all: p.pricing_min * p.quantity as f32,
            pricing_max_all: p.pricing_max * p.quantity as f32,
        })
        .collect::<Vec<FileRecord>>();

    let pricing_min_all = records.iter().fold(0.0, |acc, r| acc + r.pricing_min_all);
    let pricing_max_all = records.iter().fold(0.0, |acc, r| acc + r.pricing_max_all);
    let sum_file_record = FileRecord {
        part_ipn: "Итого".into(),
        quantity: 0,
        pricing_min: 0.0,
        pricing_max: 0.0,
        pricing_min_all,
        pricing_max_all,
    };
    records.push(sum_file_record);

    // Записываем в файл
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
    pub quantity: u32,
    #[serde(serialize_with = "round_serialize")]
    pub pricing_min: f32,
    #[serde(serialize_with = "round_serialize")]
    pub pricing_max: f32,
    #[serde(serialize_with = "round_serialize")]
    pub pricing_min_all: f32,
    #[serde(serialize_with = "round_serialize")]
    pub pricing_max_all: f32,
}

fn round_serialize<S>(x: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let v = format!("{:.3}", x);
    s.serialize_str(&v)
}

// let mut records = parts
//     .iter()
//     .filter_map(|p| {
//         if !filter(&p.option) {
//             return None;
//         }
//         let r = FileRecord {
//             part_ipn: p.part_ipn.clone(),
//             quantity: p.quantity,
//             pricing_min: p.pricing_min,
//             pricing_max: p.pricing_max,
//             pricing_min_all: p.pricing_min * p.quantity as f32,
//             pricing_max_all: p.pricing_max * p.quantity as f32,
//         };
//         Some(r)
//     })
//     .collect::<Vec<FileRecord>>();
