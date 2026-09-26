use super::TrafficSqliteStorage;
use crate::contracts::ViolationsStorage;
use crate::models::violation::Violation;
use async_sqlite::rusqlite::{params_from_iter, types::Value};
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl ViolationsStorage for TrafficSqliteStorage {
    async fn add(
        &self,
        violation: Violation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            INSERT INTO violations (id, violation_type, base_fine_range, has_warning, ban_duration_months, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(7);
        parameters.push(violation.id.to_string().into());
        parameters.push(violation.violation_type.into());
        parameters.push((violation.base_fine_range as f64).into());
        parameters.push(Value::Integer(if violation.has_warning { 1 } else { 0 }));
        parameters.push((violation.ban_duration_months as i64).into());
        parameters.push(violation.created_at.to_rfc3339().into());
        parameters.push(violation.updated_at.map(|dt| dt.to_rfc3339()).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Violation, Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            SELECT id, violation_type, base_fine_range, has_warning, ban_duration_months, created_at, updated_at
            FROM violations WHERE id = ?1
        ";

        let parameters: Vec<Value> = vec![id.to_string().into()];

        let row_data = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, params_from_iter(parameters), |row| {
                    let id_str: String = row.get(0)?;
                    let v_type: String = row.get(1)?;
                    let fine: f64 = row.get(2)?;
                    let warn: i64 = row.get(3)?;
                    let ban: i64 = row.get(4)?;
                    let created: String = row.get(5)?;
                    let updated: Option<String> = row.get(6)?;
                    Ok((id_str, v_type, fine, warn, ban, created, updated))
                })
            })
            .await?;

        Ok(Violation {
            id: Uuid::parse_str(&row_data.0)?,
            violation_type: row_data.1,
            base_fine_range: row_data.2 as f32,
            has_warning: row_data.3 == 1,
            ban_duration_months: row_data.4 as u32,
            created_at: DateTime::parse_from_rfc3339(&row_data.5)?.with_timezone(&Utc),
            updated_at: row_data.6.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        })
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "DELETE FROM violations WHERE id = ?1";
        let parameters: Vec<Value> = vec![id.to_string().into()];

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(
        &self,
        violation: &Violation,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            UPDATE violations 
            SET violation_type = ?1, base_fine_range = ?2, has_warning = ?3, ban_duration_months = ?4, updated_at = ?5
            WHERE id = ?6
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(6);
        parameters.push(violation.violation_type.clone().into());
        parameters.push((violation.base_fine_range as f64).into());
        parameters.push(Value::Integer(if violation.has_warning { 1 } else { 0 }));
        parameters.push((violation.ban_duration_months as i64).into());
        parameters.push(violation.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(violation.id.to_string().into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
