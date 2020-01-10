use crate::assets::math::Face;


pub enum Block<L, S> {

    Air,

    Liquid(L),

    Solid(S),
}


pub trait World {
    type Liquid;
    type Solid;

    fn get(&mut self, loc: &MCBlockLoc) -> Block<Self::Liquid, Self::Solid>;
    
    fn is_air(&mut self, loc: &MCBlockLoc) -> bool {
        match self.get(loc) {
            Block::Air => true,
            _ => false
        }
    }

    fn is_liquid(&mut self, loc: &MCBlockLoc) -> bool {
        match self.get(loc) {
            Block::Liquid(_) => true,
            _ => false
        }
    }

    fn is_solid(&mut self, loc: &MCBlockLoc) -> bool {
        match self.get(loc) {
            Block::Solid(_) => true,
            _ => false
        }
    }
}


/**
 * 
 */


/**
 * 
 */
pub struct MCBlockLoc {
    
    pub x: i32,

    pub y: u8, 
    
    pub z: i32
}

impl MCBlockLoc {

    pub fn new(x: i32, y: u8, z: i32) -> Self {
        MCBlockLoc {
            x,
            y,
            z
        }
    }

    pub fn x_f32(&self) -> f32 { 
        self.x as f32
    }

    pub fn y_f32(&self) -> f32 { 
        self.y as f32
    }

    pub fn z_f32(&self) -> f32 { 
        self.z as f32
    }

    pub fn near(&self, face: &Face) -> Option<Self> {
        match face {
            Face::West => {
                if self.x == std::i32::MIN {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x - 1, y: self.y, z: self.z })
                }
            },
            Face::Down => {
                if self.y == std::u8::MIN {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x, y: self.y - 1, z: self.z })
                }
            },
            Face::North => {
                if self.z == std::i32::MIN {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x, y: self.y, z: self.z - 1 })
                }
            },
            Face::South => {
                if self.z == std::i32::MAX {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x, y: self.y, z: self.z + 1 })
                }
            },
            Face::Up => {
                if self.y == std::u8::MAX {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x, y: self.y + 1, z: self.z })
                }
            },
            Face::East => {
                if self.x == std::i32::MAX {
                    None
                } else {
                    Some(MCBlockLoc{ x: self.x + 1, y: self.y, z: self.z })
                }
            },
        }
    }
}
