use super::TrafficSqliteStorage;
use crate::contracts::PenaltiesStorage;
use crate::models::penalty::Penalty;
use async_sqlite::rusqlite::{params_from_iter, types::Value};
use chrono::{DateTime, Utc};
use uuid::Uuid;

impl PenaltiesStorage for TrafficSqliteStorage {
    async fn add(&self, penalty: Penalty) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            INSERT INTO penalties (id, violation_id, datetime, driver_license_number, district, fine_amount, is_paid, suspension_months, officer_id, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(11);
        parameters.push(penalty.id.to_string().into());
        parameters.push(penalty.violation_id.to_string().into());
        parameters.push(penalty.datetime.to_rfc3339().into());
        parameters.push(penalty.driver_id.to_string().into());
        parameters.push(penalty.district.into());
        parameters.push((penalty.fine_amount as f64).into());
        parameters.push(Value::Integer(if penalty.is_paid { 1 } else { 0 }));
        parameters.push((penalty.suspension_months as i64).into());
        parameters.push(penalty.officer_id.into());
        parameters.push(penalty.created_at.to_rfc3339().into());
        parameters.push(penalty.updated_at.map(|dt| dt.to_rfc3339()).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(&self, id: Uuid) -> Result<Penalty, Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            SELECT id, violation_id, datetime, driver_license_number, district, fine_amount, is_paid, suspension_months, officer_id, created_at, updated_at
            FROM penalties WHERE id = ?1
        ";

        let parameters: Vec<Value> = vec![id.to_string().into()];

        let row_data = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, params_from_iter(parameters), |row| {
                    let pid: String = row.get(0)?;
                    let vid: String = row.get(1)?;
                    let dt: String = row.get(2)?;
                    let did: String = row.get(3)?;
                    let dist: String = row.get(4)?;
                    let fine: f64 = row.get(5)?;
                    let paid: i64 = row.get(6)?;
                    let susp: i64 = row.get(7)?;
                    let officer: String = row.get(8)?;
                    let created: String = row.get(9)?;
                    let updated: Option<String> = row.get(10)?;
                    Ok((
                        pid, vid, dt, did, dist, fine, paid, susp, officer, created, updated,
                    ))
                })
            })
            .await?;

        Ok(Penalty {
            id: Uuid::parse_str(&row_data.0)?,
            violation_id: Uuid::parse_str(&row_data.1)?,
            datetime: DateTime::parse_from_rfc3339(&row_data.2)?.with_timezone(&Utc),
            driver_id: Uuid::parse_str(&row_data.3)?,
            district: row_data.4,
            fine_amount: row_data.5 as f32,
            is_paid: row_data.6 == 1,
            suspension_months: row_data.7 as u32,
            officer_id: row_data.8,
            created_at: DateTime::parse_from_rfc3339(&row_data.9)?.with_timezone(&Utc),
            updated_at: row_data.10.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        })
    }

    async fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "DELETE FROM penalties WHERE id = ?1";
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
        penalty: &Penalty,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            UPDATE penalties 
            SET is_paid = ?1, updated_at = ?2
            WHERE id = ?3
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(3);
        parameters.push(Value::Integer(if penalty.is_paid { 1 } else { 0 }));
        parameters.push(penalty.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(penalty.id.to_string().into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
