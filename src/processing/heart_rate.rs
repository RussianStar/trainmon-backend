use fitparser::profile::MesgNum;

use super::super::model;

pub fn map_hr_zones(data: Vec<fitparser::FitDataRecord>, zones: Vec<u8>) -> model::heart_rate_data::hr_data {
    let heart_rates: Vec<u32> = data.iter()
    .filter_map(|record| match record {
        fitparser::FitDataRecord { .. }  => {
            let kind = record.kind();
            if let MesgNum::Record = kind {
                let heart_rate_field = record.fields().iter()
                .find(|data_field| data_field.name() == "heart_rate");
                match heart_rate_field {
                    Some(field) => {
                        match *field.value() {
                            fitparser::Value::UInt8(value) => {
                                Some(value as u32)
                            },
                            _ => {
                                println!("The value is not a u16");
                                None
                            }
                        }
                    },
                    None => {
                        println!("Heart rate field not found");
                        None
                    }
                }
            }
            else{
                None
            }
        },
        _ => None,
    })
    .collect();
    
    let mut zone_counts = vec![0; zones.len() - 1];
    
    for &rate in &heart_rates {
        for i in 0..(zones.len() - 1) {
            if rate > zones[i] as u32 && rate <= zones[i + 1] as u32 {
                zone_counts[i] += 1;
                break;
            }
        }
    }
    
    let total_heart_rate: u32 = heart_rates.iter().sum();
    let average_heart_rate = total_heart_rate as f32 / heart_rates.len() as f32;
    let zone_percentages: Vec<f32> = zone_counts.iter().map(|&count| (count as f32 / heart_rates.len() as f32) * 100.0).collect();
    
    println!("Average heart rate : {:.2}", average_heart_rate);
    for (i, &percentage) in zone_percentages.iter().enumerate() {
        println!("Percentage of heart rates in zone {} ({}-{}): {:.2}%", i + 1, zones[i], zones[i + 1], percentage);
    }

    model::heart_rate_data::hr_data {
        average: average_heart_rate as u8,
        zone_percentages,
    }
}

