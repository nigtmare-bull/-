use super::TrafficSqliteStorage;
use crate::contracts::CarsStorage;
use crate::models::car::Car;
use async_sqlite::rusqlite::{params_from_iter, types::Value};
use chrono::{DateTime, Utc};

impl CarsStorage for TrafficSqliteStorage {
    async fn add(&self, car: Car) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            INSERT INTO cars (car_number, brand, model, color, year_of_manufacture, registration_date, created_at, updated_at, deleted_at)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(9);
        parameters.push(car.car_number.into());
        parameters.push(car.brand.into());
        parameters.push(car.model.into());
        parameters.push(car.color.into());
        parameters.push((car.year_of_manufacture as i64).into());
        parameters.push(car.registration_date.to_rfc3339().into());
        parameters.push(car.created_at.to_rfc3339().into());
        parameters.push(car.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(car.deleted_at.map(|dt| dt.to_rfc3339()).into());

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
        car_number: String,
    ) -> Result<Car, Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            SELECT car_number, brand, model, color, year_of_manufacture, registration_date, created_at, updated_at, deleted_at 
            FROM cars WHERE car_number = ?1
        ";

        let parameters: Vec<Value> = vec![car_number.into()];

        let row_data = self
            .connection
            .conn(move |conn| {
                conn.query_row(QUERY, params_from_iter(parameters), |row| {
                    let num: String = row.get(0)?;
                    let brand: String = row.get(1)?;
                    let model: String = row.get(2)?;
                    let color: String = row.get(3)?;
                    let year: i64 = row.get(4)?;
                    let reg_date: String = row.get(5)?;
                    let created: String = row.get(6)?;
                    let updated: Option<String> = row.get(7)?;
                    let deleted: Option<String> = row.get(8)?;
                    Ok((
                        num, brand, model, color, year, reg_date, created, updated, deleted,
                    ))
                })
            })
            .await?;

        Ok(Car {
            car_number: row_data.0,
            brand: row_data.1,
            model: row_data.2,
            color: row_data.3,
            year_of_manufacture: row_data.4 as u32,
            registration_date: DateTime::parse_from_rfc3339(&row_data.5)?.with_timezone(&Utc),
            created_at: DateTime::parse_from_rfc3339(&row_data.6)?.with_timezone(&Utc),
            updated_at: row_data.7.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            deleted_at: row_data.8.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
        })
    }

    async fn remove(
        &self,
        car_number: String,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "DELETE FROM cars WHERE car_number = ?1";
        let parameters: Vec<Value> = vec![car_number.into()];

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }

    async fn update(&self, car: &Car) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        const QUERY: &str = "
            UPDATE cars 
            SET brand = ?1, model = ?2, color = ?3, updated_at = ?4
            WHERE car_number = ?5
        ";

        let mut parameters: Vec<Value> = Vec::with_capacity(5);
        parameters.push(car.brand.clone().into());
        parameters.push(car.model.clone().into());
        parameters.push(car.color.clone().into());
        parameters.push(car.updated_at.map(|dt| dt.to_rfc3339()).into());
        parameters.push(car.car_number.clone().into());

        self.connection
            .conn(move |conn| {
                conn.execute(QUERY, params_from_iter(parameters))?;
                Ok(())
            })
            .await?;
        Ok(())
    }
}
