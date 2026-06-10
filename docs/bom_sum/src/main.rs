use std::{
    collections::BTreeMap,
    fs::{File, read_dir},
    io::{self, Write},
    path::{Path, PathBuf},
};

use csv::Reader;
use serde::Deserialize;
use tracing::warn;

fn main() {
    tracing_subscriber::fmt().init();

    let dir = Path::new("../pcb");

    visit_dirs(dir).unwrap();
}

fn visit_dirs(dir: &Path) -> io::Result<()> {
    let mut all_parts: BTreeMap<String, Vec<(String, u32)>> = BTreeMap::new();

    if dir.is_dir() {
        for entry in read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            let bom_path = path_to_bom_file(&path)?;
            let bom_file = match bom_path {
                Some(bom_file) => bom_file,
                None => {
                    warn!("Файл BOM не найден на пути: {:?}", path);
                    continue;
                }
            };

            let parts = read_csv(&bom_file)?;

            for part in parts {
                all_parts
                    .entry(part.part_ipn)
                    .and_modify(|e| e.push((bom_file.module_name.clone(), part.quantity)))
                    .or_insert(vec![(bom_file.module_name.clone(), part.quantity)]);
            }
        }
    }

    // Сортируем по названию модуля
    let all_parts = all_parts
        .iter_mut()
        .map(|(k, v)| {
            v.sort();
            (k.clone(), v.clone())
        })
        .collect::<BTreeMap<_, _>>();

    let toml_file = toml::to_string(&all_parts).unwrap();

    let _ = File::create(Path::new("all_parts.toml"))?.write(toml_file.as_bytes());

    Ok(())
}

/// Путь к файлу BOM для заданного модуля
fn path_to_bom_file(dir: &Path) -> io::Result<Option<BomFile>> {
    let module_name = dir.file_name().unwrap().to_str().unwrap();
    let module_file = format!("{}.BOM.csv", module_name);

    let mut dir = dir.to_path_buf();
    dir.push(module_file);

    let res = match dir.exists() {
        true => {
            let bom_file = BomFile {
                path: dir,
                module_name: module_name.to_string(),
            };
            Some(bom_file)
        }
        false => None,
    };

    Ok(res)
}

fn read_csv(bom_file: &BomFile) -> io::Result<Vec<CsvItem>> {
    let file = File::open(bom_file.path.clone())?;
    let mut rdr: Reader<File> = csv::Reader::from_reader(file);

    let mut records = vec![];

    for result in rdr.deserialize() {
        let record: CsvItem = result?;
        records.push(record);
    }

    Ok(records)
}

struct BomFile {
    path: PathBuf,
    module_name: String,
}

#[derive(Debug, Deserialize)]
struct CsvItem {
    part_ipn: String,
    quantity: u32,
}
