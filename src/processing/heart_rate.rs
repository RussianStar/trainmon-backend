use fitparser::profile::MesgNum;

use crate::model;

pub fn process_heart_rate_data(results: &[model::enums::PartialResult], zones: &[u8]) -> model::heart_rate_data::HrData {
    let results_a: Vec<_> = results.iter()
        .filter_map(|res| {
            if let model::enums::PartialResult::HeartRateData(res_a) = res {
                Some(res_a)
            } else {
                println!("Debug: No HeartRateData found in results");
                None
            }
        })
        .collect();

    if results_a.is_empty() {
        model::heart_rate_data::HrData { current: 0, average: 0, zone_percentages: vec![] }
    } else {
        let sum: u64 = results_a.iter().map(|res| res.current as u64).sum();
        let avg: f32 = (sum as f32) / (results_a.len() as f32);
        let mut zone_counts = vec![0; zones.len() - 1];
        for res in &results_a {
            for i in 0..(zones.len() - 1) {
                if res.current >= zones[i] && res.current < zones[i + 1] {
                    zone_counts[i] += 1;
                }
            }
        }
        let zone_percentages: Vec<f32> = zone_counts.iter().map(|&count| (count as f32) / (results_a.len() as f32) * 100.0).collect();
        model::heart_rate_data::HrData { 
            average: avg.round() as u8,
            current:0,
            zone_percentages: zone_percentages }
    }
}