use super::ModeTrait;

pub struct IdleMode {}

impl IdleMode {
    pub fn create() -> anyhow::Result<IdleMode> {
        Ok(IdleMode {})
    }
}

impl ModeTrait for IdleMode {}
