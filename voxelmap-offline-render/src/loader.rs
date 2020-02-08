use std::io::Read;
use std::io::Seek;
use std::str::Split;
use std::convert::TryFrom;

use zip::ZipArchive;

use mc_render::model;
use mc_render::model::biome::BiomeColor;
use mc_render::model::biome::Biome;
use mc_render::model::model::RefModel;
use mc_render::glrender::texture::CombinedTexture;

pub type GEResult<T> = Result<T, Box<dyn std::error::Error>>;
pub type Model = RefModel<CombinedTexture>;
pub type ModelProvider = model::ModelProvider<CombinedTexture>;

pub struct LayerView<'a> {
    raw: &'a[u8],
}

impl<'a> LayerView<'a> {

    pub fn height(&self) -> u8 {
        self.raw[0]
    }

    pub fn blockstate_id(&self) -> u16 {
        (self.raw[1] as u16) << 8 + self.raw[2] 
    }

    pub fn light(&self) -> u8 {
        self.raw[3]
    }
}

pub struct ElementView<'a> {
    raw: &'a[u8],
}

impl<'a> ElementView<'a> {

    pub fn shading(&self) -> LayerView<'a> {
        LayerView { raw: &self.raw[0..4] }
    }

    pub fn seafloor(&self) -> LayerView<'a> {
        LayerView { raw: &self.raw[5..8] }
    }

    pub fn ceil(&self) -> LayerView<'a> {
        LayerView { raw: &self.raw[9..12] }
    }

    pub fn vegetation(&self) -> LayerView<'a> {
        LayerView { raw: &self.raw[13..16] }
    }

    // pub fn _(&self) -> u8 {
    //     self.raw[16]
    // }

    pub fn biome(&self) -> u8 {
        self.raw[17]
    }
}

pub struct TileView<'a> {
    raw: &'a[u8],
}

impl<'a> TileView<'a> {

    pub fn element(&self, x: i32, z: i32) -> ElementView<'a> {
        let index = (x + z * 256) as usize;
        ElementView { raw: &self.raw[index .. index + 18] }
    }
}



pub enum InnerColor {
    None,
    Water,
    Grass,
    Foliage,
}

impl From<&str> for InnerColor {

    fn from(value: &str) -> Self {
        match value {
            "minecraft:water" => Self::Water,
            "minecraft:grass_block" => Self::Grass,
            //"minecraft:foliage" => Self::Water,
            _ => Self::None,
        }
    }
}

impl InnerColor {

    pub fn get_inner_color(&self, biome_color_gen: &BiomeColor, biome: u8, height: u8) -> [u8; 3] {
        match self {
            Self::None => [0, 0, 0],
            Self::Water => biome_color_gen.get_water(&Biome(biome as usize)),
            Self::Grass => biome_color_gen.get_grass(&Biome(biome as usize), height as i32),
            Self::Foliage => biome_color_gen.get_foliage(&Biome(biome as usize), height as i32),
        }
    }
}


pub struct BlockProps {

    pub air: bool,

    pub water: bool,

    pub waterlogged: bool,

    pub biome_color: InnerColor,
}

impl BlockProps {

    pub fn new() -> Self {
        BlockProps {
            air: true,
            water: false,
            waterlogged: false,
            biome_color: InnerColor::None,
        }
    }

    pub fn new_from<'a, I: Iterator<Item = &'a str>>(name: &'a str, state: I) -> Self {
        let mut waterlogged = false;
        for s in state {
            let mut it = s.split('=');
            if it.next() == Some("waterlogged") {
                if it.next() == Some("true") {
                    waterlogged = true;
                }
            }
        }
        BlockProps {
            air: name == "minecraft:air",
            water: name == "minecraft:water",
            waterlogged,
            biome_color: InnerColor::from(name)
        }
    }
}


pub struct Tile {

    id: (i32, i32),

    data: Vec<u8>,

    key: Vec<(Vec<Model>, BlockProps)>,

}

impl Tile {

    pub fn load<R: Read + Seek>(reader: R, id: (i32, i32), pvd: &ModelProvider) -> GEResult<Self> {
        let mut zip = ZipArchive::new(reader).map_err(Box::new)?;
        let mut data = Vec::new();
        let n = zip.by_name("data").map_err(Box::new)?.read_to_end(&mut data).map_err(Box::new)?;
        if n != 256 * 256 * 18 {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "data")))
        }
        let mut key = Vec::new();
        let mut key_string = String::new();
        let n = zip.by_name("key").map_err(Box::new)?.read_to_string(&mut key_string).map_err(Box::new)?;
        for line in key_string.lines() {
            match KeyLine::try_from(line) {
                Ok(k) => {
                    key.push((pvd.get(k.name, SplitIter::from(k.state)), BlockProps::new_from(k.name, SplitIter::from(k.state))));
                },
                Err(e) => {
                    eprintln!("parse error: `{}` @{}", line, e); //TODO: log
                    key.push((Vec::new(), BlockProps::new()))
                }
            }
        }
        Ok(Tile {
            id,
            data,
            key
        })
    }

    pub fn view<'a>(&'a self) -> TileView<'a> {
        TileView { raw: self.data.as_slice() }
    }

    pub fn get_model<'a>(&'a self, id: u16) -> &'a (Vec<Model>, BlockProps) {
        &self.key[(id - 1) as usize]
    }
}



#[derive(Debug)]
pub struct KeyLine<'a> {
    pub id: usize,
    pub name: &'a str,
    pub state: Option<&'a str>,
}


impl<'a> TryFrom<&'a str> for KeyLine<'a> {
    type Error = usize;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let mut state = None;

        let mut p = value;
        let mut pos = 0;

        let i = p.find(' ').ok_or(pos)?;
        let id = p[0..i].parse().map_err(|e| pos)?;
        p = &p[i+1..];
        pos += i + 1;

        let i = p.find('{').ok_or(pos)?;
        let s = &p[0..i];
        if s != "Block" {
            return Err(pos);
        }
        p = &p[i+1..];
        pos += i + 1;

        let i = p.find('}').ok_or(pos)?;
        let name = &p[0..i];
        p = &p[i+1..];
        pos += i + 1;

        if p.starts_with('[') {
            p = &p[1..];
            pos += 1;
            let i = p.find(']').ok_or(pos)?;
            state = Some(&p[0..i]);
        }

        Ok(KeyLine {
            id,
            name,
            state
        })
    }
}

pub struct SplitIter<'a>(Option<Split<'a, char>>);

impl<'a> From<Option<&'a str>> for SplitIter<'a> {

    fn from(value: Option<&'a str>) -> Self {
        SplitIter(value.map(|s| s.split(',')))
    }
}

impl<'a> Iterator for SplitIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(it) = &mut self.0 {
            it.next()
        } else {
            None
        }
    }
}

