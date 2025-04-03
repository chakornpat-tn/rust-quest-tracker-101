use anyhow::Result;
use std::fmt;

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Stage {
    Local,
    #[default]
    Dev,
    Prod,
}
impl fmt::Display for Stage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let stage = match self {
            Stage::Local => "local",
            Stage::Dev => "dev",
            Stage::Prod => "prod",
        };
        write!(f, "{}", stage)
    }
}


impl Stage {
    pub fn try_from(stage: &str) -> Result<Self> {
        match stage {
            "local" => Ok(Stage::Local),
            "dev" => Ok(Stage::Dev),
            "prod" => Ok(Stage::Prod),
            _ => Err(anyhow::anyhow!("Invalid stage: {}", stage)),
        }
    }
}