use std::future::Future;
use uuid::Uuid;

use crate::models::car::Car;
use crate::models::driver::Driver;
use crate::models::penalty::Penalty;
use crate::models::violation::Violation;

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

pub trait ViolationsStorage {
    fn add(
        &self,
        violation: Violation,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Violation, Box<dyn std::error::Error + Send + Sync>>>;
    fn remove(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn update(
        &self,
        violation: &Violation,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
}

pub trait PenaltiesStorage {
    fn add(
        &self,
        penalty: Penalty,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn get(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<Penalty, Box<dyn std::error::Error + Send + Sync>>>;
    fn remove(
        &self,
        id: Uuid,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
    fn update(
        &self,
        penalty: &Penalty,
    ) -> impl Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>>;
}
