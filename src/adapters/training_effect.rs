use crate::ports::training_effect::TrainingEffect;
use anyhow::{Result, Context};
use linregress::{FormulaRegressionBuilder, RegressionDataBuilder};
use polars::frame::DataFrame;
use polars::prelude::*;
use polars::prelude::col;

#[derive(Debug)]
struct ProgressAnalyzer {
}

impl TrainingEffect for ProgressAnalyzer {
    fn calculate_effect(performance_indicator: String, load_indicator: String, form_indicator: String) -> Result<DataFrame, anyhow::Error>{

        // Number of days to average over for the performance metric.
        let rolling = "PERF_ROLLING";
        let average_range: i32 = 30;
        let k1 = "K1";
        // Inputs :
        // - performance measure estimate, this also includes preprocessing like for vo2abs =
        // vo2rel * weight
        // - CTL
        // - TSB
        //
        // Outputs :
        // - dataframe with column : "K1" that contains how much they depend.

        let df = get_data_frame(performance_indicator, load_indicator, performance_indicator)?;
        // This is the accumulated load, usually CTL
        let load: Vec<f64> = df.column(&load_indicator)?.f64().context("")?.into_no_null_iter().collect();
        // This is the current form in TSB
        let form: Vec<f64> = df.column(&form_indicator)?.f64().context("")?.into_no_null_iter().collect();
        // Estimate of a hard performance metric like VO2.
        let performance: Vec<f64> = df.column(&performance_indicator)?.f64().context("")?.into_no_null_iter().collect();
        
        let data = RegressionDataBuilder::new()
            .build_from(vec![("X",load), ("Y", performance)])
            .expect("Input data is valid");

        let regression = FormulaRegressionBuilder::new()
            .data(&data)
            .fit()
            .expect("Regression analysis is successful");
        let intercept: &f64 = regression.parameters().first().context("Could not find the intercept")?;

        let df= df.lazy()
            .with_column(col(&performance_indicator).rolling_mean(average_range as u32).alias(rolling))
            .collect()?;

        let df_with_k1 = df.lazy()
            .with_column(
                ((col(rolling) - lit(intercept)) / col(&load_indicator)).alias(k1)
            )
            .collect()?;

        println!("{:?}", df_with_k1);

        Ok(df_with_k1)
    }

}

fn get_data_frame(performance_label: String, load_label: String, form_label: String) -> Result<DataFrame> {
    todo!()
}
