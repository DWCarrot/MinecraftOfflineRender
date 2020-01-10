
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


pub struct JarArchive<R: Read + Seek> {
    zip: ZipArchive<R>,
}

impl<R: Read + Seek> JarArchive<R> {

    pub fn new(reader: R) -> ZipResult<Self> {
        let zip = ZipArchive::new(reader)?;
        Ok(JarArchive{
            zip
        })
    }

    pub fn get_file<'a>(&'a mut self, name: &str) -> ZipResult<ZipFile<'a>> {
        self.zip.by_name(name)
    }

    pub fn iter_zip_file_names<E, F: FnMut(&[&str]) -> Result<bool,E>>(&mut self, filter: Scanner, mut f: F) -> Result<bool, E> {
        for i in 0..self.zip.len() {
            let zipfile = self.zip.by_index(i).unwrap();
            let name = zipfile.name();
            let mut args = [name;8];
            if filter.scan(name, &mut args) == filter.argc() {
                if !f(&args)? {
                    return Ok(false);
                }
            }
        }
        Ok(true)
    }

    pub fn unwrap(self) -> ZipArchive<R> {
        self.zip
    }

}


/**
 * 
 */

pub struct ModelRawProvider<R: Read + Seek> {
    zip: Rc<RefCell<ZipArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<ZipArchive<R>>>> for ModelRawProvider<R> {

    fn from(zip: Rc<RefCell<ZipArchive<R>>>) -> Self {
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
    zip: Rc<RefCell<ZipArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<ZipArchive<R>>>> for BlockStateRawProvider<R> {

    fn from(zip: Rc<RefCell<ZipArchive<R>>>) -> Self {
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
    zip: Rc<RefCell<ZipArchive<R>>>,
    pub count: usize
}

impl<R: Read + Seek> From<Rc<RefCell<ZipArchive<R>>>> for TextureImageProvider<R> {

    fn from(zip: Rc<RefCell<ZipArchive<R>>>) -> Self {
        TextureImageProvider {
            count: 0,
            zip
        }
    }
}

impl<'a, R: Read + Seek> Provider for TextureImageProvider<R> {
    type Item = RgbaImage;

    fn provide(&mut self, name: &str) -> Option<Self::Item> {
        let full = format!("assets/minecraft/textures/{}.png", name);
        match self.zip.borrow_mut().by_name(&full) {
            Ok(v) => match image::png::PNGDecoder::new(v) {
                Ok(v) => match image::DynamicImage::from_decoder(v) {
                    Ok(d) => match d {
                        image::DynamicImage::ImageRgba8(v) => {
                            self.count += 1;
                            Some(v)
                        },
                        image::DynamicImage::ImageRgb8(v) => {
                            self.count += 1;
                            Some(v.convert())
                        },
                        _ => {
                            //TODO: log
                            return None
                        }
                    },
                    Err(e) => {
                        //TODO: log
                        return None 
                    }
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





