use chrono::{DateTime, Utc};
use uuid::Uuid;

#[doc = "Взыскание (Факт совершения нарушения)"]
pub struct Penalty {
    pub driver_id: Uuid,
    pub id: Uuid,
    pub violation_id: Uuid, // Связь с Нарушением
    pub datetime: DateTime<Utc>,
    pub district: String,
    pub fine_amount: f32,
    pub is_paid: bool,
    pub suspension_months: u32,
    pub officer_id: String,

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Penalty {
    pub fn new(
        violation_id: uuid::Uuid,
        driver_id: uuid::Uuid,
        district: String,
        fine_amount: f32,
        suspension_months: u32,
        officer_id: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            violation_id,
            datetime: chrono::Utc::now(),
            driver_id,
            district,
            fine_amount,
            is_paid: false, // по умолчанию штраф не оплачен
            suspension_months,
            officer_id,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }
}
