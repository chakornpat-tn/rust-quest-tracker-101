use anyhow::{Ok, Result};
use std::sync::Arc;

use crate::domain::{
    repositories::quest_viewing::QuestViewingRepository,
    value_objects::{board_checking_filter::BoardCheckingFilter, quest_model::QuestModel},
};

pub struct QuestViewingUsecase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    quest_viewing_repository: Arc<T>,
}

impl<T> QuestViewingUsecase<T>
where
    T: QuestViewingRepository + Send + Sync,
{
    pub fn new(quest_viewing_repository: Arc<T>) -> Self {
        QuestViewingUsecase {
            quest_viewing_repository,
        }
    }

    pub async fn view_details(&self, quest_id: i32) -> Result<QuestModel> {
        let res = self.quest_viewing_repository.view_details(quest_id).await?;

        let adventurers_count = self
            .quest_viewing_repository
            .adventurer_counting(quest_id)
            .await?;

        let quest_model = res.to_model(adventurers_count);
        Ok(quest_model)
    }

    pub async fn board_checking(&self, filter: &BoardCheckingFilter) -> Result<Vec<QuestModel>> {
        let res = self.quest_viewing_repository.board_checking(filter).await?;

        let mut quests_models: Vec<QuestModel> = Vec::new();

        for quest in res.into_iter() {
            let adventurers_count = self
                .quest_viewing_repository
                .adventurer_counting(quest.id)
                .await?;

            quests_models.push(quest.to_model(adventurers_count));
        }

        Ok(quests_models)
    }
}
