#![warn(clippy::unwrap_used)]

use std::{
    collections::{HashMap, HashSet},
    fs::{create_dir_all, remove_dir_all},
    path::Path,
};

use clap::Parser;
use serde::Serialize;
use tracing::debug;

mod error;
mod read_bom_file;
mod req_part_pk;
mod req_sku;
mod write_lcsc_file;
mod write_price_files;

const INVENTREE_URL: &str = "http://inventree.network/api";
const SUPPLIER_LCSC: u32 = 12;

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    let args = Args::parse();

    let inventree_token = format!("Token {}", args.token);
    let output_dir = Path::new(&args.output);

    let client = reqwest::blocking::Client::new();

    // удаляем и создаем папку
    if output_dir.exists() {
        remove_dir_all(output_dir).map_err(|e| error::Error::RemoveOutpurDir {
            source: e,
            dir_path: output_dir.to_string_lossy().into_owned(),
        })?;
    }
    create_dir_all(output_dir)?;

    // читаем csv файл
    let parts_csv = read_bom_file::read_bom_file(&args.input)?;
    debug!("BOM file read: {:?}", parts_csv);

    // узнаём в API идентификаторы pk
    let mut parts_with_pk = vec![];
    for part in parts_csv {
        let pk = req_part_pk::req_part_pk(&client, part, &inventree_token, &args.url)?;
        if let Some(part) = pk {
            parts_with_pk.push(part);
        }
    }

    // получаем sku для поставщика LCSC
    let mut parts_with_sku = vec![];
    for part in parts_with_pk {
        let lcsc_sku = req_sku::req_sku(&client, part, &inventree_token, &args.url)?;
        parts_with_sku.push(lcsc_sku);
    }

    debug!("Parts with SKU: {:?}", parts_with_sku);

    // записываем файл для загрузки в LCSC
    write_lcsc_file::write_lcsc_file(&parts_with_sku, output_dir)?;

    // записываем файлы с ценами для разных опций
    write_price_files::write_price_files(&parts_with_sku, output_dir)?;

    Ok(())
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Part {
    pub part_ipn: String,
    pub pk: u32,
    pub quantity: u32,
    pub lcsc_sku: String,
    pub pricing_min: f32,
    pub pricing_max: f32,
    pub option: HashMap<u32, HashSet<String>>,
}

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Входной файл спецификации
    #[arg(short, long)]
    input: String,

    /// Путь к папке для сохранения результатов
    #[arg(short, long)]
    output: String,

    /// Токен для доступа к API
    #[arg(short, long)]
    token: String,

    /// URL для доступа к API
    #[arg(short, long, default_value = INVENTREE_URL)]
    url: String,

    #[arg(short, long, default_value_t = SUPPLIER_LCSC)]
    supplier_lcsc: u32,
}
