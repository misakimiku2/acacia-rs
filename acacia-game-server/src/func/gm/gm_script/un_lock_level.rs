use std::sync::Arc;

use crate::base::conn::Conn;

#[derive(Debug)]
pub struct UnLockLevel {
    pub chapter: i32,
    pub diff: i32,
    pub chapter_id: i32,
    pub no_star: i32,
}

impl UnLockLevel {
    pub fn from_args(args: &[String]) -> Self {
        Self {
            chapter: args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0),
            diff: args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0),
            chapter_id: args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(0),
            no_star: args.get(0).and_then(|s| s.parse::<i32>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}
