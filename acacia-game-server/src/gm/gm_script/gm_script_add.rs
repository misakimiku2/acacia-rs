use std::sync::Arc;
use tracing::warn;

use crate::base::conn::Conn;
use acacia_commons::lua_parser;

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
        let sub_args = lua_parser::parse_function_args(args.get(1).map(String::as_str).unwrap_or(""));

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
    pub value: i64,
    pub count: i64,
}

impl GMScriptAddMoney {
    fn from_args(args: &[String]) -> Self {
        Self {
            value: args.get(0).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
            count: args.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
        }
    }

    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
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
    pub g: i64,
    pub d: i64,
    pub p: i64,
    pub l: i64,
    pub count: i64,
}

impl GMScriptAddItem {
    fn from_args(args: &[String]) -> Self {
        Self {
            g: args.get(0).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
            d: args.get(1).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
            p: args.get(2).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
            l: args.get(3).and_then(|s| s.parse::<i64>().ok()).unwrap_or(0),
            count: args.get(4).and_then(|s| s.parse::<i64>().ok()).unwrap_or(1),
        }
    }

    pub async fn run(&self, _: Arc<Conn>) -> anyhow::Result<()> {
        Ok(())
    }
}
