use std::sync::Arc;

use crate::domain::repositories::{adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository};

pub struct AuthenticationUsecase<T1,T2> where
    T1: AdventurersRepository + Send + Sync,
    T2:GuildCommandersRepository + Send + Sync,
{
    adventurers_repository: Arc<T1>,
    crew_switchboard_repository: Arc<T2>,
}

impl<T1,T2> AuthenticationUsecase<T1,T2> where
    T1: AdventurersRepository + Send + Sync,
    T2:GuildCommandersRepository + Send + Sync,
{
    pub fn new(adventurers_repository: Arc<T1>, crew_switchboard_repository: Arc<T2>) -> Self {
        AuthenticationUsecase {
            adventurers_repository,
            crew_switchboard_repository,
        }
    }

    pub async fn adventurer_login(&self) {
        unimplemented!()
    }

    pub async fn adventurer_refresh_token(&self) {
        unimplemented!()
    }

    pub async fn guild_commander_login(&self) {
        unimplemented!()
    }

    pub async fn guild_commander_refresh_token(&self) {
        unimplemented!()
    }

}