use glium::vertex::Vertex;
use glium::vertex::AttributeType;
use glium::vertex::VertexFormat;
use glium::uniforms::Uniforms;
use glium::uniforms::UniformValue;
use glium::uniforms::AsUniformValue;
use glium::uniforms::Sampler;
use glium::backend::Facade;
use glium::program::Program;
use glium::draw_parameters::DrawParameters;
use glium::Frame;
use glium::Surface;
use glium::texture::Texture2d;
use glium::texture::Texture2dArray;


pub type GEResult<T> = Result<T, Box<dyn std::error::Error>>;


#[derive(Clone, Copy)]
pub struct MeshVertex {

    pub loc: [i32; 3],

    pub pos: [f32; 3],

    pub tex: [f32; 2],

    pub tex_id: i32,

    pub color: [u8; 4],

    pub light: u32,

}

impl Vertex for MeshVertex {

    fn build_bindings() -> VertexFormat {
        use std::borrow::Cow;

        let dummy: MeshVertex = unsafe { std::mem::uninitialized() };

        macro_rules! offset {
            ($dummy: expr, $field: expr) => {
                {
                    let dummy_ref = &$dummy;
                    let field_ref = &$field;
                    (field_ref as *const _ as usize) - (dummy_ref as *const _ as usize)
                }
            };
        }

        Cow::Owned(vec![
            (Cow::Borrowed("loc"), offset!(dummy, dummy.loc), AttributeType::I32I32I32, false ),
            (Cow::Borrowed("pos"), offset!(dummy, dummy.pos), AttributeType::F32F32F32, false ),
            (Cow::Borrowed("tex"), offset!(dummy, dummy.tex), AttributeType::F32F32, false ),
            (Cow::Borrowed("tex_id"), offset!(dummy, dummy.tex_id), AttributeType::I32, false ),
            (Cow::Borrowed("color"), offset!(dummy, dummy.color), AttributeType::U32, false ),
            (Cow::Borrowed("light"), offset!(dummy, dummy.light), AttributeType::U32, false ),
        ])

    }
}
//glium::implement_vertex!(MeshVertex, loc, pos, tex, tex_id, color, light);


pub struct MeshUniform<'a> {

    pub world: [[f32; 4]; 4],

    pub center: [i32; 3],

    pub textures: Sampler<'a, Texture2dArray>,

    pub light_map: Sampler<'a, Texture2d>,

}

impl<'b> Uniforms for MeshUniform<'b> {

    fn visit_values<'a, F: FnMut(&str, UniformValue<'a>)>(&'a self, mut output: F) {
        output("world", self.world.as_uniform_value());
        output("center", self.center.as_uniform_value());
        output("textures", self.textures.as_uniform_value());
        output("light_map", self.light_map.as_uniform_value());
    }
}


pub struct Mesh<V> {

    vertexs: Vec<V>,

    indices: Vec<u16>,

}

impl<V: Vertex> Mesh<V> {

    pub fn new() -> Self {
        Mesh {
            vertexs: Vec::new(),
            indices: Vec::new(),
        }
    }

    pub fn with_capacity(capacity: (usize, usize)) -> Self {
        Mesh {
            vertexs: Vec::with_capacity(capacity.0),
            indices: Vec::with_capacity(capacity.1),
        }
    }

    pub fn with_count(n: usize) -> Self {
        Mesh {
            vertexs: Vec::with_capacity(n * 4),
            indices: Vec::with_capacity(n * 6),
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.vertexs.len(), self.indices.len())
    }

    pub fn append(&mut self, vertexs: &[V]) {
        let base = self.vertexs.len() as u16;
        let mut indices = [0, 1, 2, 3, 2, 1];
        for i in &mut indices {
            *i += base;
        }
        self.vertexs.extend(&vertexs[0..4]);
        self.indices.extend(&indices[0..6]);
    }

    pub fn draw<F: Facade, U: Uniforms>(&self, facade: &F, frame: &mut Frame, program: &Program, uniforms: &U, draw_parameters: &DrawParameters) -> GEResult<()> {
        if self.vertexs.len() > 0 {
            let vbuf = glium::VertexBuffer::immutable(facade, self.vertexs.as_slice()).map_err(Box::new)?;
            let ibuf = glium::IndexBuffer::immutable(facade, glium::index::PrimitiveType::TrianglesList, self.indices.as_slice()).map_err(Box::new)?;
            frame.draw(&vbuf, &ibuf, program, uniforms, draw_parameters).map_err(Box::new)?;
        }
        Ok(())
    }

}