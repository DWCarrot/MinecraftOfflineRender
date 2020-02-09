use std::io::Read;
use std::io::Seek;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
use std::rc::Rc;
use std::cell::RefCell;

use cgmath::Vector3;
use cgmath::Matrix4;

use glium::texture::MipmapsOption;

use mc_render::assets::data_type::Face;
use mc_render::assets::resource::AssetsArchive;
use mc_render::assets::resource::BlockStateRawProvider;
use mc_render::assets::resource::ModelRawProvider;
use mc_render::assets::resource::TextureImageProvider;
use mc_render::model;
use mc_render::model::block::RenderableBlock;
use mc_render::model::block::World;
use mc_render::model::biome::BiomeColor;
use mc_render::glrender::MeshGenerator;
use mc_render::glrender::context::Context;
use mc_render::glrender::context::WindowHideContext;
use mc_render::glrender;
use mc_render::glrender::texture::CombinedTextureGen;
use mc_render::glrender::OffScreenRenderer;

use crate::loader::*;


pub struct TileWorld<'a> {
    water_models: Vec<Model>,
    air_props: BlockProps,
    tile: Tile,
    biome_color_gen: &'a BiomeColor,
}

impl<'a> TileWorld<'a> {

    pub fn new<R: Read + Seek>(reader: R, id: (i32, i32), pvd: &ModelProvider, biome_color_gen: &'a BiomeColor) -> Self {
        TileWorld {
            water_models: pvd.get("minecraft:water", SplitIter::from(None)),
            air_props: BlockProps::new(),
            tile: Tile::load(reader, id, pvd).unwrap(),
            biome_color_gen
        }
    }

    pub fn draw(&'a self) -> MeshGenerator {
        let faces = [Face::Up];
        let mut r = MeshGenerator::new();
        let view = self.tile.view();
        for x in 0 .. 256 {
            for z in 0 .. 256 {
                let element = view.element(x, z);
                let block = element.seafloor();
                if block.blockstate_id() != 0 {
                    let loc = Vector3::new(x, block.height() as i32, z);
                    model::draw(&faces, &loc, &mut r, self).unwrap();
                }
                let block = element.ceil();
                if block.blockstate_id() != 0 {
                    let loc = Vector3::new(x, block.height() as i32, z);
                    model::draw(&faces, &loc, &mut r, self).unwrap();
                }
                let block = element.vegetation();
                if block.blockstate_id() != 0 {
                    let loc = Vector3::new(x, block.height() as i32, z);
                    model::draw(&faces, &loc, &mut r, self).unwrap();
                }
                let block = element.shading();
                if block.blockstate_id() != 0 {
                    let loc = Vector3::new(x, block.height() as i32, z);
                    model::draw(&faces, &loc, &mut r, self).unwrap();
                }
            }
        }
        r
    }

    fn gen(&'a self, block: LayerView<'a>, element: ElementView<'a>) -> TileBlock<'a> {
        let (model, props) = self.tile.get_model(block.blockstate_id());
        TileBlock {
            model: model.as_slice(),
            water: self.water_models.as_slice(),
            props,
            light: block.light(),
            color: props.biome_color.get_inner_color(self.biome_color_gen, element.biome(), block.height()),
        }
    }

    fn air(&'a self) -> TileBlock<'a> {
        TileBlock {
            model: &self.water_models[0..0],
            water: &self.water_models[..],
            props: &self.air_props,
            light: 0,
            color: [0, 0, 0]
        }
    }
}

impl<'a> World<'a> for TileWorld<'a> {
    type Block = TileBlock<'a>;

    fn get(&'a self, loc: &Vector3<i32>) -> Self::Block {
        if !(loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x > 255 || loc.y > 255 || loc.z > 255) {
            let element = self.tile.view().element(loc.x, loc.z);
            let block = element.ceil();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.seafloor();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.shading();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
            let block = element.vegetation();
            if block.height() as i32 == loc.y {
                return self.gen(block, element);
            }
        }
        self.air()
    }
    
    fn is_air(&self, loc: &Vector3<i32>) -> bool {
        if loc.x < 0 || loc.y < 0 || loc.z < 0 || loc.x > 255 || loc.y > 255 || loc.z > 255 {
            let element = self.tile.view().element(loc.x, loc.z);
            let block = element.ceil();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.seafloor();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.shading();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
            let block = element.vegetation();
            if block.height() as i32 == loc.y && block.blockstate_id() != 0 {
                return false;
            }
        }
        true
    }

}


pub struct TileBlock<'a> {
    model: &'a [Model],
    water: &'a [Model],
    props: &'a BlockProps,
    light: u8,
    color: [u8; 3]
}


