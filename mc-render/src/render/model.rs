use std::rc::Rc;
use std::collections::btree_map::BTreeMap;
use std::io;
use std::fmt::Debug;

use cgmath::Matrix2;
use cgmath::Matrix3;
use cgmath::Vector2;
use cgmath::Vector3;

use crate::assets::math::Face;
use crate::assets::math::Rotate90;
use crate::assets::math::Expression;
use crate::assets::util::Provider;
use crate::assets::data_raw::FaceTextureRaw;
use crate::assets::data_raw::ElementRaw;
use crate::assets::data_raw::ModelRaw;
use crate::assets::data_raw::BlockStateRaw;
use crate::assets::data_raw::ApplyRaw;
use crate::assets::data_raw::AppliedModelRaw;

use super::block;
use super::block::Block;
use super::block::MCBlockLoc;

type Render<Tex> = dyn BlockRenderer<Texture = Tex>;
type World<S, L> = dyn block::World<Liquid = L, Solid = S>;

/**
 * Render: register texture map => load biome-color/
 */ 
pub trait BlockRenderer {
    type Texture;

    /**
     *  draw face
     */ 
    fn draw_textured(&mut self, loc: &Vector3<f32>, mesh: &Mesh<Self::Texture>);

    /**
     *  draw transparent textured block
     */
    fn draw_water(&mut self, loc: &Vector3<f32>, mesh: &Mesh<Self::Texture>);

    fn flush(&mut self);

}


pub trait TextureGen {
    type Texture;

    fn get(&mut self, name: &str) -> Self::Texture;
}


pub trait RenderableBlock {
    type Model;

    fn get_models<'a>(&'a self) -> &'a [Self::Model];

    fn get_inline_color(&self, tintindex: usize) -> [f32; 4] {
        let _ = tintindex;
        [1.0, 1.0, 1.0, 1.0]
    }
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
            Face::East  => Vector3::new( S::one(),  S::zero(),  S::zero()),
            Face::West  => Vector3::new(-S::one(),  S::zero(),  S::zero()),
            Face::Up    => Vector3::new( S::zero(),  S::one(),  S::zero()),
            Face::Down  => Vector3::new( S::zero(), -S::one(),  S::zero()),
            Face::South => Vector3::new( S::zero(),  S::zero(),  S::one()),
            Face::North => Vector3::new( S::zero(),  S::zero(), -S::one()),
        }
    }
}


pub fn transform_x(rotation: &Rotate90, cube: &mut [Face]) {
    debug_assert_eq!(cube.len(), 6);
    match rotation {
        Rotate90::R0   => {   },
        Rotate90::R90  => {
            let tmp = cube[0].clone(); 
            cube[0] = cube[2].clone(); 
            cube[2] = cube[5].clone(); 
            cube[5] = cube[3].clone(); 
            cube[3] = tmp;   
        },
        Rotate90::R180 => {
            let tmp1 = cube[0].clone();
            let tmp2 = cube[2].clone();
            cube[0] = cube[5].clone(); 
            cube[2] = cube[3].clone(); 
            cube[5] = tmp1; 
            cube[3] = tmp2; 
        },
        Rotate90::R270 => {
            let tmp = cube[5].clone();       
            cube[5] = cube[2].clone(); 
            cube[2] = cube[0].clone(); 
            cube[0] = cube[3].clone(); 
            cube[3] = tmp;   
        },
    }
}


pub fn transform_y(rotation: &Rotate90, cube: &mut [Face]) {
    debug_assert_eq!(cube.len(), 6);
    match rotation {
        Rotate90::R0   => {   },
        Rotate90::R90  => {
            let tmp = cube[4].clone(); 
            cube[4] = cube[2].clone(); 
            cube[2] = cube[1].clone(); 
            cube[1] = cube[3].clone(); 
            cube[3] = tmp;   
        },
        Rotate90::R180 => {
            let tmp1 = cube[4].clone();
            let tmp2 = cube[2].clone();
            cube[4] = cube[1].clone(); 
            cube[2] = cube[3].clone(); 
            cube[1] = tmp1; 
            cube[3] = tmp2; 
        },
        Rotate90::R270 => {
            let tmp = cube[1].clone();
            cube[1] = cube[2].clone();        
            cube[2] = cube[4].clone(); 
            cube[4] = cube[3].clone(); 
            cube[3] = tmp;   
        },
    }
}


