use crate::models::driver::Driver;
use std::future::Future;

pub trait DriversStorage {
    fn add(
        &self,
        driver: Driver,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn get(
        &self,
        license_number: String,
    ) -> impl Future<Output = Result<Driver, Box<dyn std::error::Error + Send + Sync>>>;
    fn remove(
        &self,
        license_number: String,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn update(
        &self,
        driver: &Driver,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
}
