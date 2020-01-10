/**
 * 
 */



/**
 * 
 */
pub struct Scanner {

    parts: Vec<&'static str>,
}

impl Scanner {

    pub fn new(s: &'static str) -> Self {
        Scanner {
            parts: s.split("{}").collect()
        }
    }

    pub fn argc(&self) -> usize {
        self.parts.len() - 1
    }

    pub fn scan<'a, 'b>(&self, s: &'a str, c: &'b mut[&'a str]) -> usize {
        let filter = self.parts.as_slice();
        let mut n = 0;
        let mut p = s;
        let mut q = filter[n];
        if !p.starts_with(q) {
            return 0;
        }
        n += 1;
        while n < filter.len() {
            p = p.split_at(q.len()).1;
            q = filter[n];
            let i = if q == "" {
                p.len()
            } else {
                match p.find(q) {
                    Some(v) => v,
                    None => return 0
                }
            };
            let r = p.split_at(i);
            c[n - 1] = r.0;
            p = r.1;
            n += 1;
        }
        n - 1
    }
}


use serde::de::{self, Deserialize, Deserializer, Visitor};

#[derive(Debug)]
pub enum Lazy<T> {
    Name(String),
    Real(T),
}

impl<'de, T> Deserialize<'de> for Lazy<T> {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;
        use std::marker;

        struct InnerVisitor<T> {
            _marker: marker::PhantomData<T>
        }

        impl<'de, T> Visitor<'de> for InnerVisitor<T> {
            type Value = Lazy<T>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("str of name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Lazy::Name(value.to_owned()))
            }
        }

        deserializer.deserialize_str(InnerVisitor{_marker: marker::PhantomData})
    }
}


/**
 * 
 */
use serde_json::Value;

pub fn merge(a: &mut Value, b: Value) {
    match (a, b) {
        (a @ &mut Value::Object(_), Value::Object(b)) => {
            let a = a.as_object_mut().unwrap();
            for (k, v) in b {
                merge(a.entry(k).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b,
    }
}


/**
 * 
 */

pub trait Provider {
    type Item;

    fn provide(&mut self, name: &str) -> Option<Self::Item>;

}



/**
 * 
 */

#[derive(Debug)]
pub struct ParseError {

    msg: String,

    value: Value,

    source: Option<Box<dyn std::error::Error + 'static>>,
}

impl std::fmt::Display for ParseError {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>)-> std::fmt::Result {
        f.write_str(&self.msg)?;
        if let Some(cause) = &self.source {
            f.write_fmt(format_args!("\ncaused by: {}", cause))?;
        }
        Ok(())
    }
}

impl std::error::Error for ParseError {

    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.source {
            Some(b) => Some(b.as_ref()),
            None => None,
        }
    }
}

impl ParseError {

    pub fn new(msg: String, value: Value) -> Self {
        ParseError {
            msg,
            value,
            source: None
        }
    }

    pub fn new_with<E: std::error::Error + 'static>(msg: String, value: Value, source: E) -> Self {
        ParseError {
            msg,
            value,
            source: Some(Box::new(source))
        }
    }
}