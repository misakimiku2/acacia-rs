use acacia_game_msg::core;
use std::collections::HashMap;
use surrealdb_types::{Error, Object, SurrealValue, Value};

#[derive(Debug, Clone, Default)]
pub struct PlayerItemSlots(pub HashMap<u32, u64>);

impl SurrealValue for PlayerItemSlots {
    fn kind_of() -> surrealdb_types::Kind {
        surrealdb_types::kind!(object)
    }

    fn into_value(self) -> Value {
        let mut obj = Object::new();
        for (k, v) in self.0 {
            obj.insert(k.to_string(), v);
        }
        Value::Object(obj)
    }

    fn from_value(value: Value) -> Result<Self, Error> {
        let Value::Object(obj) = value else {
            return Err(Error::thrown("slots should be an object".into()));
        };

        let mut out = HashMap::new();
        for (k, v) in obj {
            let key = k.parse::<u32>().map_err(|_| Error::thrown(format!("invalid slot key: {k}")))?;
            let val = u64::from_value(v)?;
            out.insert(key, val);
        }

        Ok(PlayerItemSlots(out))
    }
}

#[derive(Debug, SurrealValue)]
#[surreal(default)]
pub struct PlayerItemEnhance {
    pub level: u32,
    pub exp: u32,
    #[surreal(rename = "break")]
    pub r#break: u32,
    pub evolue: u32,
    pub trust: u32,
    pub pro_level: u32,
    pub spines: Vec<u64>,
    pub affixs: Vec<u32>,
}

impl Default for PlayerItemEnhance {
    fn default() -> Self {
        Self {
            level: 0,
            exp: 0,
            r#break: 0,
            evolue: 0,
            trust: 0,
            pro_level: 0,
            affixs: vec![],
            spines: vec![],
        }
    }
}

impl PlayerItemEnhance {
    pub fn to_prost(self) -> core::Enhance {
        core::Enhance {
            level: self.level,
            exp: self.exp,
            r#break: self.r#break,
            evolue: self.evolue,
            trust: self.trust,
            pro_level: self.pro_level,
            spines: self.spines,
            affixs: self.affixs,
        }
    }

    pub fn from_prost(src: core::Enhance) -> Self {
        Self {
            level: src.level,
            exp: src.exp,
            r#break: src.r#break,
            evolue: src.evolue,
            trust: src.trust,
            pro_level: src.pro_level,
            spines: src.spines,
            affixs: src.affixs,
        }
    }
}

#[derive(Debug, SurrealValue)]
pub struct PlayerItem {
    #[surreal(rename = "sid")]
    pub id: u64,
    pub template: u64,
    pub count: u32,
    pub flag: u32,
    pub userdata: u32,
    pub expiration: u32,
    pub enhance: Option<PlayerItemEnhance>,
    pub slots: PlayerItemSlots,
}

impl PlayerItem {
    pub fn to_prost(self) -> core::Item {
        core::Item {
            id: self.id,
            template: self.template,
            count: self.count,
            flag: self.flag,
            userdata: self.userdata,
            expiration: self.expiration,
            enhance: self.enhance.map(PlayerItemEnhance::to_prost),
            slots: self.slots.0,
        }
    }

    pub fn from_prost(src: core::Item) -> Self {
        Self {
            id: src.id,
            template: src.template,
            count: src.count,
            flag: src.flag,
            userdata: src.userdata,
            expiration: src.expiration,
            enhance: src.enhance.map(PlayerItemEnhance::from_prost),
            slots: PlayerItemSlots(src.slots),
        }
    }
}
