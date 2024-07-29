use crate::entity::EntityManager;

mod entity;
mod component;

fn main() {
    let mut entity_manager = EntityManager::new();

    let _ = entity_manager.create_entity();
}