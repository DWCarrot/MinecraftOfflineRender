use std::collections::btree_map::BTreeMap as Map;
use std::collections::btree_map::Values;
use std::collections::btree_map::ValuesMut;


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


/**
 * 
 */

#[derive(Clone, Debug)]
pub struct BlockState<K, M> {

    expr: Expression<K, M, usize>,

    group: Vec<usize>

}

impl<K: std::cmp::Ord, M> BlockState<K, M> {

    pub fn new() -> Self {
        BlockState {
            expr: Expression::default(),
            group: Vec::new(),
        }
    }
}

impl<K: std::cmp::Ord, M: Clone> BlockState<K, M> {

    pub fn get<'a, I: Iterator<Item = &'a K>>(&'a self, key: I) -> Vec<M> {
        let mut m = 0;
        for k in key {
            if let Some(n) = self.expr.keys.get(k) {
                m |= n;
            }
        }
        let mut parts = Vec::with_capacity(self.group.len());
        for mask in self.group.iter() {
            let n = *mask & m;
            if let Some(model) = self.expr.values.get(&n) {
                parts.push(model.clone())
            }
        }
        parts
    }
} 

impl<K: std::cmp::Ord + Clone, M> BlockState<K, M> {

    pub fn start_group(&mut self) {
        self.group.push(self.expr.get_initial_link());
    }

    pub fn insert_group<'a, I:Iterator<Item=&'a K>>(&'a mut self, key: I, value: M) -> Option<usize> {
        let mut m = self.expr.get_initial_link();
        for k in key {
            self.expr.insert_key(k, &mut m)?;
        }
        let mask = self.group.last_mut()?;
        self.expr.values.insert(m, value);
        *mask |= m;
        Some(m)
    }
}


// pub struct MultiPartModelIter<'a, M> {

//     parts: Vec<SliceIter<'a, M>>,

//     index: usize,

// }

// impl<'a, M: Clone> Iterator for MultiPartModelIter<'a, M> {
//     type Item = M;

//     fn next(&mut self) -> Option<Self::Item> {
//         while self.index < self.parts.len() {
//             let it = &mut self.parts[self.index];
//             if let Some(item) = it.next() {
//                 return Some(item.clone())
//             }
//             self.index += 1;
//         }
//         None
//     }
// }