/**
 * 
 */
#[derive(Debug)]
pub struct Mesh<Tex> {
    
    pub geo_vert: [Vector3<f32>; 4],

    pub tex_vert: [Vector2<f32>; 4],

    pub texture: Tex,

    pub color: [f32; 4],

    pub transform: (Vector3<f32>, Matrix3<f32>)

}

#[derive(Debug)]
pub struct TransformedModel<Tex> {

    pub model: Rc<Model<Tex>>,

    pub mapping: [Face; 6],

    pub rotation: [Rotate90; 6],

    pub inv_mapping: [Face; 6],

}

impl<Tex> TransformedModel<Tex> {

    pub fn from_mxy(model: Rc<Model<Tex>>, x: Rotate90, y: Rotate90, uvlock: bool) -> Self {
        let mut mapping = [Face::West, Face::Down, Face::North, Face::South, Face::Up, Face::East];
        let mut inv_mapping = [Face::West, Face::Down, Face::North, Face::South, Face::Up, Face::East];
        let mut rotation = [Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0, Rotate90::R0];
        transform_x(&x, &mut mapping);
        transform_y(&y, &mut mapping);
        transform_y(&y.inverse(), &mut inv_mapping);
        transform_x(&x.inverse(), &mut inv_mapping);
        if !uvlock {
            rotation[1] = y.inverse();
            rotation[4] = y.clone();
            rotation[0] = x.inverse();
            rotation[5] = x.clone();
        }
        TransformedModel {
            model,
            mapping,
            inv_mapping,
            rotation
        }
    }

    pub fn mapping<'a>(&'a self, face: &Face) -> &'a Face {
        &self.mapping[face.index()]
    }

    pub fn inv_mapping<'a>(&'a self, face: &Face) -> &'a Face {
        &self.inv_mapping[face.index()]
    }

    pub fn rotation<'a>(&'a self, face: &Face) -> &'a Rotate90 {
        &self.rotation[face.index()]
    }

    pub fn mapping_transform(&self, t: &Matrix3<f32>) -> Matrix3<f32> {
        let tr = Matrix3::from_cols(
            self.mapping[5].get_unit(), 
            self.mapping[4].get_unit(), 
            self.mapping[3].get_unit()
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

    pub fn get_face_vert(&self) -> [Vector2<f32>; 4] {
        let uv = &self.uv;
        let origin = [
            uv.x,
            Vector2::new(uv.y.x, uv.x.y),
            Vector2::new(uv.x.x, uv.y.y),
            uv.y,
        ];
        match self.rotation {
            Rotate90::R0   => [origin[0], origin[1], origin[3], origin[2]],
            Rotate90::R90  => [origin[1], origin[3], origin[2], origin[0]],
            Rotate90::R180 => [origin[3], origin[2], origin[0], origin[1]],
            Rotate90::R270 => [origin[2], origin[0], origin[1], origin[3]],
        }
    }
}

#[derive(Debug)]
pub struct Element<Tex> {

    pub from: Vector3<f32>,

    pub to: Vector3<f32>,

    pub rotation: (Vector3<f32>, Matrix3<f32>),

    pub faces: [Option<FaceTexture<Tex>>; 6],

    pub shade: bool,
    
}


impl<Tex> Element<Tex> {

    pub fn get_face_vert<'a>(&self, face: &Face, rotation: &Rotate90) -> [Vector3<f32>; 4] {
        let p0 = &self.from;
        let p1 = &self.to;
        let origin = match face {
            Face::West  => [Vector3::new(p0.x, p0.y, p0.z), Vector3::new(p0.x, p0.y, p1.z), Vector3::new(p0.x, p1.y, p0.z), Vector3::new(p0.x, p1.y, p1.z)],
            Face::Down  => [Vector3::new(p0.x, p0.y, p0.z), Vector3::new(p1.x, p0.y, p0.z), Vector3::new(p0.x, p0.y, p1.z), Vector3::new(p1.x, p0.y, p1.z)],
            Face::North => [Vector3::new(p1.x, p0.y, p0.z), Vector3::new(p0.x, p0.y, p0.z), Vector3::new(p1.x, p1.y, p0.z), Vector3::new(p0.x, p1.y, p0.z)],
            Face::South => [Vector3::new(p1.x, p0.y, p1.z), Vector3::new(p1.x, p0.y, p0.z), Vector3::new(p1.x, p1.y, p1.z), Vector3::new(p1.x, p1.y, p0.z)],
            Face::Up    => [Vector3::new(p0.x, p1.y, p1.z), Vector3::new(p1.x, p1.y, p1.z), Vector3::new(p0.x, p1.y, p0.z), Vector3::new(p1.x, p1.y, p0.z)],
            Face::East  => [Vector3::new(p0.x, p0.y, p1.z), Vector3::new(p1.x, p0.y, p1.z), Vector3::new(p0.x, p1.y, p1.z), Vector3::new(p1.x, p1.y, p1.z)],
        };
        match rotation {
            Rotate90::R0   => [origin[0], origin[1], origin[3], origin[2]],
            Rotate90::R90  => [origin[1], origin[3], origin[2], origin[0]],
            Rotate90::R180 => [origin[3], origin[2], origin[0], origin[1]],
            Rotate90::R270 => [origin[2], origin[0], origin[1], origin[3]],
        }
    }

}

