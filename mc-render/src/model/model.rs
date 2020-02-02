use std::rc::Rc;
use std::collections::btree_map::BTreeMap;
use std::io;
use std::fmt::Debug;

use cgmath::Matrix2;
use cgmath::Matrix3;
use cgmath::Vector2;
use cgmath::Vector3;

use crate::assets::data_type::Face;
use crate::assets::data_type::Rotate90;
use crate::assets::util::Provider;
use crate::assets::data_raw::FaceTextureRaw;
use crate::assets::data_raw::ElementRaw;
use crate::assets::data_raw::ModelRaw;
use crate::assets::data_raw::BlockStateRaw;
use crate::assets::data_raw::ApplyRaw;
use crate::assets::data_raw::AppliedModelRaw;
use super::blockstate::BlockState;

pub trait TextureGen {
    type Texture;

    fn get(&mut self, name: &str) -> Self::Texture;
}




/**
 * 
 */

impl Face {
    pub fn get_unit<S>(&self) -> Vector3<S> 
    where
        S: cgmath::Zero + cgmath::One + std::ops::Neg<Output = S>
    {
        match self {
            Self::East  => Vector3::new( S::one(),  S::zero(),  S::zero()),
            Self::West  => Vector3::new(-S::one(),  S::zero(),  S::zero()),
            Self::Up    => Vector3::new( S::zero(),  S::one(),  S::zero()),
            Self::Down  => Vector3::new( S::zero(), -S::one(),  S::zero()),
            Self::South => Vector3::new( S::zero(),  S::zero(),  S::one()),
            Self::North => Vector3::new( S::zero(),  S::zero(), -S::one()),
        }
    }
}


// pub fn transform_x(rotation: &Rotate90, cube: &mut [Face]) {
//     debug_assert_eq!(cube.len(), 6);
//     match rotation {
//         Rotate90::R0   => {   },
//         Rotate90::R90  => {
//             let tmp = cube[0].clone(); 
//             cube[0] = cube[2].clone(); 
//             cube[2] = cube[5].clone(); 
//             cube[5] = cube[3].clone(); 
//             cube[3] = tmp;   
//         },
//         Rotate90::R180 => {
//             let tmp1 = cube[0].clone();
//             let tmp2 = cube[2].clone();
//             cube[0] = cube[5].clone(); 
//             cube[2] = cube[3].clone(); 
//             cube[5] = tmp1; 
//             cube[3] = tmp2; 
//         },
//         Rotate90::R270 => {
//             let tmp = cube[5].clone();       
//             cube[5] = cube[2].clone(); 
//             cube[2] = cube[0].clone(); 
//             cube[0] = cube[3].clone(); 
//             cube[3] = tmp;   
//         },
//     }
// }


// pub fn transform_y(rotation: &Rotate90, cube: &mut [Face]) {
//     debug_assert_eq!(cube.len(), 6);
//     match rotation {
//         Rotate90::R0   => {   },
//         Rotate90::R90  => {
//             let tmp = cube[4].clone(); 
//             cube[4] = cube[2].clone(); 
//             cube[2] = cube[1].clone(); 
//             cube[1] = cube[3].clone(); 
//             cube[3] = tmp;   
//         },
//         Rotate90::R180 => {
//             let tmp1 = cube[4].clone();
//             let tmp2 = cube[2].clone();
//             cube[4] = cube[1].clone(); 
//             cube[2] = cube[3].clone(); 
//             cube[1] = tmp1; 
//             cube[3] = tmp2; 
//         },
//         Rotate90::R270 => {
//             let tmp = cube[1].clone();
//             cube[1] = cube[2].clone();        
//             cube[2] = cube[4].clone(); 
//             cube[4] = cube[3].clone(); 
//             cube[3] = tmp;   
//         },
//     }
// }


#[derive(Debug)]
pub struct TransformedModel<Tex> {

