use std::collections::btree_map::BTreeMap as Map;
use rand;
use serde::Deserialize;
use serde::Serialize;
use serde::de;
use serde::de::Deserializer;
use serde::de::Visitor;
use serde::de::MapAccess;
use serde::de::SeqAccess;
use serde::ser::Serializer;
use serde::ser::SerializeSeq;
use serde::ser::SerializeStruct;
use super::primary_type::PrimaryType;
//use super::util::Provider;
use super::math::Rotate90;
use super::math::Face;
use super::math::Axis;
use super::math::Expression;


/**
 * 
 */
fn default_bool_true() -> bool { true }
fn default_f32_1() -> f32 { 1.0 }
fn default_3f32_0() -> [f32; 3] { [0.0f32, 0.0, 0.0] }
fn default_3f32_1() -> [f32; 3] { [1.0f32, 1.0, 1.0] }
fn default_3f32_8() -> [f32; 3] { [8.0f32, 8.0, 8.0] }
fn default_3f32_16() -> [f32;3] { [16.0, 16.0, 16.0] }

/**
 * 
 */

macro_rules! option_override {
    ($t:ident,$s:ident,$($field:ident),+) => {
        $(
            if $t.$field.is_none() && $s.$field.is_some() {
                $t.$field = $s.$field.clone();
            }
        )+
    };
}

//type TProvider<T> = dyn Provider<Item=T>;

pub trait Merge {

    fn merge(&mut self, other: &Self)  -> bool;
}


/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ModelRaw {

    #[serde(default)]
    pub parent: Option<String>,

    #[serde(default = "default_bool_true")]
    pub ambientocclusion: bool, //true

    #[serde(default)]
    pub display: Option<Box<DisplayRaw>>,

    #[serde(default)]
    pub elements: Vec<ElementRaw>,

    #[serde(default)]
    pub textures: Option<Map<String, String>>
}

impl Merge for ModelRaw {

    fn merge(&mut self, other: &Self)  -> bool {
        self.parent = other.parent.clone();
        if let Some(s) = &mut self.display {
            if let Some(o) = &other.display {
                let s = s.as_mut();
                let o = o.as_ref();
                s.merge(o);
            }
        } else {
            if other.display.is_some() {
                self.display = other.display.clone();
            }
        }
        if self.elements.len() == 0 && other.elements.len() > 0 {
            self.elements = other.elements.clone();
        }
        if let Some(s) = &mut self.textures {
            if let Some(o) = &other.textures {
                for (k, v) in o {
                    s.entry(k.clone()).or_insert_with(|| v.clone());
                }
            }
        } else {
            if other.display.is_some() {
                self.display = other.display.clone();
            }
        }
        true
    }
}

impl ModelRaw {

    // pub fn integrate(&mut self, model_provider: &mut TProvider<Self>) -> Result<&Self, i32> {
         
    //     while let Some(s) = &self.parent {
    //         let parent = model_provider.provide(s).ok_or_else(|| 53)?;
    //         self.merge(&parent);
    //     }
        
    //     if let Some(textures) = self.textures.as_mut() {
    //         for element in self.elements.as_mut_slice() {
    //             for(face, tex) in &mut element.faces {
    //                 // if tex.cullface.is_none() {
    //                 //     tex.cullface = Some(face.opposite())
    //                 // } // cullface = None has meanings.
    //                 let mut s = tex.texture.as_str();
    //                 if s.starts_with('#') {
    //                     loop {
    //                         s = &s[1..];
    //                         s = textures.get(s).ok_or_else(|| 54)?.as_str();
    //                         if !s.starts_with('#') {
    //                             break;
    //                         }
    //                     }
    //                     tex.texture = s.to_string();
    //                 }
    //             }
    //         }
    //         if self.elements.len() > 0 {
    //             self.textures = None;
    //         }
    //     }
    //     Ok(self)  
    // }
}

