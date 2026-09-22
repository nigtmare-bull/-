use chrono::{DateTime, Utc};

#[doc = "Водитель"]
pub struct Driver {
    // Номер водительского удостоверения (Уникальный ключ)
    pub license_number: String,
    pub full_name: String,
    pub address: String,
    pub phone_number: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
impl Driver {
    pub fn new(
        license_number: String,
        full_name: String,
        address: String,
        phone_number: String,
    ) -> Self {
        Self {
            license_number,
            full_name,
            address,
            phone_number,
            created_at: chrono::Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
