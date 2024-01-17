use uuid::Uuid;

use serde::Serialize;

pub fn create_uuid<T: Serialize>(item: &T) -> Uuid {
    let namespace = Uuid::NAMESPACE_DNS;
    let data = serde_json::to_string(&item).expect("Failed to serialize data");

    Uuid::new_v5(&namespace, data.as_bytes())
}
