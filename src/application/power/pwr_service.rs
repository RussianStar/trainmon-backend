use crate::domain::model::partial::partial_result::PartialResult;
use crate::domain::model::results::power_result::PowerResult;
pub fn process_power_data(results: &[PartialResult], zones: &[u16]) -> PowerResult {
    let results_a: Vec<_> = results.iter()
        .filter_map(|res| {
            if let PartialResult::PowerData(res_a) = res {
                Some(res_a)
            } else {
                //println!("Debug: No PowerData found in results");
                None
            }
        })
        .collect();


    if results_a.is_empty() {
       PowerResult { average: 0, weighted_average: 0, normalized: 0, time_in_zone: vec![], time_in_zone_effective: vec![] }
    } else {
        let sum: u64 = results_a.iter().map(|res| res.current_power as u64).sum();
        let avg: f32 = (sum as f32) / (results_a.len() as f32);
        let mut zone_counts = vec![0; zones.len() - 1];
        for res in &results_a {
            for i in 0..(zones.len() - 1) {
                if res.current_power >= zones[i] && res.current_power < zones[i + 1] {
                    zone_counts[i] += 1;
                }
            }
        }
        let time_in_zone: Vec<f32> = zone_counts.iter().map(|&count| (count as f32) / (results_a.len() as f32) * 100.0).collect();
        PowerResult { 
            average: avg.round() as u16,
            weighted_average: 0,
            normalized: 0,
            time_in_zone: time_in_zone,
            time_in_zone_effective: vec![] }
    }
}
