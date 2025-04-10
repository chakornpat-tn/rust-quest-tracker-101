use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum QuestStatus {
    #[default]
    Open,
    InJourney,
    Completed,
    Failed,
}

impl fmt::Display for QuestStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            QuestStatus::Open => "Open",
            QuestStatus::InJourney => "InJourney",
            QuestStatus::Completed => "Completed",
            QuestStatus::Failed => "Failed",
        };
        write!(f, "{}", stage)
    }
}
