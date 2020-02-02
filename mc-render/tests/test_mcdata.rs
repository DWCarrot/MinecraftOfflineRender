use std::io::{Read, Write, Seek};
use std::collections::HashMap;
use serde_json::Value;
use mc_render::assets::{resource, util};
use mc_render::assets::util::Provider;
use mc_render::assets::data_raw::BlockStateRaw;
use mc_render::assets::resource::{TextureImageProvider, BlockStateRawProvider, ModelRawProvider};
use mc_render::model::model::{TextureGen, BlockModelBuilder};

#[test]
fn test_blockstate_parse() {
    
    let ifile = std::fs::File::open("tests/redstone_wire.json").unwrap();
    let data: BlockStateRaw = serde_json::from_reader(ifile).unwrap();
    let data = if let BlockStateRaw::MultiPart(v) = data { v } else { panic!("what?") };
    let mut l = 1;
    for tuple in data.into_iter() {
        let mut expr = tuple.0;
        println!("========================================");
        let mut s = 0;
        loop  {
            let it: Vec<String> = expr.line_iter().map(|s| s.clone()).collect();
            println!("{:?}", it);
            s += 1;
            if !expr.update_iter() {
                break;
            }
        }
        println!("{}", s);
        l *= (s + 1);
    }
    println!("#end {}", l);
}

// #[test]
// fn test_resource_parse() {
//     let s = std::env::var("ASSETS").expect("$env::ASSETS is not ref to a minecraft version.jar");
//     let ifile = std::fs::File::open(s).unwrap();
//     let mut a = resource::JarOriginAssets::new(ifile).unwrap();
//     let v = resource::integrate_model(&mut a, "block/dirt").unwrap();
//     println!("{}", v);
//     let v = resource::integrate_model(&mut a, "block/lava").unwrap();
//     println!("{}", v);
// }

#[test]
fn test_blockstate_find() {
    let s = std::env::var("ASSETS").expect("$env::ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();
    let mut a = resource::JarArchive::new(ifile).unwrap();
    let mut list = HashMap::new();
    a.iter_zip_file_names(
        util::Scanner::new("assets/minecraft/blockstates/{}.json"), 
        |args: &[&str]| -> std::io::Result<bool> {
            list.insert(args[0].to_string(), None);
            Ok(true)
        }
    ).expect("what: ");

    let zip = std::rc::Rc::new(std::cell::RefCell::new(a.unwrap()));
    let mut mdl_pvd = ModelRawProvider::from(zip.clone());
    let mut bs_pvd = BlockStateRawProvider::from(zip.clone());
    let mut tex_pvd = TextureImageProvider::from(zip.clone());
    let mut tex_gen = SimpleTexGen{ p: &mut tex_pvd, table: HashMap::new() };

    let mut b = BlockModelBuilder::new(&mut bs_pvd, &mut mdl_pvd, &mut tex_gen);

    for (key, val) in &mut list {
        let expr = match b.build(key.as_str()) {
            Ok(v) => v,
            Err(e) => {
                println!("{}: {:?}", key, e); 
                continue;
            }
        };
        *val = Some(expr);
    }

    println!("load block: {}", bs_pvd.count);
    println!("load model: {}", mdl_pvd.count);
    println!("load texture: {}", tex_pvd.count);

    //list.insert(String::from("#"), Some(3));
    let mut ofile = std::fs::File::create("../target/test.log").unwrap();
    for (k, v) in list {
        writeln!(ofile, "{}\n\t{:?}\n", k, v).unwrap();
    }

    println!("logged");
}

struct SimpleTexGen<'a, R: Read + Seek> {
    p: &'a mut TextureImageProvider<R>,
    table: HashMap<String, (usize, u32, u32)>,
}

impl<'a, R: Read + Seek> TextureGen for SimpleTexGen<'a, R> {
    type Texture = (usize, u32, u32);

    fn get(&mut self, name: &str) -> Self::Texture { 
        let c = self.table.len() + 1;
        let p = &mut self.p;
        self.table.entry(name.to_owned()).or_insert_with(|| -> (usize, u32, u32) {
            if let Some(img) = p.provide(name) {
                (c, img.width(), img.height())
            } else {
                dbg!(name);
                (0, 0, 0)
            }
        }).clone()
    }
}

