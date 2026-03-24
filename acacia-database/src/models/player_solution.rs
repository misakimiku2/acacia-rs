use acacia_game_msg::core;
use surrealdb_types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct PlayerLineup {
    pub index: u32,
    pub name: String,
    pub member1: u64,
    pub member2: u64,
    pub member3: u64,
}

impl PlayerLineup {
    pub fn to_prost(self) -> core::Lineup {
        core::Lineup {
            index: self.index,
            name: self.name,
            member1: self.member1,
            member2: self.member2,
            member3: self.member3,
        }
    }

    pub fn from_prost(src: core::Lineup) -> Self {
        Self {
            index: src.index,
            name: src.name,
            member1: src.member1,
            member2: src.member2,
            member3: src.member3,
        }
    }
}
