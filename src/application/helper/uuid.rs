use uuid::Uuid;

use crate::domain::model::results::general_result::GeneralResult;


pub fn create_uuid(result: &GeneralResult) -> Uuid {
    let namespace = Uuid::NAMESPACE_DNS;
    let data = serde_json::to_string(&result).expect("Failed to serialize data");

    Uuid::new_v5(&namespace, data.as_bytes())
}