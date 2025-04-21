use anyhow::{Ok, Result};
use axum::async_trait;
use diesel::{insert_into, ExpressionMethods, RunQueryDsl};
use std::sync::Arc;

use crate::{
    domain::{
        entities::quests::{AddQuestEntity, EditQuestEntity},
        repositories::quest_ops::QuestOpsRepository,
        value_objects::quest_statuses::QuestStatus,
    },
    infrastructure::postgres::postgres_connector::PgPoolSquad,
    schema::quests,
};
pub struct QuestOpsPostgres {
    db_pool: Arc<PgPoolSquad>,
}

impl QuestOpsPostgres {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl QuestOpsRepository for QuestOpsPostgres {
    async fn add(&self, add_quest_entity: AddQuestEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let res = insert_into(quests::table)
            .values(add_quest_entity)
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(res)
    }
    async fn edit(&self, quest_id: i32, edit_quest_entity: EditQuestEntity) -> Result<i32> {
        let mut conn = Arc::clone(&self.db_pool).get()?;
        let res = diesel::update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .filter(quests::status.eq(QuestStatus::Open.to_string()))
            .set(edit_quest_entity)
            .returning(quests::id)
            .get_result::<i32>(&mut conn)?;

        Ok(res)
    }
    async fn remove(&self, quest_id: i32, guild_commander_id: i32) -> Result<()> {
        let mut conn = Arc::clone(&self.db_pool).get()?;

        diesel::update(quests::table)
            .filter(quests::id.eq(quest_id))
            .filter(quests::deleted_at.is_null())
            .filter(quests::status.eq(QuestStatus::Open.to_string()))
            .set((
                quests::deleted_at.eq(chrono::Utc::now().naive_utc()),
                quests::guild_commander_id.eq(guild_commander_id),
            ))
            .execute(&mut conn)?;

        Ok(())
    }
}
