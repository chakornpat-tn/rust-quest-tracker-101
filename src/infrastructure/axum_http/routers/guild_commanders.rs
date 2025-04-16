use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use std::sync::Arc;

use crate::{
    application::usecases::guild_commanders::GuildCommandersUsecase,
    domain::{
        repositories::guild_commanders::GuildCommandersRepository,
        value_objects::guild_commander_model::RegisterGuildCommanderModel,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::guild_commanders::GuildCommandersPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let guild_commanders_repo = GuildCommandersPostgres::new(db_pool);
    let guild_commanders_usecase = GuildCommandersUsecase::new(Arc::new(guild_commanders_repo));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(guild_commanders_usecase))
}

pub async fn register<T>(
    State(guild_commanders_use_case): State<Arc<GuildCommandersUsecase<T>>>,
    Json(register_adventurer_model): Json<RegisterGuildCommanderModel>,
) -> impl IntoResponse
where
    T: GuildCommandersRepository + Send + Sync,
{
    unimplemented!()
}
