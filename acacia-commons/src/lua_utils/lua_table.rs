use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    Int(i64),
    Str(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
    Nil,
    SeqTable(Vec<Value>),
    Table(HashMap<Key, Value>),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Bool(b) => write!(f, "{b}"),
            Value::Int(n) => write!(f, "{n}"),
            Value::Float(n) => write!(f, "{n}"),
            Value::Str(s) => write!(f, "\"{s}\""),
            Value::Nil => write!(f, "nil"),
            Value::SeqTable(v) => {
                write!(f, "[")?;
                for (i, item) in v.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Value::Table(m) => {
                write!(f, "{{")?;
                for (i, (k, v)) in m.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    match k {
                        Key::Int(n) => write!(f, "[{n}]={v}")?,
                        Key::Str(s) => write!(f, "{s}={v}")?,
                    }
                }
                write!(f, "}}")
            }
        }
    }
}

struct Parser<'a> {
    input: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self { input: input.as_bytes(), pos: 0 }
    }

    fn peek(&self) -> Option<u8> {
        self.input.get(self.pos).copied()
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn expect(&mut self, ch: u8) -> Result<(), String> {
        if self.peek() == Some(ch) {
            self.advance();
            Ok(())
        } else {
            Err(format!("expected '{}' at pos {}, got {:?}", ch as char, self.pos, self.peek().map(|c| c as char)))
        }
    }

    fn skip_spaces(&mut self) {
        while self.peek() == Some(b' ') {
            self.advance();
        }
    }

    fn parse_value(&mut self) -> Result<Value, String> {
        self.skip_spaces();
        match self.peek() {
            Some(b'"') => self.parse_string(),
            Some(b'{') => self.parse_table(),
            Some(b't') => self.parse_true(),
            Some(b'f') => self.parse_false(),
            Some(b'n') => self.parse_nil(),
            Some(c) if c == b'-' || c.is_ascii_digit() => self.parse_number(),
            other => Err(format!("unexpected char {:?} at pos {}", other.map(|c| c as char), self.pos)),
        }
    }

    fn parse_true(&mut self) -> Result<Value, String> {
        self.expect_str("true")?;
        Ok(Value::Bool(true))
    }

    fn parse_false(&mut self) -> Result<Value, String> {
        self.expect_str("false")?;
        Ok(Value::Bool(false))
    }

    fn parse_nil(&mut self) -> Result<Value, String> {
        self.expect_str("nil")?;
        Ok(Value::Nil)
    }

    fn expect_str(&mut self, s: &str) -> Result<(), String> {
        for b in s.bytes() {
            self.expect(b)?;
        }
        Ok(())
    }

    fn parse_string(&mut self) -> Result<Value, String> {
        self.expect(b'"')?;
        let start = self.pos;
        while self.peek() != Some(b'"') {
            if self.peek().is_none() {
                return Err("unterminated string".into());
            }
            self.advance();
        }
        let s = std::str::from_utf8(&self.input[start..self.pos]).map_err(|e| e.to_string())?.to_owned();
        self.expect(b'"')?;
        Ok(Value::Str(s))
    }

    fn parse_number(&mut self) -> Result<Value, String> {
        let start = self.pos;
        if self.peek() == Some(b'-') {
            self.advance();
        }
        let mut is_float = false;
        while let Some(c) = self.peek() {
            if c == b'.' {
                is_float = true;
                self.advance();
            } else if c.is_ascii_digit() {
                self.advance();
            } else if c == b'e' || c == b'E' {
                is_float = true;
                self.advance();
                // optional +/-
                if matches!(self.peek(), Some(b'+') | Some(b'-')) {
                    self.advance();
                }
            } else {
                break;
            }
        }
        let token = std::str::from_utf8(&self.input[start..self.pos]).map_err(|e| e.to_string())?;
        if is_float {
            let v: f64 = token.parse().map_err(|e: std::num::ParseFloatError| e.to_string())?;
            Ok(Value::Float(v))
        } else {
            let v: i64 = token.parse().map_err(|e: std::num::ParseIntError| e.to_string())?;
            Ok(Value::Int(v))
        }
    }

    fn parse_table(&mut self) -> Result<Value, String> {
        self.expect(b'{')?;

        // 空表
        if self.peek() == Some(b'}') {
            self.advance();
            return Ok(Value::SeqTable(vec![]));
        }

        if self.is_dict_entry() { self.parse_dict_inner() } else { self.parse_list_inner() }
    }

    fn is_dict_entry(&self) -> bool {
        if self.peek() == Some(b'[') {
            return true;
        }
        let mut i = self.pos;
        while i < self.input.len() {
            let c = self.input[i];
            if c.is_ascii_alphanumeric() || c == b'_' {
                i += 1;
            } else {
                return c == b'=';
            }
        }
        false
    }

    fn parse_list_inner(&mut self) -> Result<Value, String> {
        let mut items = Vec::new();
        loop {
            items.push(self.parse_value()?);
            if self.peek() == Some(b',') {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(b'}')?;
        Ok(Value::SeqTable(items))
    }

    fn parse_dict_inner(&mut self) -> Result<Value, String> {
        let mut map = HashMap::new();
        loop {
            let key = self.parse_dict_key()?;
            self.expect(b'=')?;
            let value = self.parse_value()?;
            map.insert(key, value);
            if self.peek() == Some(b',') {
                self.advance();
            } else {
                break;
            }
        }
        self.expect(b'}')?;
        Ok(Value::Table(map))
    }

    fn parse_dict_key(&mut self) -> Result<Key, String> {
        if self.peek() == Some(b'[') {
            self.advance();
            let val = self.parse_number()?;
            self.expect(b']')?;
            match val {
                Value::Int(n) => Ok(Key::Int(n)),
                _ => Err("expected int key inside []".into()),
            }
        } else {
            let start = self.pos;
            while let Some(c) = self.peek() {
                if c.is_ascii_alphanumeric() || c == b'_' {
                    self.advance();
                } else {
                    break;
                }
            }
            if self.pos == start {
                return Err(format!("expected dict key at pos {}", self.pos));
            }
            let s = std::str::from_utf8(&self.input[start..self.pos]).map_err(|e| e.to_string())?.to_owned();
            Ok(Key::Str(s))
        }
    }
}

pub fn parse(input: &str) -> Result<Value, String> {
    let mut p = Parser::new(input);
    p.skip_spaces();
    let val = p.parse_value()?;
    if p.pos != p.input.len() {
        return Err(format!("trailing chars at pos {}", p.pos));
    }
    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        println!("{:?}", parse("{8,1,5,3}"));
        println!("{:?}", parse("{{5,1,1,1,2}}"));
    }
}