// pub const RECT_ROTATE_INDEX: [[usize;4];4] = [[0,1,3,2],[1,3,2,0],[3,2,0,1],[2,0,1,3]];
// pub const CUBE_FACE_INDEX: [[usize;4];6] = [[0,4,2,6],[0,1,4,5],[1,0,3,2],[5,1,7,3],[6,7,2,3],[4,5,6,7]];
// pub const FACE_ROTATEX: [[Face;6];4] = [
//     [Face::West,Face::Down,Face::North,Face::South,Face::Up,Face::East],
//     [Face::West,Face::South,Face::Down,Face::Up,Face::North,Face::East],
//     [Face::West,Face::Up,Face::South,Face::North,Face::Down,Face::East],
//     [Face::West,Face::North,Face::Up,Face::Down,Face::South,Face::East]
// ];
// pub const FACE_ROTATEY: [[Face;6];4] = [
//     [Face::West,Face::Down,Face::North,Face::South,Face::Up,Face::East],
//     [Face::North,Face::Down,Face::East,Face::West,Face::Up,Face::South],
//     [Face::East,Face::Down,Face::South,Face::North,Face::Up,Face::West],
//     [Face::South,Face::Down,Face::West,Face::East,Face::Up,Face::North]
// ];


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
        use crate::assets::math::Axis;
        use cgmath::Deg;
        Element {
            from: Vector3::from(raw.from),
            to: Vector3::from(raw.to),
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

pub fn draw<S, L, Tex>(loc: &MCBlockLoc, world: &mut World<S, L>, faces: &[Face], render: &mut Render<Tex>) -> bool
where
    Tex: Clone,
    S: RenderableBlock<Model = TransformedModel<Tex>>,
    L: RenderableBlock<Model = TransformedModel<Tex>>,
{
    match world.get(loc) {
        Block::Air => false,
        Block::Solid(solid) => {
            let vecloc = Vector3::new(loc.x_f32(), loc.y_f32(), loc.z_f32());
            for tmodel in solid.get_models() {
                let model = tmodel.model.as_ref();
                for element in model.elements.as_slice() {
                    for face in faces {
                        let mface = tmodel.mapping(face);
                        if let Some(face_tex) = &element.faces[mface.index()] {
                            if let Some(cullface) = &face_tex.cullface {
                                let cullface = &tmodel.inv_mapping(cullface);
                                if let Some(pos) = loc.near(cullface) {
                                    if !world.is_air(&pos) {
                                        continue;
                                    }
                                }
                            }   // cullface
                            let mesh = Mesh {
                                geo_vert: element.get_face_vert(mface, tmodel.rotation(face)),
                                tex_vert: face_tex.get_face_vert(),
                                texture: face_tex.texture.clone(),
                                color: face_tex.tintindex.map(|tintindex| solid.get_inline_color(tintindex)).unwrap_or_else(|| [1.0; 4]),
                                transform: (element.rotation.0, tmodel.mapping_transform(&element.rotation.1))
                            };
                            render.draw_textured(&vecloc, &mesh);
                        }
                    }
                }
            }
            true
        },
        Block::Liquid(liquid) => {
            let vecloc = Vector3::new(loc.x_f32(), loc.y_f32(), loc.z_f32());
            for tmodel in liquid.get_models() {
                let model = tmodel.model.as_ref();
                for element in model.elements.as_slice() {
                    for face in faces {
                        let mface = tmodel.mapping(face);
                        if let Some(face_tex) = &element.faces[mface.index()] {
                            if let Some(cullface) = &face_tex.cullface {
                                let cullface = &tmodel.inv_mapping(cullface);
                                if let Some(pos) = loc.near(cullface) {
                                    if !world.is_air(&pos) {
                                        continue;
                                    }
                                }
                            }   // cullface
                            let mesh = Mesh {
                                geo_vert: element.get_face_vert(mface, tmodel.rotation(face)),
                                tex_vert: face_tex.get_face_vert(),
                                texture: face_tex.texture.clone(),
                                color: face_tex.tintindex.map(|tintindex| liquid.get_inline_color(tintindex)).unwrap_or_else(|| [1.0; 4]),
                                transform: (element.rotation.0, tmodel.mapping_transform(&element.rotation.1))
                            };
                            render.draw_water(&vecloc, &mesh);
                        }
                    }
                }
            }
            true
        },
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

    pub fn build(&mut self, name: &str) -> io::Result<Expression<KVPair, Vec<Rc<TransformedModel<Tex>>>, usize>> {
        use std::collections::btree_map::Entry;
        use std::cmp::Ordering;
        use crate::assets::data_raw::Merge;

        impl PartialEq for AppliedModelRaw {
            fn eq(&self, other: &AppliedModelRaw) -> bool {
                self.model == other.model && self.uvlock == other.uvlock && self.x == other.x && self.y == other.y
            }
        }

        impl PartialOrd for AppliedModelRaw {
            fn partial_cmp(&self, other: &AppliedModelRaw) -> Option<Ordering> {
                match self.model.partial_cmp(&other.model)? {
                    Ordering::Greater => return Some(Ordering::Greater),
                    Ordering::Less => return Some(Ordering::Greater),
                    Ordering::Equal => { },
                };
                match self.x.partial_cmp(&other.x)? {
                    Ordering::Greater => return Some(Ordering::Greater),
                    Ordering::Less => return Some(Ordering::Greater),
                    Ordering::Equal => { },
                };
                match self.y.partial_cmp(&other.y)? {
                    Ordering::Greater => return Some(Ordering::Greater),
                    Ordering::Less => return Some(Ordering::Greater),
                    Ordering::Equal => { },
                };
                match self.uvlock.partial_cmp(&other.uvlock)? {
                    Ordering::Greater => return Some(Ordering::Greater),
                    Ordering::Less => return Some(Ordering::Greater),
                    Ordering::Equal => Some(Ordering::Equal),
                }
            }
        }

        impl Eq for AppliedModelRaw {

        }

        impl Ord for AppliedModelRaw {
            fn cmp(&self, other: &Self) -> Ordering {
                match self.model.cmp(&other.model) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Equal => { },
                };
                match self.x.cmp(&other.x) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Equal => { },
                };
                match self.y.cmp(&other.y) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Equal => { },
                };
                match self.uvlock.cmp(&other.uvlock) {
                    Ordering::Greater => return Ordering::Greater,
                    Ordering::Less => return Ordering::Greater,
                    Ordering::Equal => Ordering::Equal,
                }
            }
        }

        if let Some(bs_raw) = self.bs_pvd.provide(name) {
            let fk = |s: String| -> io::Result<KVPair> {
                let sp = s.find('=').ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, s.clone()))?;
                let pair = s.split_at(sp);
                Ok(KVPair {
                    key: pair.0.to_string(),
                    val: pair.1.to_string()
                })
            };
            let bs = match bs_raw {
                BlockStateRaw::Variants(v_raw) => {
                    let mut cache: BTreeMap<AppliedModelRaw, Rc<TransformedModel<Tex>>> = BTreeMap::new();
                    let fv = |v: ApplyRaw| -> io::Result<Vec<Rc<TransformedModel<Tex>>>> {
                        let v = v.get_fast();
                        let tm = match cache.entry(v.clone()) {
                            Entry::Occupied(oc) => oc.get().clone(),
                            Entry::Vacant(vc) => {
                                let model = match self.mdl_cache.entry(v.model.clone()) {
                                    Entry::Occupied(oc) => oc.get().clone(),
                                    Entry::Vacant(vc) => {
                                        let mut mdl_raw = self.mdl_pvd.provide(vc.key()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, vc.key().to_string()))?;
                                        while let Some(s) = &mdl_raw.parent {
                                            let parent = self.mdl_pvd.provide(s).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, s.to_string()))?;
                                            mdl_raw.merge(&parent);
                                        }
                                        let mut itex_gen = IndexTexGen{ 
                                            index: mdl_raw.textures.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "texture"))?, 
                                            tex_gen: self.tex_gen
                                        };
                                        let rcmodel = Rc::new(Model::from_raw(&mdl_raw, &mut itex_gen));
                                        vc.insert(rcmodel).clone()
                                    }
                                };
                                vc.insert(Rc::new(TransformedModel::from_mxy(model, v.x.clone(), v.y.clone(), v.uvlock))).clone()
                            }
                        };
                        Ok(vec![tm])
                    };
                    v_raw.0.transf_into(fk, fv)?
                },
                BlockStateRaw::MuitiPart(m_raw) => {
                    let mut cache: BTreeMap<AppliedModelRaw, Rc<TransformedModel<Tex>>> = BTreeMap::new();
                    let fv = |vs: Vec<ApplyRaw>| -> io::Result<Vec<Rc<TransformedModel<Tex>>>> {
                        let mut res = Vec::with_capacity(vs.len());
                        for v in vs.into_iter() {
                            let v = v.get_fast();
                            let tm = match cache.entry(v.clone()) {
                                Entry::Occupied(oc) => oc.get().clone(),
                                Entry::Vacant(vc) => {
                                    let model = match self.mdl_cache.entry(v.model.clone()) {
                                        Entry::Occupied(oc) => oc.get().clone(),
                                        Entry::Vacant(vc) => {
                                            let mut mdl_raw = self.mdl_pvd.provide(vc.key()).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, vc.key().to_string()))?;
                                            while let Some(s) = &mdl_raw.parent {
                                                let parent = self.mdl_pvd.provide(s).ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, s.to_string()))?;
                                                mdl_raw.merge(&parent);
                                            }
                                            let mut itex_gen = IndexTexGen{ 
                                                index: mdl_raw.textures.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "texture"))?, 
                                                tex_gen: self.tex_gen
                                            };
                                            let rcmodel = Rc::new(Model::from_raw(&mdl_raw, &mut itex_gen));
                                            vc.insert(rcmodel).clone()
                                        }
                                    };
                                    vc.insert(Rc::new(TransformedModel::from_mxy(model, v.x.clone(), v.y.clone(), v.uvlock))).clone()
                                }
                            };
                            res.push(tm);
                        }
                        Ok(res)
                    };
                    m_raw.0.transf_into(fk, fv)?
                }
            };
            Ok(bs)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, name.to_string()))
        }      
    }
}

