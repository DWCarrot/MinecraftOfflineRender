use std::slice::Iter as SliceIter;

use cgmath::Vector3;

use crate::assets::data_type::Face;


impl Face {

    pub fn near(&self, loc: &Vector3<i32>) -> Option<Vector3<i32>> {
        match self {
            Face::West => {
                if loc.x > std::i32::MIN {
                    Some(Vector3::new(loc.x - 1, loc.y, loc.z))
                } else {
                    None
                }
            },
            Face::Down => {
                if loc.y > 0 {
                    Some(Vector3::new(loc.x, loc.y - 1, loc.z))
                } else {
                    None
                }
            },
            Face::North => {
                if loc.z > std::i32::MIN {
                    Some(Vector3::new(loc.x, loc.y, loc.z - 1))
                } else {
                    None
                }
            },
            Face::South => {
                if loc.z < std::i32::MAX {
                    Some(Vector3::new(loc.x, loc.y, loc.z + 1))
                } else {
                    None
                }
            },
            Face::Up => {
                if loc.y < 255 {
                    Some(Vector3::new(loc.x, loc.y + 1, loc.z))
                } else {
                    None
                }
            },
            Face::East => {
                if loc.x < std::i32::MAX {
                    Some(Vector3::new(loc.x + 1, loc.y, loc.z))
                } else {
                    None
                }
            },
        }
    }

}


pub trait RenderableBlock {
    type Model;

    fn is_air(&self) -> bool;

    fn is_water(&self) -> bool;

    fn is_water_logged(&self) -> bool;

    fn get_models<'a>(&'a self) -> SliceIter<'a, Self::Model>;

    fn get_water_models<'a>(&'a self) -> SliceIter<'a, Self::Model>;

    fn get_inline_color(&self, tintindex: usize) -> [u8; 4] {
        let _ = tintindex;
        [255, 255, 255, 255]
    }

    fn get_light(&self) -> u8;
    
}


pub trait World {
    type Block: RenderableBlock;

    fn get(&self, loc: &Vector3<i32>) -> Self::Block;
    
    fn is_air(&self, loc: &Vector3<i32>) -> bool {
        self.get(loc).is_air()
    }

    fn is_water(&self, loc: &Vector3<i32>) -> bool {
        self.get(loc).is_water()
    }

    fn is_water_logged(&self, loc: &Vector3<i32>) -> bool {
        self.get(loc).is_water_logged()
    }

}




