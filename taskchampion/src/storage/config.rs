use super::{InMemoryStorage, KVStorage, Storage};
use failure::Fallible;
use std::path::PathBuf;

/// The configuration required for a replica's storage.
pub enum StorageConfig {
    /// Store the data on disk.  This is the common choice.
    OnDisk {
        /// Path containing the task DB.
        taskdb_dir: PathBuf,
    },
    /// Store the data in memory.  This is only useful for testing.
    InMemory,
}

impl StorageConfig {
    pub fn into_storage(self) -> Fallible<Box<dyn Storage>> {
        Ok(match self {
            StorageConfig::OnDisk { taskdb_dir } => Box::new(KVStorage::new(taskdb_dir)?),
            StorageConfig::InMemory => Box::new(InMemoryStorage::new()),
        })
    }
}
