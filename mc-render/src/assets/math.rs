use std::collections::btree_map::BTreeMap as Map;
use std::collections::btree_map::Values;
use std::collections::btree_map::ValuesMut;

use serde::Deserialize;
use serde::Serialize;
use serde::de;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::ser::Serializer;
use serde::ser::SerializeStruct;



/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum Axis {
    #[serde(rename = "x")]
    X = 0,
    #[serde(rename = "y")]
    Y = 1,
    #[serde(rename = "z")]
    Z = 2
}


/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
#[derive(PartialEq, PartialOrd, Eq, Ord)]
pub enum Face {

    #[serde(rename = "down")]
    Down, 

    #[serde(rename = "up")]
    Up, 

    #[serde(rename = "north")]
    North, 

    #[serde(rename = "south")]
    South, 

    #[serde(rename = "west")]
    West, 

    #[serde(rename = "east")]
    East,

}


impl Face {

    pub fn index(&self) -> usize {
        match self {
            Self::West => 0,
            Self::Down => 1,
            Self::North => 2,
            Self::South => 3,
            Self::Up => 4,
            Self::East => 5,
        }
    }

    pub fn opposite(&self) -> Face {
        match self {
            Self::West => Self::East,
            Self::Down => Self::Up,
            Self::North => Self::South,
            Self::South => Self::North,
            Self::Up => Self::Down,
            Self::East => Self::West,
        }
    }
}


/**
 * 
 */
#[derive(Clone, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Rotate90 {
    R0,
    R90,
    R180,
    R270,
}

impl Default for Rotate90 {
    
    fn default() -> Self {
        Rotate90::R0
    }
}

impl Rotate90 {

    pub fn index(&self) -> usize {
        match self {
            Self::R0 => 0,
            Self::R90 => 1,
            Self::R180 => 2,
            Self::R270 => 3,
        }
    }

    pub fn inverse(&self) -> Rotate90 {
        match self {
            Self::R0 => Self::R0,
            Self::R90 => Self::R270,
            Self::R180 => Self::R180,
            Self::R270 => Self::R90,
        }
    }
}

impl<'de> Deserialize<'de> for Rotate90 {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = Rotate90;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("number: 0, 90, 180, 270")
            }

            fn visit_u16<E>(self, value: u16) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Unsigned(value as u64), &self))
                }
            }

            fn visit_u32<E>(self, value: u32) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Unsigned(value as u64), &self))
                }
            }

            fn visit_u64<E>(self, value: u64) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Unsigned(value as u64), &self))
                }
            }

            fn visit_i16<E>(self, value: i16) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Signed(value as i64), &self))
                }
            }

            fn visit_i32<E>(self, value: i32) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Signed(value as i64), &self))
                }
            }

            fn visit_i64<E>(self, value: i64) -> Result<Rotate90, E>
            where
                E: de::Error,
            {
                match value {
                    0 => Ok(Rotate90::R0),
                    90 => Ok(Rotate90::R90),
                    180 => Ok(Rotate90::R180),
                    270 => Ok(Rotate90::R270),
                    _ => Err(de::Error::invalid_type(de::Unexpected::Signed(value as i64), &self))
                }
            }
        }
        deserializer.deserialize_i32(InnerVisitor)
    }
}

impl Serialize for Rotate90 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i32(self.clone() as i32)
    }
}

/**
 * 
 */
#[derive(Clone, Debug)]
pub struct Expression<K, V, M> {

    keys: Map<K, M>,

    values: Map<M, V>,

    mask: M,
}

impl<K: std::cmp::Ord, V> Default for Expression<K, V, usize> {

    fn default() -> Self {
        Expression {
            keys: Map::default(),
            values: Map::default(),
            mask: 0,
        }
    }
}

impl<K: std::cmp::Ord + Clone, V> Expression<K, V, usize> {

    pub fn insert<'a, I:Iterator<Item=&'a K>>(&'a mut self, key: I, value: V) -> Option<usize> {
        let mut m = self.get_initial_link();
        for k in key {
            self.insert_key(k, &mut m)?;
        }
        self.values.insert(m, value);
        Some(m)
    }

    pub fn insert_key(&mut self, k: &K, m: &mut usize) -> Option<usize> {
        if let Some(n) = self.keys.get(k) {
            *m |= *n;
            Some(*n)
        } else {
            if self.mask == std::usize::MAX {
                return None;
            }
            let n = self.mask + 1;
            self.mask = n | self.mask;
            self.keys.insert(k.clone(), n);
            *m |= n;
            Some(n)
        }
    }

    pub fn insert_value_unchecked(&mut self, m: usize, v: V) -> Option<V> {
        self.values.insert(m, v)
    }
}

impl<K: std::cmp::Ord, V> Expression<K, V, usize> {

    pub fn get_initial_link(&self) -> usize {
        0
    }

    pub fn size(&self) -> (usize, usize) {
        (self.keys.len(), self.values.len())
    }

    pub fn get<'a, I:Iterator<Item=&'a K>>(&'a self, key: I, strict: bool) -> Option<&'a V> {
        if !strict && self.keys.len() == 0 {
            return self.values.get(&0);
        }
        let mut m = 0;
        let keys = &self.keys;
        for k in key {
            match keys.get(k) {
                Some(n) => {
                    m |= n;
                },
                None => {
                    if strict { return None; }
                }
            }
        }
        self.values.get(&m)
    }

    pub fn get_mut<'a, I:Iterator<Item=&'a K>>(&'a mut self, key: I, strict: bool) -> Option<&'a mut V> {
        if !strict && self.keys.len() == 0 {
            return self.values.get_mut(&0);
        }
        let mut m = 0;
        let keys = &self.keys;
        for k in key {
            match keys.get(k) {
                Some(n) => {
                    m |= n;
                },
                None => {
                    if strict { return None; }
                }
            }
        }
        self.values.get_mut(&m)
    }

    pub fn all<'a>(&'a self) -> Values<'a, usize, V> {
        self.values.values()
    }

    pub fn all_mut<'a>(&'a mut self) -> ValuesMut<'a, usize, V> {
        self.values.values_mut()
    }

    pub fn transf_into<K2, V2, FK, FV, E>(self, mut fk: FK, mut fv: FV) -> Result<Expression<K2, V2, usize>, E> 
    where
        K2: std::cmp::Ord,
        FK: FnMut(K)->Result<K2, E>, 
        FV: FnMut(V)->Result<V2, E>,
    {
        let mut keys = Map::new();
        for (k, m) in self.keys {
            let k2 = fk(k)?;
            keys.insert(k2, m);
        }
        let mut values = Map::new();
        for (m, v) in self.values {
            let v2 = fv(v)?;
            values.insert(m, v2);
        }
        Ok(Expression {
            keys,
            values,
            mask: self.mask
        })
    }
}

impl<K: std::cmp::Ord, V: Default> Expression<K, V, usize> {

    pub fn get_unchecked_default(&mut self, m: usize) -> &mut V {
        self.values.entry(m).or_insert_with(V::default)
    }
}

impl<K, V, M> Serialize for Expression<K, V, M> 
where 
    K: Serialize + std::cmp::Ord,
    V: Serialize,
    M: Serialize + std::cmp::Ord,
{ 

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    { 
        let mut s = serializer.serialize_struct("Expression", 3)?;
        s.serialize_field("keys", &self.keys)?;
        s.serialize_field("values", &self.values)?;
        s.serialize_field("size", &(self.keys.len(), self.values.len()))?;
        s.end()
    }

}