/**
 * 
 */
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct DisplayRaw {

    #[serde(default)]
    pub thirdperson_righthand: Option<Transform>, 
    
    #[serde(default)]
    pub thirdperson_lefthand: Option<Transform>, 
    
    #[serde(default)]
    pub firstperson_righthand: Option<Transform>, 
    
    #[serde(default)]
    pub firstperson_lefthand: Option<Transform>, 
    
    #[serde(default)]
    pub gui: Option<Transform>, 
    
    #[serde(default)]
    pub head: Option<Transform>, 
    
    #[serde(default)]
    pub ground: Option<Transform>, 
    
    #[serde(default)]
    pub fixed: Option<Transform>,
}

impl Merge for DisplayRaw {

    fn merge(&mut self, other: &Self)  -> bool {
        option_override!(self, other, thirdperson_righthand, thirdperson_lefthand, firstperson_lefthand, firstperson_righthand, gui, head, ground, fixed);
        true
    }
}


/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Transform {

    #[serde(default = "default_3f32_0")]
    pub rotation: [f32; 3], // [0, 0, 0]

    #[serde(default = "default_3f32_0")]
    pub translation: [f32; 3], // [0, 0, 0]

    #[serde(default = "default_3f32_1")]
    pub scale: [f32; 3], // [1, 1, 1]
}

impl Default for Transform {
    fn default() -> Self {
        Transform {
            rotation: default_3f32_0(),
            translation: default_3f32_0(),
            scale: default_3f32_1(),
        }
    }
}



/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ElementRaw {

    #[serde(default = "default_3f32_0")]
    pub from: [f32; 3],

    #[serde(default = "default_3f32_16")]
    pub to: [f32; 3],

    #[serde(default)]
    pub rotation: Option<Box<Rotation>>,

    #[serde(default = "default_bool_true")]
    pub shade: bool, // true

    pub faces: Map<Face, FaceTextureRaw>,
}



impl Merge for ElementRaw {

    fn merge(&mut self, other: &Self)  -> bool {
        use std::collections::btree_map::Entry;

        option_override!(self, other, rotation);
        for (k, v) in &other.faces {
            match self.faces.entry(k.clone()) {
                Entry::Occupied(entry) => {
                    entry.into_mut().merge(v);
                }
                Entry::Vacant(entry) => {
                    entry.insert(v.clone());
                }
            }
        }
        true
    }
}


/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Rotation {

    #[serde(default = "default_3f32_8")]
    pub origin: [f32; 3],   // [8, 8, 8]

    pub axis: Axis, // Y

    #[serde(default)]
    pub angle: f32, // 0

    #[serde(default)]
    pub rescale: bool,
}

impl Default for Rotation {
    fn default() -> Self {
        Rotation {
            origin: default_3f32_8(),
            axis: Axis::Y,
            angle: Default::default(),
            rescale: Default::default()
        }
    }
}



/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FaceTextureRaw {

    #[serde(default)]
    pub uv: Option<Box<[f32; 4]>>, // [0, 0, 16, 16]

    pub texture: String,

    #[serde(default)]
    pub cullface: Option<Face>, // Side

    #[serde(default)]
    pub rotation: Option<Rotate90>,  // 0

    #[serde(default)]
    pub tintindex: Option<usize>,
}

impl Merge for FaceTextureRaw {
    fn merge(&mut self, other: &Self)  -> bool {
        option_override!(self, other, uv, cullface, tintindex, rotation);
        true
    }
}


/**
 * 
 */
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppliedModelRaw {

    #[serde(default)]
    pub x: Rotate90,
    
    #[serde(default)]
    pub y: Rotate90,

    pub model: String,

    #[serde(default)]
    pub uvlock: bool,

    #[serde(default = "default_f32_1")]
    pub weight: f32, 
}


/**
 * 
 */
#[derive(Clone, Debug)]
pub enum ApplyRaw {
    Item(AppliedModelRaw),
    Array(Vec<AppliedModelRaw>),
}

