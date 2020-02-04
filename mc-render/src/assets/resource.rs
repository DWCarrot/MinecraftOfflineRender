
use std::rc::Rc;
use std::cell::RefCell;
use std::io::Read;
use std::io::Seek;
use zip::read::ZipArchive;
use zip::read::ZipFile;
use zip::result::ZipResult;
use serde_json;
use image;
use image::RgbaImage;
use image::ConvertBuffer;
use super::util::Scanner;
use super::util::Provider;
use super::data_raw::ModelRaw;
use super::data_raw::BlockStateRaw;


pub struct AssetsArchive<R: Read + Seek> {
    zips: Vec<ZipArchive<R>>,
}

impl<R: Read + Seek> AssetsArchive<R> {

    pub fn new(reader: R) -> ZipResult<Self> {
        let zip = ZipArchive::new(reader)?;
        Ok(AssetsArchive{
            zips: vec![zip]
        })
    }

    pub fn new_list<I: IntoIterator<Item = R>>(list: I) -> ZipResult<Self> {
        let mut zips = Vec::new();
        for reader in list.into_iter() {
            zips.push(ZipArchive::new(reader)?);
        }
        Ok(AssetsArchive{
            zips
        })
    }

    pub fn by_name<'a>(&'a mut self, name: &str) -> ZipResult<ZipFile<'a>> {
        use zip::result::ZipError;
        let mut err = ZipError::FileNotFound;
        for zip in self.zips.iter_mut() {
            match zip.by_name(name) {
                Ok(v) => return Ok(v),
                Err(e) => {
                    err = e;
                    continue;
                }
            }
        }
        Err(err)
    }

    pub fn iter_zip_file_names<E, F: FnMut(&[&str]) -> Result<bool,E>>(&mut self, filter: Scanner, mut f: F) -> Result<bool, E> {
        for zip in self.zips.iter_mut().rev() {
            for i in 0..zip.len() {
                let zipfile = zip.by_index(i).unwrap();
                let name = zipfile.name();
                let mut args = [name;8];
                if filter.scan(name, &mut args) == filter.argc() {
                    if !f(&args)? {
                        return Ok(false);
                    }
                }
            }
        }
        Ok(true)
    }

}

/**
 * 
 */

pub struct ModelRawProvider<R: Read + Seek> {
    zip: Rc<RefCell<AssetsArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<AssetsArchive<R>>>> for ModelRawProvider<R> {

    fn from(zip: Rc<RefCell<AssetsArchive<R>>>) -> Self {
        ModelRawProvider {
            zip,
            count: 0
        }
    }
}

impl<R: Read + Seek> Provider for ModelRawProvider<R> {
    type Item = ModelRaw;

    fn provide(&mut self, name: &str) -> Option<Self::Item> {
        let full = format!("assets/minecraft/models/{}.json", name);
        match self.zip.borrow_mut().by_name(&full) {
            Ok(v) => match serde_json::from_reader(v) {
                Ok(v) => {
                    self.count += 1;
                    Some(v)
                },
                Err(e) => {
                    //TODO: log
                    return None 
                }
            },
            Err(e) => {
                //TODO: log
                return None 
            }
        }
    }
}


/**
 * 
 */

pub struct BlockStateRawProvider<R: Read + Seek> {
    zip: Rc<RefCell<AssetsArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<AssetsArchive<R>>>> for BlockStateRawProvider<R> {

    fn from(zip: Rc<RefCell<AssetsArchive<R>>>) -> Self {
        BlockStateRawProvider {
            count: 0,
            zip
        }
    }
}

impl<R: Read + Seek> Provider for BlockStateRawProvider<R> {
    type Item = BlockStateRaw;

    fn provide(&mut self, name: &str) -> Option<Self::Item> {
        let full = format!("assets/minecraft/blockstates/{}.json", name);
        match self.zip.borrow_mut().by_name(&full) {
            Ok(v) => match serde_json::from_reader(v) {
                Ok(v) => {
                    self.count += 1;
                    Some(v)
                },
                Err(e) => {
                    //TODO: log
                    None 
                }
            },
            Err(e) => {
                //TODO: log
                None 
            }
        } 
    }
}


/**
 * 
 */

pub struct TextureImageProvider<R: Read + Seek> {
    zip: Rc<RefCell<AssetsArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<AssetsArchive<R>>>> for TextureImageProvider<R> {

    fn from(zip: Rc<RefCell<AssetsArchive<R>>>) -> Self {
        TextureImageProvider {
            count: 0,
            zip
        }
    }
}

impl<'a, R: Read + Seek> Provider for TextureImageProvider<R> {
    type Item = RgbaImage;

    fn provide(&mut self, name: &str) -> Option<Self::Item> {
        let full = format!("assets/minecraft/textures/{}.mcmeta", name);
        let animated = self.zip.borrow_mut().by_name(&full).is_ok();
        let full = format!("assets/minecraft/textures/{}.png", name);
        let img = match self.zip.borrow_mut().by_name(&full) {
            Ok(v) => match image::png::PNGDecoder::new(v) {
                Ok(v) => match image::DynamicImage::from_decoder(v) {
                    Ok(d) => match d {
                        image::DynamicImage::ImageRgba8(v) => {
                            self.count += 1;
                            v
                        },
                        image::DynamicImage::ImageRgb8(v) => {
                            self.count += 1;
                            v.convert()
                        },
                        _ => {
                            //TODO: log
                            return None;
                        }
                    },
                    Err(e) => {
                        //TODO: log
                        return None;
                    }
                },
                Err(e) => {
                    //TODO: log
                    return None;
                }
            },
            Err(e) => {
                //TODO: log
                return None;
            }
        };
        if animated {
            let width = img.width();
            let mut buf = img.into_raw();
            buf.truncate(width as usize * width as usize * 4);
            println!("animated: {} ({})", name, width);
            Some(RgbaImage::from_vec(width, width, buf).unwrap())
        } else {
            Some(img)
        }
    }
}


// pub struct ZipFileIter<'a, R: Read + Seek> {
//     zip: &'a mut ZipArchive<R>,
//     filter: Scanner,
//     index: usize,
// }

// impl<'a, R: Read + Seek> Iterator for ZipFileIter<'a, R> {
//     type Item = &'a[&'a str];

//     fn next(&mut self) -> Option<Self::Item> {
//         let args = Vec::new();
//         while self.index < self.zip.len() {
//             if let Ok(zipfile) = self.zip.by_index(self.index) {
//                 let name = zipfile.name();
//                 if self.filter.scan(name, args.as_mut_slice()) == args.len() {
//                     return Some(args.as_slice())
//                 }
//             }  
//         }
//         None
//     }

// }





