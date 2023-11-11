#[derive(Debug)]
pub struct HrData {
    pub current: u8,
    pub average: u8,
    pub zone_percentages: Vec<f32>
}