impl Serialize for ApplyRaw {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ApplyRaw::Item(m) => serializer.serialize_newtype_struct("AppliedModelRaw", m),
            ApplyRaw::Array(l) => {
                let mut seq = serializer.serialize_seq(Some(l.len()))?;
                for element in l.as_slice() {
                    seq.serialize_element(element)?;
                }
                seq.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for ApplyRaw {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = ApplyRaw;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("{model, uvlock?, x?, y?, weight?}, [{...}]")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut x = None;
                let mut y = None;
                let mut model = None;
                let mut uvlock = None;
                let mut weight = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "x" => x = map.next_value()?,
                        "y" => y = map.next_value()?,
                        "model" => model = Some(map.next_value()?),
                        "uvlock" => uvlock = map.next_value()?,
                        "weight" => weight = map.next_value()?,
                        _ => { }
                    }
                }
                Ok(ApplyRaw::Item(AppliedModelRaw {
                    x: x.unwrap_or_default(),
                    y: y.unwrap_or_default(),
                    model: model.ok_or_else(||de::Error::missing_field("model"))?,
                    uvlock: uvlock.unwrap_or_default(),
                    weight: weight.unwrap_or_else(default_f32_1),
                }))
                
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut list: Vec<AppliedModelRaw> = Vec::with_capacity(seq.size_hint().unwrap_or(4));
                while let Some(v) = seq.next_element()? {
                    list.push(v);
                }
                Ok(ApplyRaw::Array(list))
            }

            fn visit_newtype_struct<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
            where
                D: Deserializer<'de>,
            {
                ApplyRaw::deserialize(deserializer)
            }
        }
        const FIELDS: &'static [&'static str] = &["x", "y", "uvlock", "model", "weight"];
        deserializer.deserialize_any(InnerVisitor)
    }
}

impl ApplyRaw {

    pub fn get(&self) -> &AppliedModelRaw {
        match self {
            Self::Item(p) => p,
            Self::Array(list) => {
                let x = rand::random::<f32>();
                let mut p = 0.0;
                for t in list {
                    p += t.weight;
                    if x < p {
                        return &t;
                    }
                }
                &list[list.len()-1]
            },
        }
    }

    pub fn get_fast(&self) -> &AppliedModelRaw {
        match self {
            Self::Item(p) => p,
            Self::Array(list) => &list[0],
        }
    }

}

/**
 * 
 */
#[derive(Clone, Debug)]
pub struct VariantsRaw(pub Expression<String, ApplyRaw, usize>);

impl<'de> Deserialize<'de> for VariantsRaw {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = VariantsRaw;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("{ [condition, ,]: ApplyRaw }")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut expr = Expression::default();
                while let Some(s) = map.next_key::<String>()? {
                    let key: &str = s.as_str();
                    let v = map.next_value()?;
                    if key == "" {
                        expr.insert_value_unchecked(0, v);
                    } else {
                        let mut m = expr.get_initial_link();
                        for part in key.split(',') {
                            expr.insert_key(&part.to_string(), &mut m).ok_or_else(|| de::Error::invalid_length(64, &self))?;
                        }
                        
                        if let Some(v) = expr.insert_value_unchecked(m, v) {
                            return Err(de::Error::duplicate_field("[condition]"));
                        }
                    }
                }
                Ok(VariantsRaw(expr))
            }
        }
        
        deserializer.deserialize_map(InnerVisitor)
    }
}



/**
 * 
 */
#[derive(Clone, Debug)]
pub struct MuitiPartRaw(pub Expression<String, Vec<ApplyRaw>, usize>);


impl<'de> Deserialize<'de> for MuitiPartRaw {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;
        use std::collections::btree_map::BTreeMap;

        enum Case {
            OR(Vec<BTreeMap<String, PrimaryType>>),
            AND(BTreeMap<String, PrimaryType>),
        }

