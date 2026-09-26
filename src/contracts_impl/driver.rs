use super::TrafficSqliteStorage;
use crate::contracts::DriversStorage;
use crate::models::driver::Driver;
use async_sqlite::rusqlite::{params_from_iter, types::Value};
use chrono::{DateTime, Utc};

impl DriversStorage for TrafficSqliteStorage {
    async fn add(&self, driver: Driver) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            INSERT INTO drivers (license_number, full_name, address, phone_number, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(7);
        parameters.push(driver.license_number.into());
        parameters.push(driver.full_name.into());
        parameters.push(driver.address.into());
        parameters.push(driver.phone_number.into());
        parameters.push(driver.created_at.to_rfc3339().into());
        parameters.push(driver.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(driver.deleted_at.map(|dt| dt.to_rfc3339()).into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn get(
        &self,
        license_number: String,
    ) -> Result<Driver, Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            SELECT license_number, full_name, address, phone_number, created_at, updated_at, deleted_at 
            FROM drivers WHERE license_number = ?1
        ";

        let parameters: Vec<Value> = vec![license_number.into()];

        let row_data = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, params_from_iter(parameters), |row| {
                    let license: String = row.get(0)?;
                    let name: String = row.get(1)?;
                    let addr: String = row.get(2)?;
                    let phone: String = row.get(3)?;
                    let created: String = row.get(4)?;
                    let updated: Option<String> = row.get(5)?;
                    let deleted: Option<String> = row.get(6)?;
                    Ok((license, name, addr, phone, created, updated, deleted))
                })
            })
            .await?;

        Ok(Driver {
            license_number: row_data.0,
            full_name: row_data.1,
            address: row_data.2,
            phone_number: row_data.3,
            created_at: DateTime::parse_from_rfc3339(&row_data.4)?.with_timezone(&Utc),
            updated_at: row_data.5.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            deleted_at: row_data.6.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        })
    }

    async fn remove(
        &self,
        license_number: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "DELETE FROM drivers WHERE license_number = ?1";
        let parameters: Vec<Value> = vec![license_number.into()];

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
        driver: &Driver,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            UPDATE drivers 
            SET full_name = ?1, address = ?2, phone_number = ?3, updated_at = ?4, deleted_at = ?5
            WHERE license_number = ?6
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(6);
        parameters.push(driver.full_name.clone().into());
        parameters.push(driver.address.clone().into());
        parameters.push(driver.phone_number.clone().into());
        parameters.push(driver.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(driver.deleted_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(driver.license_number.clone().into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
