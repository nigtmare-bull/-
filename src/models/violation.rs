use chrono::{DateTime, Utc};
use uuid::Uuid;

#[doc = "Нарушение (Справочник видов нарушений)"]
pub struct Violation {
    pub id: Uuid,
    pub violation_type: String,
    pub base_fine_range: f32,     // диапазон долей базовой величины
    pub has_warning: bool,        // предупреждение (Да/Нет)
    pub ban_duration_months: u32, // срок лишения прав (от 12 до 36)

    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}
impl Violation {
    pub fn new(
        violation_type: String,
        base_fine_range: f32,
        has_warning: bool,
        ban_duration_months: u32,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(), // Автоматически генерируем уникальный UUID
            violation_type,
            base_fine_range,
            has_warning,
            ban_duration_months,
            created_at: chrono::Utc::now(),
            updated_at: None,
        }
    }
}
