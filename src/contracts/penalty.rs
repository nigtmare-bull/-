use crate::models::penalty::Penalty;
use std::future::Future;
use uuid::Uuid;

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
