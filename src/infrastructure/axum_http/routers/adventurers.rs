use axum::{extract::State, response::IntoResponse, routing::post, Json, Router};
use std::sync::Arc;

use crate::{
    application::usecases::adventurers::AdventurersUsecase,
    domain::{
        repositories::adventurers::AdventurersRepository,
        value_objects::adventurer_model::RegisterAdventurerModel,
    },
    infrastructure::postgres::{
        postgres_connector::PgPoolSquad, repositories::adventurers::AdventurerPostgres,
    },
};

pub fn routers(db_pool: Arc<PgPoolSquad>) -> Router {
    let adventurers_repo = AdventurerPostgres::new(db_pool);
    let adventurers_use_case = AdventurersUsecase::new(Arc::new(adventurers_repo));

    Router::new()
        .route("/", post(register))
        .with_state(Arc::new(adventurers_use_case))
}

pub async fn register<T>(
    State(adventurers_use_case): State<Arc<AdventurersUsecase<T>>>,
    Json(register_adventurer_model): Json<RegisterAdventurerModel>,
) -> impl IntoResponse
where
    T: AdventurersRepository + Send + Sync,
{
    unimplemented!()
}

