#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Ошибка создания CSV файла: {source}")]
    CreateOutputCsv { source: csv::Error },

    #[error("Ошибка десериализации CSV записи: {0}")]
    CsvDeser(csv::Error),

    #[error("Ошибка записи в CSV файл: {source}")]
    FlushFile { source: std::io::Error },

    #[error("Ошибка открытия CSV файла {file_path}: {source}")]
    OpenCsv {
        source: std::io::Error,
        file_path: String,
    },

    #[error("Ошибка парсинга опций для {part_ipn}: {options}")]
    ParseOptions { part_ipn: String, options: String },

    #[error("Ошибка запроса к API для {part_ipn}: {source}")]
    Request {
        source: reqwest::Error,
        part_ipn: String,
    },

    #[error("Ошибка удаления директории {dir_path}: {source}")]
    RemoveOutpurDir {
        source: std::io::Error,
        dir_path: String,
    },

    #[error("Ошибка десериализации ответа от API для {part_ipn}: {source}")]
    PartPkDeser {
        source: reqwest::Error,
        part_ipn: String,
    },

    #[error("Ошибка записи в CSV файл: {source}")]
    WriteToCsv { source: csv::Error },
}