    pub model: Rc<Model<Tex>>,

    pub x: Rotate90,

    pub y: Rotate90,

    pub uvlock: bool,

}

impl<Tex> TransformedModel<Tex> {

    pub fn from_mxy(model: Rc<Model<Tex>>, x: Rotate90, y: Rotate90, uvlock: bool) -> Self {
        // let mut rotation = [Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0];
        // if !uvlock {
        //     rotation[1] = y.clone();
        //     rotation[4] = y.clone();
        //     rotation[0] = x.inverse();
        //     rotation[5] = x.clone();
        // }
        TransformedModel {
            model,
            x,
            y,
            uvlock
        }
    }

    pub fn mapping(&self, face: Face) -> Face {
        FACE_ROTATE[self.x.index()][self.y.index()][face.index()].clone()
    }

    pub fn inv_mapping(&self, face: Face) -> Face {
        FACE_ROTATE_INV[self.x.index()][self.y.index()][face.index()].clone()
    }

    pub fn rotation(&self, original_face: Face) -> Rotate90 {
        let rotate = if !self.uvlock {
            Rotate90::R0
        } else {
            - match original_face {
                Face::Up => self.y.clone(),
                Face::Down => -self.y.clone(),
                Face::East => -self.x.clone(),
                Face::West => self.x.clone(),
                Face::South => Rotate90::R0,
                Face::North => Rotate90::R0,
            }
        };
        rotate
    }

    pub fn mapping_transform(&self, t: &Matrix3<f32>) -> Matrix3<f32> {
        let tr = Matrix3::from_cols(
            self.inv_mapping(Face::East).get_unit(), 
            self.inv_mapping(Face::Up).get_unit(), 
            self.inv_mapping(Face::South).get_unit()
        );
        tr * t
    }
}

#[derive(Debug)]
pub struct Model<Tex> {

    pub ambientocclusion: bool,

    pub elements: Vec<Element<Tex>>,
}

#[derive(Debug)]
pub struct FaceTexture<Tex> {

    pub uv: Matrix2<f32>,

    pub cullface: Option<Face>,

    pub rotation: Rotate90,

    pub texture: Tex,

    pub tintindex: Option<usize>,

}

impl<Tex> FaceTexture<Tex> {

    pub fn get_face_vert(&self, rotation: Rotate90, bl: &mut Vector2<f32>, br: &mut Vector2<f32>, tl: &mut Vector2<f32>, tr: &mut Vector2<f32>) {
        
        macro_rules! cp_vec2 {
            ($e:expr, $n1:expr, $n2:expr) => {
                $e.x = $n1.x;
                $e.y = $n2.y;
            };
        }

        let rotation = self.rotation.clone() + rotation;
        match rotation {
            Rotate90::R0 => {
                cp_vec2!(bl, self.uv.x, self.uv.x);
                cp_vec2!(br, self.uv.y, self.uv.x);
                cp_vec2!(tr, self.uv.y, self.uv.y);
                cp_vec2!(tl, self.uv.x, self.uv.y);
            },
            Rotate90::R270  => {
                cp_vec2!(br, self.uv.x, self.uv.x);
                cp_vec2!(tr, self.uv.y, self.uv.x);
                cp_vec2!(tl, self.uv.y, self.uv.y);
                cp_vec2!(bl, self.uv.x, self.uv.y);
            },
            Rotate90::R180 => {
                cp_vec2!(tr, self.uv.x, self.uv.x);
                cp_vec2!(tl, self.uv.y, self.uv.x);
                cp_vec2!(bl, self.uv.y, self.uv.y);
                cp_vec2!(br, self.uv.x, self.uv.y);
            },
            Rotate90::R90 => {
                cp_vec2!(tl, self.uv.x, self.uv.x);
                cp_vec2!(bl, self.uv.y, self.uv.x);
                cp_vec2!(br, self.uv.y, self.uv.y);
                cp_vec2!(tr, self.uv.x, self.uv.y);
            },
        }
    }
}

