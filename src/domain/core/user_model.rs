#[derive(Clone)]
pub struct UserModel{
    pub name: String,
    pub hr_zones: Vec<u8>,
    pub pwr_zones: Vec<u16>
}