use std::{collections::HashMap, sync::Arc};
use tracing::warn;

use crate::base::conn::Conn;
use acacia_commons::{bits::set_template, lua_utils::lua_func};
use acacia_game_msg::{core, protocol, protocol::Pf};

#[derive(Debug)]
pub enum GMScriptAdd {
    AddExp(GMScriptAddExp),
    AddMoney(GMScriptAddMoney),
    AddVigor(GMScriptAddVigor),
    AddItem(GMScriptAddItem),
}

impl GMScriptAdd {
    pub fn from_args(args: &[String]) -> Option<Self> {
        let sub_fn = args.get(0).map(String::as_str);
        let sub_args = if let Some(sub_args_str) = args.get(1) {
            lua_func::parse_function_args(sub_args_str.trim_start_matches('{').trim_end_matches('}'))
        } else {
            Vec::new()
        };

        let add_action = match sub_fn {
            Some("GM.AddExp") => Self::AddExp(GMScriptAddExp::from_args(&sub_args)),
            Some("GM.AddMoney") => Self::AddMoney(GMScriptAddMoney::from_args(&sub_args)),
            Some("GM.AddItem") => Self::AddItem(GMScriptAddItem::from_args(&sub_args)),
            Some("GM.AddVigor") => Self::AddVigor(GMScriptAddVigor::from_args(&sub_args)),
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
pub struct GMScriptAddExp {
    pub count: i64,
}

impl GMScriptAddExp {
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
pub struct GMScriptAddMoney {
    pub typ: u32,
    pub count: u32,
}

impl GMScriptAddMoney {
    fn from_args(args: &[String]) -> Self {
        Self {
            typ: args.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0),
            count: args.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, conn: Arc<Conn>) -> anyhow::Result<()> {
        let manager = conn.manager().await;

        let mut attr = manager.attrs.get_attr(1, &self.typ * 2 + 1).await?;
        attr.1 += &self.count;
        manager.attrs.set(attr.0, attr.1).await?;

        let rsp = protocol::NtfSyncPlayer {
            custom: HashMap::from([attr]),
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct GMScriptAddVigor {
    pub count: i64,
}

impl GMScriptAddVigor {
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
pub struct GMScriptAddItem {
    pub g: u16,
    pub d: u16,
    pub p: u16,
    pub l: u16,
    pub count: u32,
}

impl GMScriptAddItem {
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
        let template_id = set_template(self.g, self.d, self.p, self.l)?;
        let manager = conn.manager().await;

        let item = if let Some(mut item) = manager.items.get_item_by_template(template_id).await? {
            item.count += &self.count;
            item
        } else {
            core::Item {
                id: manager.items.new_id().await?,
                template: template_id,
                count: self.count,
                ..core::Item::default()
            }
        };

        manager.items.set(item.clone()).await?;
        let rsp = protocol::NtfSyncPlayer {
            items: vec![item],
            ..protocol::NtfSyncPlayer::default()
        };
        conn.send_msg(Pf::NtfSyncattr, &rsp, 0).await?;
        Ok(())
    }
}