#[derive(Debug)]
pub struct Cubic<S> {
    
    pub from: Vector3<S>,

    pub to: Vector3<S>,

}

impl<S: std::cmp::PartialOrd> Cubic<S> {

    pub fn rectify(mut self) -> Self {
        use std::mem::swap;
        if self.from.x > self.to.x {
            swap(&mut self.from.x, &mut self.to.x)
        }
        if self.from.y > self.to.y {
            swap(&mut self.from.y, &mut self.to.y)
        }
        if self.from.z > self.to.z {
            swap(&mut self.from.z, &mut self.to.z)
        }
        self
    }
}



impl<S: Copy> Cubic<S> {

    pub fn get_face_vert(&self, face: Face, bl: &mut Vector3<S>, br: &mut Vector3<S>, tl: &mut Vector3<S>, tr: &mut Vector3<S>) {
       
        macro_rules! cp_vec3 {
            ($e:expr, $n1:expr, $n2:expr, $n3:expr) => {
                $e.x = $n1.x;
                $e.y = $n2.y;
                $e.z = $n3.z;
            };
        }

        match face {
            Face::West  => { 
                cp_vec3!(bl, self.from, self.from, self.from);
                cp_vec3!(br, self.from, self.from, self.to  ); 
                cp_vec3!(tr, self.from, self.to  , self.to  ); 
                cp_vec3!(tl, self.from, self.to  , self.from);
            },
            Face::Down  => { 
                cp_vec3!(bl, self.from, self.from, self.from);
                cp_vec3!(br, self.to  , self.from, self.from); 
                cp_vec3!(tr, self.to  , self.from, self.to  ); 
                cp_vec3!(tl, self.from, self.from, self.to  );
            },
            Face::North  => { 
                cp_vec3!(bl, self.to  , self.from, self.from);
                cp_vec3!(br, self.from, self.from, self.from); 
                cp_vec3!(tr, self.from, self.to  , self.from); 
                cp_vec3!(tl, self.to  , self.to  , self.from);
            },
            Face::South  => { 
                cp_vec3!(bl, self.from, self.from, self.to  );
                cp_vec3!(br, self.to  , self.from, self.to  ); 
                cp_vec3!(tr, self.to , self.to  , self.to  ); 
                cp_vec3!(tl, self.from, self.to  , self.to  );
            },
            Face::Up  => { 
                cp_vec3!(bl, self.from, self.to  , self.to  );
                cp_vec3!(br, self.to  , self.to  , self.to  ); 
                cp_vec3!(tr, self.to  , self.to  , self.from); 
                cp_vec3!(tl, self.from, self.to  , self.from);
            },
            Face::East  => { 
                cp_vec3!(bl, self.to  , self.from, self.to  );
                cp_vec3!(br, self.to  , self.from, self.from); 
                cp_vec3!(tr, self.to  , self.to  , self.from); 
                cp_vec3!(tl, self.to  , self.to  , self.to  );
            },
        };
        
    }
}


#[derive(Debug)]
pub struct Element<Tex> {

    pub cubic: Cubic<f32>,

    pub rotation: (Vector3<f32>, Matrix3<f32>),

    pub faces: [Option<FaceTexture<Tex>>; 6],

    pub shade: bool,
    
}

