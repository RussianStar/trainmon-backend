use anyhow::Ok;
use axum::response::Html;
use anyhow::{Result, anyhow};

use serde::Deserialize;
use serde::Serialize;
use askama::Template;



#[derive(Template)]
#[template(path = "oura_upload.html")]
#[derive(Serialize, Deserialize,Debug)]
pub struct OuraUpload {
    pub path: String,
}

pub async fn import_oura() -> Html<String> {
    let template = OuraUpload { path: "".to_string()};
    Html(template.render().unwrap())
}

use polars::prelude::*;
pub fn transform_metrics(data: DataFrame) -> Result<(),anyhow::Error> {

    // Select rows where col1 is equal to col1value
    let mut weight_df = data.filter(&data.column("Type")?.equal("Weight Kilograms")?)?;
    weight_df.drop_in_place("Type")?;
    let rw =weight_df.rename("Value","Weight Kilograms")?;

    let mut hr_df = data.filter(&data.column("Type")?.equal("Pulse")?)?;
    hr_df.drop_in_place("Type")?;
    let hr = hr_df.rename("Value","Pulse")?;

    println!("{:?}", rw);
    println!("{:?}", hr);
    let joined_df = rw.join(&hr, ["Timestamp"], ["Timestamp"], JoinArgs::new(JoinType::Outer { coalesce: true }))?;
    println!("{:?}", joined_df);
    Ok(())
}

pub fn transform(data: DataFrame, metrics: Vec<&str>) -> Result<(),anyhow::Error> {
    //is_subset(&metrics, data.get_column_names())?;

    let mut dfs: Vec<DataFrame> = vec![];
    for &metric in metrics.iter() {
        let mut df = data.filter(&data.column("Type")?.equal(metric)?)?;
        let _ = DataFrame::drop_in_place(&mut df, "Type")?;
        let rw = df.rename("Value", metric)?;

        dfs.push(rw.clone());
    }

    let mut result: DataFrame = dfs.first().unwrap().clone();
    for metric_df in dfs.iter().skip(1) {
        result = result.join(&metric_df, ["Timestamp"], ["Timestamp"], JoinArgs::new(JoinType::Outer { coalesce: true }))?;
    }

    println!("{:?}",result);
    Ok(())
}

fn is_subset(subset: &Vec<&str>, set: Vec<&str>) -> Result<(),anyhow::Error>{
    if subset.iter().all(|item| set.contains(item)) {
        Ok(())
    } else {
        Err(anyhow!("The first vector is not a subset of the second"))
    }
}


use polars::prelude::*;
use std::error::Error;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_csv_and_print_first_line() -> Result<(), anyhow::Error> {
        // Specify the path to your CSV file
        let file_path = "C:\\work\\privat\\training-monitor\\users\\1\\oura\\oura.csv";
        let metrics = "C:\\work\\privat\\training-monitor\\users\\1\\metrics\\tp\\metrics.csv";

        // Read the CSV file into a DataFrame
        let df = CsvReader::from_path(metrics)?
            .infer_schema(None)
            .finish()?;
       
        println!("{:?}",df.head(Some(10)));

        let _ = transform(df, vec!["Sleep Hours", "Pulse", "Weight Kilograms"]);
        Ok(())
    }
}
