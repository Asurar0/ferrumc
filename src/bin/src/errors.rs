use thiserror::Error;
use ferrumc_core::errors::CoreError;
use ferrumc_ecs::errors::ECSError;
use ferrumc_net_encryption::errors::EventsError;
use ferrumc_plugins::errors::NetError;
use ferrumc_storage::errors::PluginsError;
use ferrumc_utils::errors::StorageError;
use ferrumc_world::errors::UtilsError;

#[derive(Debug, Error)]
pub enum BinaryError {
    #[error("Core error: {0}")]
    Core(#[from] CoreError),

    #[error("ECS error: {0}")]
    Ecs(#[from] ECSError),

    #[error("Events error: {0}")]
    Events(#[from] EventsError),

    #[error("Net error: {0}")]
    Net(#[from] NetError),

    #[error("Plugins error: {0}")]
    Plugins(#[from] PluginsError),

    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("Utils error: {0}")]
    Utils(#[from] UtilsError),

    #[error("World error: {0}")]
    World(#[from] WorldError),
    
    #[error("{0}")]
    Custom(String),

    // #[error("IO error: {0}")]
    // Io(#[from] std::io::Error),

    // idk add ur own errors here
}