// notice that x-rotate is inv-clockwise(right hand) and y-rotate is clockwise
pub const FACE_ROTATE: [[[Face;6];4];4] = [
    [
        [Face::West,Face::Down,Face::North,Face::South,Face::Up,Face::East],
        [Face::South,Face::Down,Face::West,Face::East,Face::Up,Face::North],
        [Face::East,Face::Down,Face::South,Face::North,Face::Up,Face::West],
        [Face::North,Face::Down,Face::East,Face::West,Face::Up,Face::South]
    ],
    [
        [Face::West,Face::South,Face::Down,Face::Up,Face::North,Face::East],
        [Face::Up,Face::South,Face::West,Face::East,Face::North,Face::Down],
        [Face::East,Face::South,Face::Up,Face::Down,Face::North,Face::West],
        [Face::Down,Face::South,Face::East,Face::West,Face::North,Face::Up]
    ],
    [
        [Face::West,Face::Up,Face::South,Face::North,Face::Down,Face::East],
        [Face::North,Face::Up,Face::West,Face::East,Face::Down,Face::South],
        [Face::East,Face::Up,Face::North,Face::South,Face::Down,Face::West],
        [Face::South,Face::Up,Face::East,Face::West,Face::Down,Face::North]
    ],
    [
        [Face::West,Face::North,Face::Up,Face::Down,Face::South,Face::East],
        [Face::Down,Face::North,Face::West,Face::East,Face::South,Face::Up],
        [Face::East,Face::North,Face::Down,Face::Up,Face::South,Face::West],
        [Face::Up,Face::North,Face::East,Face::West,Face::South,Face::Down]
    ]
];

pub const FACE_ROTATE_INV: [[[Face;6];4];4] = [
    [
        [Face::West,Face::Down,Face::North,Face::South,Face::Up,Face::East],
        [Face::North,Face::Down,Face::East,Face::West,Face::Up,Face::South],
        [Face::East,Face::Down,Face::South,Face::North,Face::Up,Face::West],
        [Face::South,Face::Down,Face::West,Face::East,Face::Up,Face::North]
    ],
    [
        [Face::West,Face::North,Face::Up,Face::Down,Face::South,Face::East],
        [Face::North,Face::East,Face::Up,Face::Down,Face::West,Face::South],
        [Face::East,Face::South,Face::Up,Face::Down,Face::North,Face::West],
        [Face::South,Face::West,Face::Up,Face::Down,Face::East,Face::North]
    ],
    [
        [Face::West,Face::Up,Face::South,Face::North,Face::Down,Face::East],
        [Face::North,Face::Up,Face::West,Face::East,Face::Down,Face::South],
        [Face::East,Face::Up,Face::North,Face::South,Face::Down,Face::West],
        [Face::South,Face::Up,Face::East,Face::West,Face::Down,Face::North]
    ],
    [
        [Face::West,Face::South,Face::Down,Face::Up,Face::North,Face::East],
        [Face::North,Face::West,Face::Down,Face::Up,Face::East,Face::South],
        [Face::East,Face::North,Face::Down,Face::Up,Face::South,Face::West],
        [Face::South,Face::East,Face::Down,Face::Up,Face::West,Face::North]
    ]
];


/**
 * 
 */
impl<Tex> FaceTexture<Tex> {

    pub fn from_raw<'a>(raw: &FaceTextureRaw, tex_gen: &'a mut dyn TextureGen<Texture = Tex>) -> Self {
        FaceTexture {
            uv: {
                let arr = match &raw.uv {
                    Some(a) => a.as_ref(),
                    None => &[0.0, 0.0, 16.0, 16.0],
                };
                Matrix2::from_cols(Vector2::new(arr[0], arr[1]), Vector2::new(arr[2], arr[3]))
            },
            cullface: raw.cullface.clone(),
            texture: tex_gen.get(raw.texture.as_str()),
            rotation: raw.rotation.clone().unwrap_or_else(|| Rotate90::R0),
            tintindex: raw.tintindex,
        }
    } 

}


impl<Tex> Element<Tex> {