impl<'a> RenderableBlock<'a> for TileBlock<'a> {
    type Model = Model;

    fn is_air(&self) -> bool {
        self.props.air
    }

    fn is_water(&self) -> bool {
        self.props.water
    }

    fn is_water_logged(&self) -> bool {
        self.props.waterlogged
    }

    fn get_models(&self) -> std::slice::Iter<'a, Self::Model> {
        self.model.iter()
    }

    fn get_water_models(&self) -> std::slice::Iter<'a, Self::Model> {
        self.water.iter()
    }

    fn get_inline_color(&self, tintindex: usize) -> [u8; 4] {
        let _ = tintindex;
        [self.color[0], self.color[1], self.color[2], 255]
    }

    fn get_light(&self) -> u8 {
        self.light
    }
}

pub struct AppOptions {
    pub width: u32,
    pub height: u32,
    pub tex_width: u32,
    pub tex_height: u32,
    pub assets: Vec<String>,
    pub cache_folder: String,
    pub output_folder: String,
    pub world: Matrix4<f32>,
    pub center: Vector3<i32>,
    pub night_mod: bool,
}

impl Default for AppOptions {

    fn default() -> Self {
        AppOptions {
            width: 256,
            height: 256,
            tex_width: 16,
            tex_height: 16,
            assets: Vec::new(),
            cache_folder: String::from("."),
            output_folder: String::from("../image"),
            world: Matrix4::from_angle_y(cgmath::Deg(90.0)) * Matrix4::from_scale(1.0 / 128.0),
            center: Vector3::new(128, 128, 128),
            night_mod: false,
        }
    }
}

pub fn wrap_assets(assets: Vec<String>) -> Vec<File> {
    let mut res = Vec::new();
    for s in assets {
        match File::open(s.as_str()) {
            Ok(ifile) => {
                res.push(ifile);
            },
            Err(e) => {
                eprintln!("file [{}]: {}", s.as_str(), e);
            }
        }
    }
    res
}



pub fn app(options: AppOptions) -> GEResult<()> {

    if let Err(e) = fs::create_dir_all(options.output_folder.as_str()) {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            return Err(Box::new(e));
        }
    }

    let assets = wrap_assets(options.assets);
    let mut ctx = WindowHideContext::build(options.width, options.height, glium::glutin::GlRequest::Specific(glium::glutin::Api::OpenGl, (3, 3)));
    let assets = Rc::new(RefCell::new(AssetsArchive::from_list(assets.into_iter()).map_err(Box::new)?));
    let list = assets.borrow_mut().find_blockstates();
    let mut bs_pvd = BlockStateRawProvider::from(assets.clone());
    let mut mdl_pvd = ModelRawProvider::from(assets.clone());
    let tex_pvd = TextureImageProvider::from(assets.clone());
    let mut tex_gen = CombinedTextureGen::new(ctx.facade(), tex_pvd);
    let mut modelpvd = ModelProvider::new();
    modelpvd.build("minecraft", list.into_iter(), &mut bs_pvd, &mut mdl_pvd, &mut tex_gen);
    let textures = tex_gen.build(options.tex_width, options.tex_height, MipmapsOption::AutoGeneratedMipmaps)?;
    let light_map = glium::texture::Texture2d::new(ctx.facade(), glrender::default_lmmp(options.night_mod)).unwrap();
    let mut renderer = OffScreenRenderer::new(&ctx, &textures, &light_map);
    let biome_color_gen = BiomeColor::new();

    for path in fs::read_dir(options.cache_folder.as_str()).map_err(Box::new)? {
        let path = path.map_err(Box::new)?.path();
        if let Some(id) = parse_file_name(&path) {
            println!("path {}", path.display());
            let world = TileWorld::new(File::open(&path).map_err(Box::new)?, id, &modelpvd, &biome_color_gen);
            let mesh = world.draw();
            let img = renderer.draw(mesh.unwrap().iter(), options.world, options.center)?;
            let mut path = PathBuf::from(options.output_folder.as_str());
            path.push(format!("{},{}.png", id.0, id.1));
            if let Err(e) = img.save_with_format(&path, image::ImageFormat::PNG) {
                eprintln!("{}", e);
            }
        }      
    }

    ctx.wait();

    Ok(())
}


pub fn parse_file_name<P: AsRef<Path>>(path: P) -> Option<(i32, i32)> {
    const EXT: &'static str = ".zip";
    let file_name = path.as_ref().file_name()?.to_str()?;
    if file_name.ends_with(EXT) {
        let mut s = file_name[0 .. file_name.len() - EXT.len()].split(',');
        let x = s.next()?.parse().ok()?;
        let z = s.next()?.parse().ok()?;
        Some((x, z))
    } else {
        None
    }
}