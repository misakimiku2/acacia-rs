use super::player_attr::PlayerAttrManager;
use super::player_item::PlayerItemManager;
use super::player_solution::PlayerSolutionManager;
use super::player_str_attr::PlayerStrAttrManager;

pub struct PlayerManager {
    pub pid: u64,
    pub attrs: PlayerAttrManager,
    pub items: PlayerItemManager,
    pub solutions: PlayerSolutionManager,
    pub str_attrs: PlayerStrAttrManager,
}

impl PlayerManager {
    pub fn new(pid: u64) -> Self {
        Self {
            attrs: PlayerAttrManager::new(pid),
            items: PlayerItemManager::new(pid),
            solutions: PlayerSolutionManager::new(pid),
            str_attrs: PlayerStrAttrManager::new(pid),
            pid,
        }
    }
}
