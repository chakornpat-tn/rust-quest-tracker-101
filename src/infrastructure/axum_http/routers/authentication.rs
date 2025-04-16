use std::sync::Arc;

use axum::{extract::State, response::IntoResponse, routing::post, Router};

use crate::{
    application::usecases::authentication::{self, AuthenticationUsecase},
    domain::repositories::{
        adventurers::AdventurersRepository, guild_commanders::GuildCommandersRepository,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad,
        repositories::{
            adventurers::AdventurerPostgres, guild_commanders::GuildCommandersPostgres,
        },
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repo = AdventurerPostgres::new(Arc::clone(&db_pool));
    let guild_commanders_repo = GuildCommandersPostgres::new(Arc::clone(&db_pool));
    let authentication_use_case =
        AuthenticationUsecase::new(Arc::new(adventurers_repo), Arc::new(guild_commanders_repo));

    Router::new()
        .route("/adventurers/login", post(adventurers_login))
        .route("/adventurers/refresh-token", post(adventurers_refresh_token))
        .route("/guild_commanders/login", post(guild_commanders_login))
        .route("/guild_commanders/refresh-token", post(guild_commanders_refresh_token))
        .with_state(Arc::new(authentication_use_case))
}

pub async fn adventurers_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUsecase<T1, T2>>>,
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn adventurers_refresh_token<T1,T2>(
    State(authentication_use_case): State<Arc<AuthenticationUsecase<T1, T2>>>,
) -> impl IntoResponse
where 
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_login<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUsecase<T1, T2>>>
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}

pub async fn guild_commanders_refresh_token<T1, T2>(
    State(authentication_use_case): State<Arc<AuthenticationUsecase<T1, T2>>>
) -> impl IntoResponse
where
    T1: AdventurersRepository + Send + Sync,
    T2: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}