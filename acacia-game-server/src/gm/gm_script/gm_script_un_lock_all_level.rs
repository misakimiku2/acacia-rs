use std::sync::Arc;

use crate::base::conn::Conn;

#[derive(Debug)]
pub struct GMScriptUnLockAllLevel {
    pub no_star: i32,
}

impl GMScriptUnLockAllLevel {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            no_star: args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0),
        }
    }

    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}
