pub mod contracts;
pub mod contracts_impl;
pub mod models;

use contracts_impl::TrafficSqliteStorage;

fn main() {
    smol::block_on(async {
        let storage = TrafficSqliteStorage::new("database/data.db").await;
        println!("база данных успешно инициализирована, пул подключений готов!");
    });
}
