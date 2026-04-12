use serde::Deserialize;
use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};

use super::lua_table;

pub fn from_str<'de, T: Deserialize<'de>>(s: &str) -> Result<T, String> {
    let value = lua_table::parse(s)?;
    T::deserialize(value).map_err(|e| e.to_string())
}

#[derive(Debug)]
pub struct DeError(String);

impl std::fmt::Display for DeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}
impl std::error::Error for DeError {}
impl de::Error for DeError {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        DeError(msg.to_string())
    }
}

impl<'de> de::Deserializer<'de> for lua_table::Value {
    type Error = DeError;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Self::Error> {
        match self {
            lua_table::Value::Bool(b) => visitor.visit_bool(b),
            lua_table::Value::Int(n) => visitor.visit_i64(n),
            lua_table::Value::Float(n) => visitor.visit_f64(n),
            lua_table::Value::Str(s) => visitor.visit_string(s),
            lua_table::Value::Nil => visitor.visit_unit(),
            lua_table::Value::SeqTable(v) => {
                let _len = v.len();
                let mut access = LuaSeqAccess(v.into_iter());
                let result = visitor.visit_seq(&mut access)?;
                Ok(result)
            }
            lua_table::Value::Table(m) => {
                let mut access = LuaDictAccess {
                    iter: m.into_iter(),
                    pending_value: None,
                };
                visitor.visit_map(&mut access)
            }
        }
    }

    serde::forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64
        char str string bytes byte_buf option unit unit_struct
        newtype_struct seq tuple tuple_struct map struct
        enum identifier ignored_any
    }
}

struct LuaSeqAccess(std::vec::IntoIter<lua_table::Value>);

impl<'de> SeqAccess<'de> for LuaSeqAccess {
    type Error = DeError;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>, Self::Error> {
        match self.0.next() {
            Some(val) => seed.deserialize(val).map(Some),
            None => Ok(None),
        }
    }
}

struct LuaDictAccess {
    iter: std::collections::hash_map::IntoIter<lua_table::Key, lua_table::Value>,
    pending_value: Option<lua_table::Value>,
}

impl<'de> MapAccess<'de> for LuaDictAccess {
    type Error = DeError;

    fn next_key_seed<K: DeserializeSeed<'de>>(&mut self, seed: K) -> Result<Option<K::Value>, Self::Error> {
        match self.iter.next() {
            Some((key, value)) => {
                self.pending_value = Some(value);
                let key_value = match key {
                    lua_table::Key::Int(n) => lua_table::Value::Int(n),
                    lua_table::Key::Str(s) => lua_table::Value::Str(s),
                };
                seed.deserialize(key_value).map(Some)
            }
            None => Ok(None),
        }
    }

    fn next_value_seed<V: DeserializeSeed<'de>>(&mut self, seed: V) -> Result<V::Value, Self::Error> {
        let val = self.pending_value.take().expect("value must follow key");
        seed.deserialize(val)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug, Deserialize)]
    struct Item {
        id: u32,
        name: String,
    }

    #[test]
    fn test() {
        let v: Vec<u32> = from_str("{8,1,2,3}").unwrap();
        println!("{v:?}");
        let v: Vec<Vec<u32>> = from_str("{{5,1,1,1,2}}").unwrap();
        println!("{v:?}");
    }
}
