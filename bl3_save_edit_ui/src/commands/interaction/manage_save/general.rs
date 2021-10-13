use uuid::Uuid;

pub fn generate_random_guid() -> String {
    let hex = format!("{:X}", Uuid::new_v4());
    hex.replace("-", "")
}
