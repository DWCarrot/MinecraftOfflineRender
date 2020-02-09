pub mod context;
pub mod mesh;
pub mod texture;


use std::io;
use std::io::Read;
use std::fs::File;
use std::path::Path;

use glium::Surface;
use glium::texture::RawImage2d;
use glium::texture::Texture2d;
use glium::texture::Texture2dArray;
use glium::program::Program;
use glium::draw_parameters::DrawParameters;
use glium::uniforms::Sampler;
use glium::uniforms::MagnifySamplerFilter;
use glium::uniforms::MinifySamplerFilter;

use image::RgbaImage;

use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Matrix4;

use crate::model::BlockRenderer;
use context::Context;
use texture::CombinedTexture;
use texture::RgbaTexture2d;
use mesh::MeshVertex;
use mesh::Mesh;
use mesh::MeshUniform;

pub type GEResult<T> = Result<T, Box<dyn std::error::Error>>;

pub struct OffScreenRenderer<'a, C: Context> {

    ctx: &'a C,

    sampled_textures: Sampler<'a, Texture2dArray>,

    sampled_light_map: Sampler<'a, Texture2d>,

    shader: Program,

    draw_params: DrawParameters<'a>,

}


impl<'a, C: Context> OffScreenRenderer<'a, C> {

    pub fn new(ctx: &'a C, textures: &'a Texture2dArray, light_map: &'a Texture2d) -> Self {
        let vert = {
            let mut s = String::new();
            open_alter(&["src/glrender/opengl-main.vert", "mc-render/src/glrender/opengl-main.vert", "opengl-main.vert"]).unwrap().read_to_string(&mut s).unwrap();
            s
        };
        let frag = {
            let mut s = String::new();
            open_alter(&["src/glrender/opengl-main.frag", "mc-render/src/glrender/opengl-main.frag","opengl-main.frag"]).unwrap().read_to_string(&mut s).unwrap();
            s
        };
        let sourcecode = glium::program::ProgramCreationInput::SourceCode {
            vertex_shader: vert.as_str(),
            tessellation_control_shader: None,
            tessellation_evaluation_shader: None,
            geometry_shader: None,
            fragment_shader: frag.as_str(),
            transform_feedback_varyings: None,
            outputs_srgb: true,
            uses_point_size: true,
        };
        let shader = Program::new(ctx.facade(), sourcecode).unwrap();
        OffScreenRenderer {
            ctx,
            sampled_textures: textures.sampled().minify_filter(MinifySamplerFilter::NearestMipmapNearest).magnify_filter(MagnifySamplerFilter::Nearest),
            sampled_light_map: light_map.sampled(),
            shader,
            draw_params: DrawParameters {
                depth: glium::Depth {
                    test: glium::DepthTest::IfMoreOrEqual,
                    write: true,
                    .. Default::default()
                },
                blend: glium::Blend::alpha_blending(),
                .. Default::default()
            }
        }
    }

    pub fn draw<'b, I>(&mut self, meshes: I, world: Matrix4<f32>, center: Vector3<i32>) -> GEResult<RgbaImage>
    where
        I: Iterator<Item=&'b Mesh<MeshVertex>> 
    {
        let mut frame = self.ctx.surface();
        frame.clear_all((0.0, 0.0, 0.0, 0.0), -1.0, 0);
        let uniforms = MeshUniform {
            world: world.into(),
            center: center.into(),
            textures: self.sampled_textures,
            light_map: self.sampled_light_map,
        };
        for mesh in meshes {
            mesh.draw(self.ctx.facade(), &mut frame, &self.shader, &uniforms, &self.draw_params)?;
        }
        frame.finish().map_err(Box::new)?;
        let raw2d: RgbaTexture2d = self.ctx.context().read_front_buffer().map_err(Box::new)?;
        Ok(raw2d.inner())
    }

}


/**
 * 
 */

pub struct MeshGenerator {

    current: usize,

    meshes: Vec<(Mesh<MeshVertex>, i32)>,
}

impl MeshGenerator {

    pub fn new() -> Self {
        MeshGenerator {
            current: 0,
            meshes: vec![(Mesh::new(), 0)],
        }
    }

    pub fn unwrap(mut self) -> Vec<Mesh<MeshVertex>> {
        self.meshes.sort_by_key(|t| t.1);
        self.meshes.into_iter().map(|t| t.0).collect()
    }
}

impl BlockRenderer for MeshGenerator {
    type Texture = CombinedTexture;
    type E = Box<dyn std::error::Error>;

    fn state(&mut self, prior: i32) -> i32 {
        let old = self.meshes[self.current].1;
        if old != prior {
            for (i, (_, id)) in self.meshes.iter().enumerate() {
                if *id == prior {
                    self.current = i;
                    return old;
                }
            }
            let mesh = Mesh::new();
            self.meshes.push((mesh, prior));
            self.current = self.meshes.len() - 1;
            return old;
        }
        return prior;
    }

    fn draw(
        &mut self, 
        loc: Vector3<i32>, 
        vp0: Vector3<f32>, vp1: Vector3<f32>, vp2: Vector3<f32>, vp3: Vector3<f32>, 
        vt0: Vector2<f32>, vt1: Vector2<f32>, vt2: Vector2<f32>, vt3: Vector2<f32>,
        tex: Self::Texture, 
        color: [u8; 4], 
        light: u8
    ) -> Result<(), Self::E> 
    {
        let mesh = &mut self.meshes[self.current].0;
        let vertexs = [
            MeshVertex { loc: loc.into(), pos: vp0.into(), tex: vt0.into(), tex_id: tex.0, color: color, light: light as u32 },
            MeshVertex { loc: loc.into(), pos: vp1.into(), tex: vt1.into(), tex_id: tex.0, color: color, light: light as u32 },
            MeshVertex { loc: loc.into(), pos: vp2.into(), tex: vt2.into(), tex_id: tex.0, color: color, light: light as u32 },
            MeshVertex { loc: loc.into(), pos: vp3.into(), tex: vt3.into(), tex_id: tex.0, color: color, light: light as u32 },    
        ];
        mesh.append(&vertexs);
        Ok(())
    }

}




pub fn default_lmmp<'a>(light: bool) -> RawImage2d<'a, u8> {
    let w = 16;
    let h = 16;
    let mut data = vec![255u8; w * h * 3];
    if light {
        for x in 0..w {
            for y in 0..h {
                let i = (y * w + x) * 3;
                let g = std::cmp::max(x * 255 / w, y * 255 / h) as u8;
                data[i + 0] = g;
                data[i + 1] = g;
                data[i + 2] = g;
            }
        }
    }
    RawImage2d::from_raw_rgb(data, (w as u32, h as u32))
}


pub fn open_alter<P: AsRef<Path>>(paths: &[P]) -> io::Result<File> {
    for path in paths {
        match File::open(path) {
            Ok(ifile) => { return Ok(ifile); },
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => { continue; }
                _ => { return Err(e); }
            }
        };
    }
    Err(io::Error::from(io::ErrorKind::NotFound))
}
