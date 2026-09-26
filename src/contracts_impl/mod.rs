use async_sqlite::rusqlite::{params_from_iter, types::Value};
use async_sqlite::{Pool, PoolBuilder};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use uuid::Uuid;

use crate::contracts::{CarsStorage, DriversStorage, PenaltiesStorage, ViolationsStorage};
use crate::models::car::Car;
use crate::models::driver::Driver;
use crate::models::penalty::Penalty;
use crate::models::violation::Violation;

pub struct TrafficSqliteStorage {
    connection: Pool,
}

impl TrafficSqliteStorage {
    /// Инициализация пула подключений к SQLite
    pub async fn new(database_url: &str) -> Self {
        let path_buf: PathBuf = PathBuf::from(database_url);
        let builder: PoolBuilder = PoolBuilder::new()
            .path(path_buf)
            .journal_mode(async_sqlite::JournalMode::Wal)
            .num_conns(10);

        let connection: Pool = builder.open().await.expect("Ошибка инициализации БД");
        Self { connection }
    }
}
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
