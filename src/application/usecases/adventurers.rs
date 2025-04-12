use crate::domain::{
    entities::adventurers::AdventurerEntity, repositories::adventurers::AdventurersRepository, value_objects::adventurer_model::RegisterAdventurerModel
};
use anyhow::Result;
use std::sync::Arc;

pub struct AdventurersUsecase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    adventurers_repository: Arc<T>,
}

impl<T> AdventurersUsecase<T>
where
    T: AdventurersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T>) -> Self {
        AdventurersUsecase {
            adventurers_repository,
        }
    }

    pub async fn register(
        &self,
        mut register_adventurer_model: RegisterAdventurerModel,
    ) -> Result<i32> {
        unimplemented!()
    }

}
