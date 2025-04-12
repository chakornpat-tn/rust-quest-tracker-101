use std::sync::Arc;
use anyhow::Result;
use crate::domain::{repositories::guild_commanders::GuildCommandersRepository, value_objects::guild_commander_model::RegisterGuildCommanderModel};


pub struct GuildCommandersUsecase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    guild_commanders_repository: Arc<T>,
}

impl<T> GuildCommandersUsecase<T>
where
    T: GuildCommandersRepository + Send + Sync,
{
    pub fn new(guild_commanders_repository: Arc<T>) -> Self {
        GuildCommandersUsecase {
            guild_commanders_repository,
        }
    }

    pub async fn register(&self, guild_commander_register_model:RegisterGuildCommanderModel) -> Result<i32> {
        unimplemented!()
    }
}