        impl<'de> Deserialize<'de> for Case {

            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct InnerVisitorCase;

                impl<'de> Visitor<'de> for InnerVisitorCase {
                    type Value = Case;
        
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("{[k:string]:string} or OR:[{...}]")
                    }

                    fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
                    where
                        V: MapAccess<'de>,
                    {
                        let mut case = BTreeMap::new();
                        while let Some(key) = map.next_key()? {
                            if key == "OR" {
                                if case.len() > 0 {
                                    return Err(de::Error::duplicate_field("OR"));
                                }
                                let multi = map.next_value()?;
                                return Ok(Case::OR(multi));
                            }
                            let value = map.next_value()?;
                            case.insert(key, value);
                        }
                        Ok(Case::AND(case))
                    }
                }

                deserializer.deserialize_map(InnerVisitorCase)
            }
            
        }

        #[derive(Deserialize)]
        struct CaseTuple {
            when: Option<Case>,
            apply: ApplyRaw
        }

        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = MuitiPartRaw;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("[{when: {}, apply: <ApplyRaw>} ]")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: SeqAccess<'de>,
            {
                let mut expr: Expression<String, Vec<ApplyRaw>, usize> = Expression::default();
                let mut all: Vec<ApplyRaw> = Vec::new();

                let mut insert_case = |pair: BTreeMap<String, PrimaryType>, value: ApplyRaw | -> Result<usize, A::Error> {
                    let mut keys: Vec<usize> = Vec::new();
                    let mut tmp = expr.get_initial_link();
                    keys.push(tmp);
                    for (pk, pv) in pair {
                        let pv: String = pv.into();
                        if pv.contains('|') {
                            let len = keys.len();
                            let cp = keys.clone();
                            let mut start = 0 as usize;
                            for s in pv.split('|') {
                                let k = pk.clone() + "=" + s;
                                let n = expr.insert_key(&k, &mut tmp).ok_or_else(|| de::Error::invalid_length(64, &self))?;
                                if start + len > keys.len() {
                                    keys.extend_from_slice(cp.as_slice());
                                }
                                for m in &mut keys[start..start+len] { *m |= n; }
                                start += len; 
                            }
                        } else {
                            let k = pk + "=" + &pv;
                            let n = expr.insert_key(&k, &mut tmp).ok_or_else(|| de::Error::invalid_length(64, &self))?;
                            for m in keys.iter_mut() { *m |= n; }
                        }
                    }
                    let len = keys.len();
                    for m in keys.into_iter() {
                        expr.get_unchecked_default(m).push(value.clone());
                    }
                    Ok(len)
                };

                while let Some(t) = seq.next_element::<CaseTuple>()? {
                    if let Some(case) = t.when {
                        match case {
                            Case::AND(pair) => {
                                insert_case(pair, t.apply.clone())?;
                            }
                            Case::OR(list) => {
                                for pair in list.into_iter() {
                                    insert_case(pair, t.apply.clone())?;
                                }
                            }
                        }
                    } else {
                        all.push(t.apply);
                    }
                }
                if all.len() > 0 {
                    for v in expr.all_mut() {
                        v.extend_from_slice(all.as_slice());
                    }
                    expr.insert_value_unchecked(0, all);
                };
                Ok(MuitiPartRaw(expr))
            }
        }
        
        deserializer.deserialize_seq(InnerVisitor)
    }
}



/**
 * 
 */
#[derive(Clone, Debug)]
pub enum BlockStateRaw {
    Variants(VariantsRaw),
    MuitiPart(MuitiPartRaw),
}

impl Serialize for BlockStateRaw {

    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            BlockStateRaw::Variants(v) => {
                let mut s = serializer.serialize_struct("variant", 1)?;
                s.serialize_field("map", &v.0)?;
                s.end()
            }
            BlockStateRaw::MuitiPart(m) => {
                let mut s = serializer.serialize_struct("muiltipart", 1)?;
                s.serialize_field("map", &m.0)?;
                s.end()
            }
        }
    }
}

