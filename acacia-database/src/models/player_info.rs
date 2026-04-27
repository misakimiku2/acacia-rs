use acacia_game_msg::core;
use surrealdb_types::SurrealValue;

#[derive(Debug, SurrealValue)]
pub struct PlayerInfo {
    pub pid: u64,
    pub account: String,
    pub name: String,
    pub sign: String,
    pub level: u32,
    pub exp: u32,
    pub vigor: u32,
    pub charged: u32,
    pub create_time: u32,
    pub last_login_time: u32,
    pub last_vigor_time: u32,
    pub item_id_alloc: u64,
    pub show_items: Vec<u64>,
    pub show_attrs: Vec<u32>,
    pub last_pieces: u64,
    pub last_global_mail_time: u32,
    pub last_person_mid: u64,
    pub badges: Vec<u64>,
    pub serial: u64,
}

impl PlayerInfo {
    pub fn to_prost(self) -> core::Player {
        core::Player {
            pid: self.pid,
            account: self.account,
            name: self.name,
            sign: self.sign,
            level: self.level,
            exp: self.exp,
            vigor: self.vigor,
            charged: self.charged,
            create_time: self.create_time,
            last_login_time: self.last_login_time,
            last_vigor_time: self.last_vigor_time,
            item_id_alloc: self.item_id_alloc,
            show_items: self.show_items,
            show_attrs: self.show_attrs,
            last_pieces: self.last_pieces,
            last_global_mail_time: self.last_global_mail_time,
            last_person_mid: self.last_person_mid,
            badges: self.badges,
            serial: self.serial,
            ..Default::default()
        }
    }

    pub fn from_prost(src: core::Player) -> Self {
        Self {
            pid: src.pid,
            account: src.account,
            name: src.name,
            sign: src.sign,
            level: src.level,
            exp: src.exp,
            vigor: src.vigor,
            charged: src.charged,
            create_time: src.create_time,
            last_login_time: src.last_login_time,
            last_vigor_time: src.last_vigor_time,
            item_id_alloc: src.item_id_alloc,
            show_items: src.show_items,
            show_attrs: src.show_attrs,
            last_pieces: src.last_pieces,
            last_global_mail_time: src.last_global_mail_time,
            last_person_mid: src.last_person_mid,
            badges: src.badges,
            serial: src.serial,
        }
    }
}
