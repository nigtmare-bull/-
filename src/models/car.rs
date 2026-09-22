use chrono::{DateTime, Utc};

#[doc = "Автомобиль"]
pub struct Car {
    // Номер автомобиля (Уникальный ключ)
    pub car_number: String,
    pub brand: String,
    pub model: String,
    pub color: String,
    pub year_of_manufacture: u32,
    pub registration_date: DateTime<Utc>,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub deleted_at: Option<DateTime<Utc>>,
}
impl Car {
    pub fn new(
        car_number: String,
        brand: String,
        model: String,
        color: String,
        year_of_manufacture: u32,
    ) -> Self {
        Self {
            car_number,
            brand,
            model,
            color,
            year_of_manufacture,
            registration_date: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
            updated_at: None,
            deleted_at: None,
        }
    }
}
