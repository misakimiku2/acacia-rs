use std::sync::Arc;

use crate::base::conn::Conn;

#[derive(Debug)]
pub struct AddAllItem {}

impl AddAllItem {
    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}
