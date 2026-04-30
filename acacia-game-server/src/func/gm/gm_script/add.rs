use acacia_data::tables::tables;
use std::{collections::HashMap, sync::Arc};
use tracing::warn;

use crate::base::conn::{self, Conn};
use acacia_commons::{bits::set_template, lua_utils::lua_func};
use acacia_game_msg::{core, protocol, protocol::Pf};

#[derive(Debug)]
pub enum Add {
    AddExp(AddExp),
    AddMoney(AddMoney),
    AddVigor(AddVigor),
    AddItem(AddItem),
}

impl Add {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let sub_fn = args.get(0).map(String::as_str);
        let sub_args = if let Some(sub_args_str) = args.get(1) {
            lua_func::parse_function_args(sub_args_str.trim_start_matches('{').trim_end_matches('}'))
        } else {
            Vec::new()
        };

        let add_action = match sub_fn {
            Some("GM.AddExp") => Self::AddExp(AddExp::from_args(&sub_args)),
            Some("GM.AddMoney") => Self::AddMoney(AddMoney::from_args(&sub_args)),
            Some("GM.AddItem") => Self::AddItem(AddItem::from_args(&sub_args)),
            Some("GM.AddVigor") => Self::AddVigor(AddVigor::from_args(&sub_args)),
            _ => {
                warn!("GM.Add Unknown Code: {:?} {:?}", sub_fn, sub_args);
                return None;
            }
        };
        Some(add_action)
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        match self {
            Self::AddExp(s) => s.run(conn).await,
            Self::AddMoney(s) => s.run(conn).await,
            Self::AddVigor(s) => s.run(conn).await,
            Self::AddItem(s) => s.run(conn).await,
        }
    }
}

#[derive(Debug)]
pub struct AddExp {
    pub count: u32,
}

impl AddExp {
    fn from_args(args: &[String]) -> Self {
        Self {
            count: args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        me.add_exp(self.count).await?;
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AddMoney {
    pub typ: u32,
    pub count: u32,
}

impl AddMoney {
    fn from_args(args: &[String]) -> Self {
        Self {
            typ: args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
            count: args.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let database = conn.database().await;

        let mut attr = database.attrs.get_attr(1, &self.typ * 2 + 1).await?;
        attr.1 += &self.count;
        database.attrs.update_one(attr).await?;

        let rsp = protocol::NtfSyncPlayer {
            custom: HashMap::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct AddVigor {
    pub count: i64,
}

impl AddVigor {
    fn from_args(args: &[String]) -> Self {
        Self {
            count: args.get(0).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
        }
    }

    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct AddItem {
    pub g: u16,
    pub d: u16,
    pub p: u16,
    pub l: u16,
    pub count: u32,
}

impl AddItem {
    fn from_args(args: &[String]) -> Self {
        Self {
            g: args.get(0).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
            d: args.get(1).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
            p: args.get(2).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
            l: args.get(3).and_then(|s| s.parse::<u16>().ok()).unwrap_or(0),
            count: args.get(4).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let mut me = conn.manager().await;
        if let Some(_weapon_template_data) = tables().suplies_template.find_one("gdpl", [self.g, self.d, self.p, self.l]) {
            let new_item = core::Item {
                id: me.new_item_id().await?,
                template: set_template(self.g, self.d, self.p, self.l)?,
                count: 1,
                enhance: Some(core::Enhance { level: 1, ..Default::default() }),
                ..Default::default()
            };
            me.set_item(new_item).await?;
        } else {
            me.add_item_count(self.g, self.d, self.p, self.l, self.count).await?;
        }
        conn.send_msg(Pf::NtfSyncattr, &me.as_sync_player(), 0).await?;
        Ok(())
    }
}
