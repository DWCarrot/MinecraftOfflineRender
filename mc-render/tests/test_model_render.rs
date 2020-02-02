use std::io::{Read, Write, Seek};
use std::collections::HashMap;
use std::rc::Rc;

use glium::glutin;

use cgmath::{Vector3, Matrix4, Vector4, Deg, SquareMatrix};

use mc_render::assets::data_type::{Face};
use mc_render::assets::util::Provider;
use mc_render::assets::resource::{TextureImageProvider, BlockStateRawProvider, ModelRawProvider};
use mc_render::model::model::{KVPair, TextureGen, BlockModelBuilder, TransformedModel};
use mc_render::model::block::{World, RenderableBlock};
use mc_render::model::blockstate::{BlockState};
use mc_render::glrender;
use mc_render::glrender::{MeshGenerator};
use mc_render::glrender::context;
use mc_render::glrender::texture::{CombinedTextureGen, CombinedTexture};

#[test]
fn test_model_render() {
    let s = std::env::var("ASSETS").expect("$env::ASSETS is not ref to a minecraft version.jar");
    let ifile = std::fs::File::open(s).unwrap();

    use context::Context;
    let ctx = context::WindowHideContext::build(256, 256, glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)));


    let zip = std::rc::Rc::new(std::cell::RefCell::new(zip::ZipArchive::new(ifile).unwrap()));
    let mut mdl_pvd = ModelRawProvider::from(zip.clone());
    let mut bs_pvd = BlockStateRawProvider::from(zip.clone());
    let tex_pvd = TextureImageProvider::from(zip.clone());
    let mut tex_gen = CombinedTextureGen::new(ctx.facade(), tex_pvd);

    let mut b = BlockModelBuilder::new(&mut bs_pvd, &mut mdl_pvd, &mut tex_gen);

    let model_list = {
        let mut model_list = std::collections::HashMap::new();
        for name in &["acacia_fence", "crafting_table", "magenta_glazed_terracotta"] {
            let expr = match b.build(name) {
                Ok(v) => v,
                Err(e) => {
                    println!("{}: {:?}", name, e); 
                    continue;
                }
            };
            model_list.insert(name.to_string(), expr);
        }
        model_list
    };
    let tex = tex_gen.build(16, 16, glium::texture::MipmapsOption::AutoGeneratedMipmaps).unwrap();
    
    macro_rules! str {
        ($s: expr) => {
            String::from($s)
        };
    }

    let world = SimpleWorld {
        data: model_list,
        pre: std::cell::RefCell::new( vec![
            
            (
                str!("magenta_glazed_terracotta"),
                vec![
                    str!("facing=south"),
                ]
            ),
            (
                str!("magenta_glazed_terracotta"),
                vec![
                    str!("facing=west"),
                ]
            ),
            (
                str!("magenta_glazed_terracotta"),
                vec![
                    str!("facing=north"),
                ]
            ),
            (
                str!("magenta_glazed_terracotta"),
                vec![
                    str!("facing=east"),
                ]
            ),
            (
                str!("acacia_fence"),
                vec![
                    // str!("south=true"),
                    str!("north=true"),
                    str!("west=true"),
                    // str!("east=true"),
                ]
            ),
            
        ].into_iter())
    };
    let mut mp = MeshGenerator::new();
    let faces = [Face::Up, Face::East, Face::South];
    mc_render::model::draw(&faces, &Vector3::new(-3, 0, 0), &mut mp, &world).unwrap();
    mc_render::model::draw(&faces, &Vector3::new(-1, 0, 0), &mut mp, &world).unwrap();
    mc_render::model::draw(&faces, &Vector3::new(1, 0, 0), &mut mp, &world).unwrap();
    mc_render::model::draw(&faces, &Vector3::new(3, 0, 0), &mut mp, &world).unwrap();
    mc_render::model::draw(&faces, &Vector3::new(0, 0, 3), &mut mp, &world).unwrap();
    
    
    let lmmp = glium::texture::Texture2d::new(ctx.facade(), glrender::default_lmmp()).unwrap();
    let mut r = glrender::OffScreenRenderer::new(&ctx, &tex, &lmmp);
    let s = mp.unwrap();
    let world = 
        Matrix4::from_diagonal(Vector4::new(0.25, 0.25, 0.25, 1.0)) *
        Matrix4::from_angle_y(Deg(-30.0)) *
        Matrix4::from_angle_x(Deg(30.0));
    // let world = 
    //     Matrix4::from_diagonal(Vector4::new(0.25, 0.25, 0.25, 1.0)) *
    //     Matrix4::from_angle_y(Deg(0.0)) *
    //     Matrix4::from_angle_x(Deg(90.0));


    let center = Vector3::new(0, 0, 0);

    r.draw(s.iter(), world, center).unwrap().save("../target/glium-test_texture_2.png").unwrap();
    //println!("{:?}", &model_list);
    //println!("{:?}", &tex);
}


struct SimpleBlock {
    is_air: bool,
    is_water: bool,
    is_water_logged: bool,
    model: Vec<Rc<TransformedModel<CombinedTexture>>>,
}

impl RenderableBlock for SimpleBlock {
    type Model = Rc<TransformedModel<CombinedTexture>>;

    fn is_air(&self) -> bool {
        self.is_air
    }

    fn is_water(&self) -> bool {
        self.is_water
    }

    fn is_water_logged(&self) -> bool {
        self.is_water_logged
    }

    fn get_models<'a>(&'a self) -> std::slice::Iter<'a, Self::Model> {
        self.model.iter()
    }

    fn get_water_models<'a>(&'a self) -> std::slice::Iter<'a, Self::Model> {
        self.model[0..0].iter()
    }

    fn get_inline_color(&self, tintindex: usize) -> [u8; 4] {
        let _ = tintindex;
        [255, 255, 255, 255]
    }

    fn get_light(&self) -> u8 {
        0xFF
    }

}


struct SimpleWorld {
    pre: std::cell::RefCell<std::vec::IntoIter<(String, Vec<String>)>>,
    data: HashMap<String, BlockState<String, Rc<TransformedModel<CombinedTexture>>>>
}

impl World for SimpleWorld {
    type Block = SimpleBlock;

    fn get(&self, loc: &Vector3<i32>) -> Self::Block {
        //let i = (loc.x * 13 + loc.y * 7 + loc.z * 3) as usize % self.pre.len();
        let (ref name, ref pair) = self.pre.borrow_mut().next().unwrap();
        let expr = self.data.get(name).unwrap();
        let mut ofile = std::fs::File::create("../target/test.log").unwrap();
        writeln!(ofile, "{:?}", expr).unwrap();
        let models = expr.get(pair.iter());
        println!("{:?} {}{:?} {}", loc, name, pair, models.len());
        writeln!(ofile, "{:?}", models).unwrap();
        SimpleBlock {
            is_air: false,
            is_water: false,
            is_water_logged: false,
            model: models.clone(),
        }
    }
    
    fn is_air(&self, loc: &Vector3<i32>) -> bool {
        true
    }

}