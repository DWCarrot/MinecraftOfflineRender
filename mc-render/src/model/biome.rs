use image::RgbImage;
use image::ImageFormat;
use image::DynamicImage;

use crate::assets::biome::BIOME_DATA;
use crate::assets::biome::COLORMAP_GRASS;
use crate::assets::biome::COLORMAP_FOLIAGE;

#[derive(Debug, Clone)]
pub struct Biome(pub usize);


const SEA_LEVEL: i32 = 64;

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        return min
    }
    if value > max {
        return max
    }
    value
}

pub struct BiomeProps {
    temperature: f32,
    rainfall: f32,
}

impl BiomeProps {

    pub fn adjust(&self, height: i32) -> Self {
        let temperature = self.temperature - (height - SEA_LEVEL) as f32 / 600.0;
        let temperature = clamp(temperature, 0.0, 1.0);
        let rainfall = clamp(self.rainfall, 0.0, 1.0) * temperature;
        BiomeProps {
            temperature,
            rainfall
        }
    }
}

pub struct BiomeColor {

    biomes: Vec<(String, BiomeProps)>,

    water: Vec<[u8; 3]>,

    grass: RgbImage,

    foliage: RgbImage,

}

impl BiomeColor {

    pub fn new() -> Self {
        let mut biomes = Vec::with_capacity(256);
        let mut water = Vec::with_capacity(256);
        for (name, t, r, c) in BIOME_DATA.iter() {
            biomes.push((name.to_string(), BiomeProps { temperature: *t, rainfall: *r }));
            water.push([((*c >> 16) & 0xFF) as u8, ((*c >> 8) & 0xFF) as u8, ((*c >> 0) & 0xFF) as u8]);
        }
        BiomeColor {
            biomes,
            water,
            grass: BiomeColor::load_from_memory(COLORMAP_GRASS),
            foliage: BiomeColor::load_from_memory(COLORMAP_FOLIAGE),
        }
    }

    #[inline]
    fn get<'a, T>(vec: &'a Vec<T>, biome: &Biome) -> &'a T {
        if biome.0 < vec.len() {
            &vec[biome.0]
        } else {
            &vec[0]
        }
    }

    fn load_from_memory(data: &[u8]) -> RgbImage {
        match image::load_from_memory_with_format(data, image::ImageFormat::PNG).unwrap() {
            DynamicImage::ImageRgb8(img) => img,
            _ => panic!("compiling error")
        }
    }

    pub fn get_water(&self, biome: &Biome) -> [u8; 3] {
        BiomeColor::get(&self.water, biome).clone()
    }

    pub fn get_grass(&self, biome: &Biome, height: i32) -> [u8; 3] {
        let biome_color = || -> [u8; 3] {
            let BiomeProps { temperature, rainfall } = BiomeColor::get(&self.biomes, biome).1.adjust(height);
            let w = self.grass.width() as f32;
            let h = self.grass.height() as f32;
            let x = (temperature * w).round() as u32;
            let y = ((1.0 - rainfall) * h).round() as u32;
            let c = self.grass.get_pixel(x, y);
            c.0.clone()
        };
        match biome.0 {
            6 => { // Swamp
                [0x4C, 0x76, 0x3C]
            },
            29 => { // Dark Forest
                let mut c = biome_color();
                let base = [0x28, 0x34, 0x0A];
                c[0] = ((c[0] as u16 + base[0]) / 2) as u8;
                c[1] = ((c[1] as u16 + base[1]) / 2) as u8;
                c[2] = ((c[2] as u16 + base[2]) / 2) as u8;
                c
            },
            37 | 38 | 39 | 165 | 166 | 167 => { // Badlands
                [0x90, 0x81, 0x4D]
            }
            _ => {
                biome_color()
            }
        }
    }

    pub fn get_foliage(&self, biome: &Biome, height: i32) -> [u8; 3] {
        let biome_color = || -> [u8; 3] {
            let BiomeProps { temperature, rainfall } = BiomeColor::get(&self.biomes, biome).1.adjust(height);
            let w = self.grass.width() as f32;
            let h = self.grass.height() as f32;
            let x = (temperature * w).round() as u32;
            let y = ((1.0 - rainfall) * h).round() as u32;
            let c = self.foliage.get_pixel(x, y);
            c.0.clone()
        };
        match biome.0 {
            6 => { // Swamp
                [0x6A, 0x70, 0x39]
            },
            29 => { // Dark Forest
                let mut c = biome_color();
                let base = [0x28, 0x34, 0x0A];
                c[0] = ((c[0] as u16 + base[0]) / 2) as u8;
                c[1] = ((c[1] as u16 + base[1]) / 2) as u8;
                c[2] = ((c[2] as u16 + base[2]) / 2) as u8;
                c
            },
            37 | 38 | 39 | 165 | 166 | 167 => { // Badlands
                [0x9E, 0x81, 0x4D]
            }
            _ => {
                biome_color()
            }
        }
    }
}