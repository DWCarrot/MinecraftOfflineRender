

use std::borrow::Cow;
use std::collections::hash_map::HashMap;

use glium::texture::Texture2d;
use glium::texture::Texture2dArray;
use glium::texture::Texture2dDataSink;
use glium::texture::Texture2dDataSource;
use glium::texture::RawImage2d;
use glium::texture::MipmapsOption;
use glium::backend::Facade;
use glium::Surface;
use glium::framebuffer::SimpleFrameBuffer;
use glium::uniforms::MagnifySamplerFilter;

use image::RgbaImage;

use crate::assets::util::Provider;
use crate::model::model::TextureGen;

pub type GEResult<T> = Result<T, Box<dyn std::error::Error>>;


#[derive(Clone, Debug)]
pub struct CombinedTexture(pub i32);

pub struct CombinedTextureGen<'a, F: Facade, P: Provider<Item=RgbaImage>> {

    cache: HashMap<String, (CombinedTexture, Texture2d)>,

    provider: P,

    facade: &'a F,
}

impl<'a, F: Facade, P: Provider<Item=RgbaImage>> TextureGen for CombinedTextureGen<'a, F, P> {
    type Texture = CombinedTexture;

    fn get(&mut self, name: &str) -> Self::Texture {
        use std::collections::hash_map::Entry;
        let len = self.cache.len();
        match self.cache.entry(name.to_string()) {
            Entry::Occupied(entry) => entry.into_mut().0.clone(),
            Entry::Vacant(entry) => {
                match self.provider.provide(name) {
                    Some(image) => {
                        let id = len as i32 + 1;
                        match Texture2d::new(self.facade, RgbaTexture2d(image)) {
                            Ok(tex2d) => { 
                                let tex = CombinedTexture(id);
                                entry.insert((tex, tex2d)).0.clone()
                            },
                            Err(e) => {
                                eprintln!("@[{}] {}", name, e); //TODO: log
                                CombinedTexture(0)
                            }
                        }
                    },
                    None => {
                        eprintln!("texture not found: {}", name); //TODO: log
                        CombinedTexture(0)
                    }
                }         
            }
        }
    }

}

impl<'a, F: Facade, P: Provider<Item=RgbaImage>> CombinedTextureGen<'a, F, P> { 

    pub fn new(facade: &'a F, provider: P) -> Self {
        CombinedTextureGen {
            cache: HashMap::new(),
            provider,
            facade,
        }
    }

    pub fn build(self, width: u32, height: u32, mipmaps: MipmapsOption) -> GEResult<Texture2dArray> {
        let sz = self.cache.len() as u32 + 1 ;
        let tex2d_arr = Texture2dArray::empty_with_mipmaps(self.facade, mipmaps, width, height, sz).map_err(Box::new)?;
        for (k, v) in self.cache {
            let id = (v.0).0;
            let tex2d = v.1;
            let fb = match SimpleFrameBuffer::new(self.facade, tex2d_arr.layer(id as u32).unwrap().main_level()) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("@ [{}] {}", id, e); //TODO log
                    continue;
                }
            };
            tex2d.as_surface().fill(&fb, MagnifySamplerFilter::Nearest);
        }
        Ok(tex2d_arr)
    }
}



pub struct RgbaTexture2d(RgbaImage);

impl RgbaTexture2d {

    pub fn inner(self) -> RgbaImage {
        self.0
    }
}

impl Texture2dDataSink<(u8, u8, u8, u8)> for RgbaTexture2d {

    fn from_raw(data: Cow<[(u8, u8, u8, u8)]>, width: u32, height: u32) -> Self {
        let w = width as usize;
        let h = height as usize;
        let mut buf = Vec::with_capacity(w * h * 4);
        let data = data.as_ref();
        unsafe {
            let line_width = w * 4;
            let p: *const u8 = &data[0].0;
            let p = std::slice::from_raw_parts(p, line_width * h);
            for y in 0 .. h {
                let line = &p[(h - 1 - y) * line_width .. (h - y) * line_width];
                buf.extend_from_slice(line);
            }
        }
        RgbaTexture2d(RgbaImage::from_raw(width, height, buf).unwrap())
    }
}

impl<'a> Texture2dDataSource<'a> for RgbaTexture2d {
    type Data = u8;

    fn into_raw(self) -> RawImage2d<'a, Self::Data> {
        let image = self.inner();
        let dim = image.dimensions();
        RawImage2d::from_raw_rgba_reversed(image.into_raw().as_ref(), dim)
    }
}