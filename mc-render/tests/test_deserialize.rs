use serde_json;

use mc_render::assets::resource;
use mc_render::assets::util;

#[test]
fn test_raw() {
    use mc_render::assets::data_raw::*;
    use std::fs::File;
    for s in &["tripwire_hook", "anvil", "beehive","torch"] {
        println!("------------ [ {} ] --------------", s);
        let ifile = File::open(format!("tests/{}.json", s)).unwrap();
        let model: ModelRaw = serde_json::from_reader(ifile).unwrap();
        println!("{:?}",&model);
        assert!(true);
    }
    
}

#[test]
fn test_merge_raw() {
    use mc_render::assets::data_raw::*;
    let s = std::env::var("ASSETS").expect("$env:ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();
    let mut a = resource::AssetsArchive::new(ifile).unwrap();
    let mut list = Vec::new();
    a.iter_zip_file_names(
        util::Scanner::new("assets/minecraft/models/block/{}.json"), 
        |args: &[&str]| -> std::io::Result<bool> {
            list.push("block/".to_string() + args[0]);
            //println!("{}: {}", list.len(), list.last().unwrap());
            Ok(true)
        }
    ).expect("what: ");
    let mut lk: std::collections::HashMap<String, ModelRaw> = std::collections::HashMap::new();
    let mut c = 0 as usize;
    for name in list.into_iter() {
        
        
        let full = format!("assets/minecraft/models/{}.json", name);
        let ifile = a.by_name(&full).unwrap();
        match serde_json::from_reader(ifile) {
            Ok(v) => {
                lk.insert(name, v);
            },
            Err(e) => {
                c += 1;
                eprintln!("err @ \"{}\" {}",name, e);
            }
        };      
    }

    struct M(std::collections::HashMap<String, ModelRaw>);

    use util::Provider;

    impl Provider for M {
        type Item=ModelRaw;

        fn provide(&mut self, name: &str) -> Option<Self::Item> {
            self.0.get(name).map(Clone::clone)
        }
    }

   

    let mut p = M(lk);
    for s in &["tripwire_hook", "anvil", "beehive","torch"] {
        println!("------------ [ {} ] --------------", s);
        let name = format!("block/{}", s);
        
        let mut model = p.provide(&name).unwrap().clone();
        //model.integrate(&mut p).expect("int");
        
        let s = serde_json::to_string(&model).expect("???");
        println!("{}",s);
        assert!(true);
    }  
}

#[test]
fn test_blockstate_one() {
    use mc_render::assets::data_raw::*;
    use std::fs::File;
    use std::io::Write;

    let s = std::env::var("ASSETS").expect("$env:ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();
    let mut a = resource::AssetsArchive::new(ifile).unwrap();

    for name in &["oak_fence"] {
        let full = format!("assets/minecraft/blockstates/{}.json", name);
        let ifile = a.by_name(&full).expect(&format!("?:{}", name));
        match serde_json::from_reader(ifile) {
            Ok(v) => {
                let k: BlockStateRaw = v;
                
                println!("{:?}", k);
            },
            Err(e) => {
                
                eprintln!("err @ \"{}\" {}",name, e);
            }
        };
    }
}

#[test]
fn test_blockstate_raw() {

    use mc_render::assets::data_raw::*;
    use std::fs::File;
    use std::io::Write;

    let s = std::env::var("ASSETS").expect("$env:ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();
    let mut a = resource::AssetsArchive::new(ifile).unwrap();
    let mut list = Vec::new();
    a.iter_zip_file_names(
        util::Scanner::new("assets/minecraft/blockstates/{}.json"), 
        |args: &[&str]| -> std::io::Result<bool> {
            list.push(args[0].to_owned());
            //println!("{}: {}", list.len(), list.last().unwrap());
            Ok(true)
        }
    ).expect("what: ");

    let mut c = 0;
    println!("wait 1");
    std::thread::sleep(std::time::Duration::from_secs(2));
    println!("continue");
    let mut map = std::collections::HashMap::new();
    for name in list.into_iter() {
        let full = format!("assets/minecraft/blockstates/{}.json", name);
        let ifile = a.by_name(&full).expect(&format!("?:{}", name));
        match serde_json::from_reader(ifile) {
            Ok(v) => {
                let k: BlockStateRaw = v;

                //let mut ofile = std::fs::File::create(format!("../target/tests/cache/{}.json", name)).unwrap();
                //serde_json::to_writer(ofile, &k);
                map.insert(name.to_string(), k);

            },
            Err(e) => {
                c += 1;
                eprintln!("err @ \"{}\" {}",name, e);
            }
        };
    }

    println!("end============");

    println!("{}", map.len());
    std::thread::sleep(std::time::Duration::from_secs(3));
    let mut ofile = std::fs::File::create("../target/test.json").unwrap();
    serde_json::to_writer(ofile, &map).expect("?");

    
}

#[test]
fn test_full() {
    use mc_render::assets::data_raw::*;
    use util::Provider;
    use std::collections::HashMap;
    use std::io::Read;
    use std::io::Seek;

    let s = std::env::var("ASSETS").expect("$env:ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();
    println!("opening...");
    let mut a = resource::AssetsArchive::new(ifile).unwrap();
    println!("listing...");
    let mut list = Vec::new();
    a.iter_zip_file_names(
        util::Scanner::new("assets/minecraft/blockstates/{}.json"), 
        |args: &[&str]| -> std::io::Result<bool> {
            list.push(args[0].to_owned());
            //println!("{}: {}", list.len(), list.last().unwrap());
            Ok(true)
        }
    ).expect("what: ");
    println!("loading blocks...");
    let mut c = 0;
    let mut map = HashMap::new();
    for name in list.into_iter() {
        let full = format!("assets/minecraft/blockstates/{}.json", name);
        let ifile = a.by_name(&full).expect(&format!("?:{}", name));
        match serde_json::from_reader(ifile) {
            Ok(v) => {
                let k: BlockStateRaw = v;
                map.insert(name.to_string(), k);
            },
            Err(e) => {
                c += 1;
                eprintln!("err @ \"{}\" {}",name, e);
            }
        };
    }

    struct ModelProvider<'a, R: Read + Seek> {
        rsc: &'a mut resource::AssetsArchive<R>,
        cache: HashMap<String, ModelRaw>,
    };

    impl<'a, R: Read + Seek> Provider for ModelProvider<'a, R> {
        type Item=ModelRaw;

        fn provide(&mut self, name: &str) -> Option<Self::Item> {
            use std::collections::hash_map::Entry;
            match self.cache.entry(name.to_string()) {
                Entry::Occupied(e) => Some(e.get().clone()),
                Entry::Vacant(e) => {
                    let full = format!("assets/minecraft/models/{}.json", name);
                    let ifile = self.rsc.by_name(&full).unwrap();
                    match serde_json::from_reader::<_, ModelRaw>(ifile) {
                        Ok(v) => {
                            e.insert(v.clone());
                            Some(v)
                        },
                        Err(err) => {
                            eprintln!("err @ \"{}\" {}",name, err);
                            None
                        }
                    }
                }
            }
        }
    }
}