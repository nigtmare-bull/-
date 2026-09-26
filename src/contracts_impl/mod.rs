use async_sqlite::{Pool, PoolBuilder};
use std::path::PathBuf;

// Подключаем файлы реализаций
pub mod car;
pub mod driver;
pub mod penalty;
pub mod violation;

pub struct TrafficSqliteStorage {
    pub(crate) connection: Pool,
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
