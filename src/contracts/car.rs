use crate::models::car::Car;
use std::future::Future;

pub trait CarsStorage {
    fn add(
        &self,
        car: Car,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn get(
        &self,
        car_number: String,
    ) -> impl Future<Output = Result<Car, Box<dyn std::error::Error + Send + Sync>>>;
    fn remove(
        &self,
        car_number: String,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn update(
        &self,
        car: &Car,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
}
