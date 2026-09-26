use crate::models::violation::Violation;
use std::future::Future;
use uuid::Uuid;

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