    pub fn from_raw<'a>(raw: &ElementRaw, tex_gen: &'a mut dyn TextureGen<Texture = Tex>) -> Self {
        use crate::assets::data_type::Axis;
        use cgmath::Deg;
        Element {
            cubic: Cubic {
                from: Vector3::from(raw.from),
                to: Vector3::from(raw.to),
            },
            shade: raw.shade,
            rotation: {
                let r = raw.rotation.clone().unwrap_or_default();
                let origin = Vector3::from(r.origin);
                let transf = match r.axis {
                    Axis::X => Matrix3::from_angle_x(Deg(r.angle)),
                    Axis::Y => Matrix3::from_angle_y(Deg(r.angle)),
                    Axis::Z => Matrix3::from_angle_z(Deg(r.angle)),
                };
                (origin, transf)
            },
            faces: {
                let mut faces = [None, None, None, None, None, None];
                for (k, v) in &raw.faces {
                    faces[k.index()] = Some(FaceTexture::from_raw(v, tex_gen));
                }
                faces
            }
        }
    } 
}


impl<Tex> Model<Tex> {

    pub fn from_raw<'a>(raw: &ModelRaw, tex_gen: &'a mut dyn TextureGen<Texture = Tex>) -> Self {
        Model {
            ambientocclusion: raw.ambientocclusion,
            elements: {
                let mut elements = Vec::with_capacity(raw.elements.len());
                for element in &raw.elements {
                    elements.push(Element::from_raw(element, tex_gen))
                }
                elements
            }
        }
    }
}


/**
 * 
 */
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct KVPair {

    pub key: String,

    pub val: String,

}

impl std::fmt::Debug for KVPair {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.key, self.val)
    }
}
/**
 * 
 */


struct IndexTexGen<'a, Tex> {
    index: &'a BTreeMap<String, String>,
    tex_gen: &'a mut dyn TextureGen<Texture = Tex>,
}

impl<'a, Tex> TextureGen for IndexTexGen<'a, Tex> {
    type Texture = Tex;

    fn get(&mut self, name: &str) -> Self::Texture {
        let mut u = name;
        while u.starts_with('#') {
            u = &u[1..];
            u = match self.index.get(u) {
                Some(v) => v,
                None => return self.tex_gen.get(name),
            };
        }
        self.tex_gen.get(u)
    }
}


pub struct BlockModelBuilder<'a, Tex> {

    bs_pvd: &'a mut dyn Provider<Item = BlockStateRaw>,

    mdl_pvd: &'a mut dyn Provider<Item = ModelRaw>,

    tex_gen: &'a mut dyn TextureGen<Texture = Tex>,

    mdl_cache: BTreeMap<String, Rc<Model<Tex>>>,
}

impl<'a, Tex> BlockModelBuilder<'a, Tex> {

    pub fn new(
        bs_pvd: &'a mut dyn Provider<Item = BlockStateRaw>,
        mdl_pvd: &'a mut dyn Provider<Item = ModelRaw>,
        tex_gen: &'a mut dyn TextureGen<Texture = Tex>,
    ) -> Self {
        BlockModelBuilder {
            bs_pvd,
            mdl_pvd,
            tex_gen,
            mdl_cache: BTreeMap::new()
        }
    }

