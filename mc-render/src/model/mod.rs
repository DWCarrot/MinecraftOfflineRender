pub mod block;
pub mod blockstate;
pub mod model;

use std::rc::Rc;

use cgmath::Vector2;
use cgmath::Vector3;
use cgmath::Zero;

use crate::assets::data_type::Face;
use model::TransformedModel;
use block::World;
use block::RenderableBlock;

pub trait BlockRenderer {
    type Texture: Clone;
    type E;

    /**
     *  change the drawing priority. smaller will be draw first
     */
    fn state(&mut self, prior: i32) -> i32;

    /**
     * 
     */
    fn draw(
        &mut self, 
        loc: Vector3<i32>, 
        vp0: Vector3<f32>, vp1: Vector3<f32>, vp2: Vector3<f32>, vp3: Vector3<f32>, 
        vt0: Vector2<f32>, vt1: Vector2<f32>, vt2: Vector2<f32>, vt3: Vector2<f32>,
        tex: Self::Texture, 
        color: [u8; 4], 
        light: u8
    ) -> Result<(), Self::E>;

}

pub fn draw<T, E, B> (
    faces: &[Face],
    loc: &Vector3<i32>, 
    renderer: &mut dyn BlockRenderer<Texture = T,E = E>, 
    world: &dyn World<Block = B>
) -> Result<(), E>
where
    T: Clone,
    B: RenderableBlock<Model = Rc<TransformedModel<T>>>,
{
    
    let draw_model = |tmodel: &Rc<TransformedModel<T>>, block: &B, renderer: &mut dyn BlockRenderer<Texture=T,E=E>| -> Result<(), E> {
        let model = tmodel.model.as_ref();
        for element in model.elements.as_slice() {
            for face in faces {
                let mface = tmodel.mapping(face.clone());
                if let Some(face_tex) = &element.faces[mface.index()] {
                    if let Some(cullface) = &face_tex.cullface {
                        let cullface = tmodel.inv_mapping(cullface.clone());
                        if let Some(pos) = cullface.near(loc) {
                            if !world.is_air(&pos) {
                                continue;
                            }
                        }
                    }   // cullface
                    let mut vp0 = Vector3::zero();
                    let mut vp1 = Vector3::zero();
                    let mut vp2 = Vector3::zero();
                    let mut vp3 = Vector3::zero();
                    element.cubic.get_face_vert(mface.clone(), &mut vp0, &mut vp1, &mut vp2, &mut vp3);
                    let tmat = tmodel.mapping_transform(&element.rotation.transf);
                    let center = element.rotation.origin;
                    vp0 = tmat * (vp0 - center) + center;
                    vp1 = tmat * (vp1 - center) + center;
                    vp2 = tmat * (vp2 - center) + center;
                    vp3 = tmat * (vp3 - center) + center;
                    let mut vt0 = Vector2::zero();
                    let mut vt1 = Vector2::zero();
                    let mut vt2 = Vector2::zero();
                    let mut vt3 = Vector2::zero();
                    face_tex.get_face_vert(tmodel.rotation(mface.clone()), &mut vt0, &mut vt1, &mut vt2, &mut vt3);
                    let texture = face_tex.texture.clone();
                    let color = face_tex.tintindex.map(|tintindex| block.get_inline_color(tintindex)).unwrap_or_else(|| [255; 4]);
                    let light = block.get_light();
                    renderer.draw(*loc, vp0, vp1, vp2, vp3, vt0, vt1, vt2, vt3, texture, color, light)?;
                }
            }
        }
        Ok(())
    };

    let block = world.get(loc);
    if block.is_water() || block.is_water_logged() {
        renderer.state(2);
        for tmodel in block.get_water_models() {
            draw_model(tmodel, &block, renderer)?;
        }
    }
    if !block.is_water() {
        renderer.state(0);
        for tmodel in block.get_models() {
            draw_model(tmodel, &block, renderer)?;
        }
    }
    Ok(())
}