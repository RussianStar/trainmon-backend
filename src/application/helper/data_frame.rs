use polars::prelude::*;
use sqlx::PgPool;
use uuid::Uuid;

pub fn create_dataframe_from_csv(contents: &[u8]) -> DataFrame {
    let csv_reader = CsvReader::new(std::io::Cursor::new(contents))
        .has_header(true);
    csv_reader.finish().unwrap()
}