    pub fn build(&mut self, name: &str) -> io::Result<BlockState<String, Rc<TransformedModel<Tex>>>> {
        use std::collections::btree_map::Entry;
        use crate::assets::data_raw::Merge;

        // impl PartialEq for AppliedModelRaw {
        //     fn eq(&self, other: &AppliedModelRaw) -> bool {
        //         self.model == other.model && self.uvlock == other.uvlock && self.x == other.x && self.y == other.y
        //     }
        // }

        // impl PartialOrd for AppliedModelRaw {
        //     fn partial_cmp(&self, other: &AppliedModelRaw) -> Option<Ordering> {
        //         match self.model.partial_cmp(&other.model)? {
        //             Ordering::Greater => return Some(Ordering::Greater),
        //             Ordering::Less => return Some(Ordering::Greater),
        //             Ordering::Equal => { },
        //         };
        //         match self.x.partial_cmp(&other.x)? {
        //             Ordering::Greater => return Some(Ordering::Greater),
        //             Ordering::Less => return Some(Ordering::Greater),
        //             Ordering::Equal => { },
        //         };
        //         match self.y.partial_cmp(&other.y)? {
        //             Ordering::Greater => return Some(Ordering::Greater),
        //             Ordering::Less => return Some(Ordering::Greater),
        //             Ordering::Equal => { },
        //         };
        //         match self.uvlock.partial_cmp(&other.uvlock)? {
        //             Ordering::Greater => return Some(Ordering::Greater),
        //             Ordering::Less => return Some(Ordering::Greater),
        //             Ordering::Equal => Some(Ordering::Equal),
        //         }
        //     }
        // }

        // impl Eq for AppliedModelRaw {

        // }

        // impl Ord for AppliedModelRaw {
        //     fn cmp(&self, other: &Self) -> Ordering {
        //         match self.model.cmp(&other.model) {
        //             Ordering::Greater => return Ordering::Greater,
        //             Ordering::Less => return Ordering::Greater,
        //             Ordering::Equal => { },
        //         };
        //         match self.x.cmp(&other.x) {
        //             Ordering::Greater => return Ordering::Greater,
        //             Ordering::Less => return Ordering::Greater,
        //             Ordering::Equal => { },
        //         };
        //         match self.y.cmp(&other.y) {
        //             Ordering::Greater => return Ordering::Greater,
        //             Ordering::Less => return Ordering::Greater,
        //             Ordering::Equal => { },
        //         };
        //         match self.uvlock.cmp(&other.uvlock) {
        //             Ordering::Greater => return Ordering::Greater,
        //             Ordering::Less => return Ordering::Greater,
        //             Ordering::Equal => Ordering::Equal,
        //         }
        //     }
        // }

        let mdl_pvd = &mut self.mdl_pvd;
        let tex_gen = &mut self.tex_gen;
        let mdl_cache = &mut self.mdl_cache;
        let mut transf_apply = |v: ApplyRaw| -> io::Result<Rc<TransformedModel<Tex>>> {
            let v = v.get_fast();
            
            let model = match mdl_cache.entry(v.model.clone()) {
                Entry::Occupied(oc) => oc.get().clone(),
                Entry::Vacant(vc) => {
                    let mut mdl_raw = mdl_pvd.provide(vc.key()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, vc.key().to_string()))?;
                    while let Some(s) = &mdl_raw.parent {
                        let parent = mdl_pvd.provide(s).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, s.to_string()))?;
                        mdl_raw.merge(&parent);
                    }
                    let mut itex_gen = IndexTexGen{ 
                        index: mdl_raw.textures.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "texture"))?, 
                        tex_gen: *tex_gen
                    };
                    let rcmodel = Rc::new(Model::from_raw(&mdl_raw, &mut itex_gen));
                    vc.insert(rcmodel).clone()
                }
            };
            Ok(Rc::new(TransformedModel::from_mxy(model, v.x.clone(), v.y.clone(), v.uvlock)))
        };

        if let Some(bs_raw) = self.bs_pvd.provide(name) {
            match bs_raw {
                BlockStateRaw::Variants(v_raw) => {
                    let mut blockstate = BlockState::new();
                    blockstate.start_group();
                    for(keys, apply_raw) in v_raw.into_iter() {
                        blockstate.insert_group(keys.iter(), transf_apply(apply_raw)?);
                    }
                    Ok(blockstate)
                },
                BlockStateRaw::MultiPart(m_raw) => {
                    let mut blockstate = BlockState::new();
                    for(mut group, apply_raw) in m_raw.into_iter() {
                        let model = transf_apply(apply_raw)?;
                        blockstate.start_group();
                        loop {
                            let it = group.line_iter();
                            blockstate.insert_group(it, model.clone());
                            if !group.update_iter() {
                                break;
                            }
                        }
                    }
                    Ok(blockstate)
                }
            }
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, name.to_string()))
        }      
    }
}

