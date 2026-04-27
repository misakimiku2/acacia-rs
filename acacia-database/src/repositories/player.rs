use super::player_attr::PlayerAttrRepository;
use super::player_info::PlayerInfoRepository;
use super::player_item::PlayerItemRepository;
use super::player_solution::PlayerSolutionRepository;
use super::player_str_attr::PlayerStrAttrRepository;

pub struct PlayerRepository {
    pub pid: u64,
    pub info: PlayerInfoRepository,
    pub attrs: PlayerAttrRepository,
    pub items: PlayerItemRepository,
    pub solutions: PlayerSolutionRepository,
    pub str_attrs: PlayerStrAttrRepository,
}

impl PlayerRepository {
    pub fn new(pid: u64) -> Self {
        Self {
            info: PlayerInfoRepository::new(pid),
            attrs: PlayerAttrRepository::new(pid),
            items: PlayerItemRepository::new(pid),
            solutions: PlayerSolutionRepository::new(pid),
            str_attrs: PlayerStrAttrRepository::new(pid),
            pid,
        }
    }
}