impl<'de> Deserialize<'de> for BlockStateRaw {

    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        use std::fmt;

        struct InnerVisitor;

        impl<'de> Visitor<'de> for InnerVisitor {
            type Value = BlockStateRaw;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(" multipart: {...} or variants: {...}")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
            where
                V: MapAccess<'de>,
            {
                if let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        "variants" => Ok(Self::Value::Variants(map.next_value()?)),
                        "multipart" => Ok(Self::Value::MuitiPart(map.next_value()?)),
                        _ => Err(de::Error::unknown_field(key.as_str(), FIELDS))
                    }
                } else {
                    Err(de::Error::missing_field(""))
                }
            }
        }

        const FIELDS: &'static [&'static str] = &["variants", "multipart"];
        deserializer.deserialize_map(InnerVisitor)
    }
}



pub fn transform(axis: Axis, rotation: Rotate90, vec3: &[f32; 3]) -> [f32; 3] {
    match rotation {
        Rotate90::R0 => {
            [vec3[0], vec3[1], vec3[2]]
        }
        Rotate90::R90 => {
            match axis {
                Axis::X => [vec3[0]         , 16.0 - vec3[2]    , vec3[1]           ],
                Axis::Y => [vec3[2]         , vec3[1]           , 16.0 - vec3[0]    ],
                Axis::Z => [16.0 - vec3[1]  , vec3[0]           , vec3[2]           ],
            } 
        }
        Rotate90::R180 => {
            match axis {
                Axis::X => [vec3[0]         , 16.0 - vec3[1]    , 16.0 - vec3[2]    ],
                Axis::Y => [16.0 - vec3[0]  , vec3[1]           , 16.0 - vec3[2]    ],
                Axis::Z => [16.0 - vec3[0]  , 16.0 - vec3[1]    , vec3[2]           ],
            } 
        }
        Rotate90::R270 => {
            match axis {
                Axis::X => [vec3[0]         , vec3[2]           , 16.0 - vec3[1]    ],
                Axis::Y => [16.0 - vec3[2]  , vec3[1]           , vec3[0]           ],
                Axis::Z => [vec3[1]         , 16.0 - vec3[0]    , vec3[2]           ],
            } 
        }
    }
}





// impl<'de, K, V, M> Deserialize<'de> for Expression<K, V, M> 
// where 
//     K: Deserialize<'de> + std::cmp::Ord,
//     V: Deserialize<'de>,
//     M: Deserialize<'de> + std::cmp::Ord,
// {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         use std::fmt;
//         use std::marker::PhantomData;

//         struct InnerVisitor<K, V, M>(PhantomData<K>, PhantomData<V>, PhantomData<M>);

//         impl<'de, K, V, M> Visitor<'de> for InnerVisitor<K, V, M>
//         where 
//             K: Deserialize<'de> + std::cmp::Ord,
//             V: Deserialize<'de>,
//             M: Deserialize<'de> + std::cmp::Ord,
//         {
//             type Value = Expression<K, V, M>;

//             fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//                 formatter.write_str(" //TODO line 1121")
//             }

//             fn visit_map<Vi>(self, mut map: Vi) -> Result<Self::Value, Vi::Error>
//             where
//                 Vi: MapAccess<'de>,
//             {
//                 let keys = None;
//                 let valuse = None;
//                 while let Some(key) = map.next_key::<String>()? {
//                     match key.as_str() {
//                         "keys" => Ok(Self::Value::Variants(map.next_value()?)),
//                         "multipart" => Ok(Self::Value::MuitiPart(map.next_value()?)),
//                         _ => Err(de::Error::unknown_field(key.as_str(), FIELDS))
//                     }
//                 }
//                 Ok(Expression{
//                     keys
//                 })
//             }
//         }

//     }
// }