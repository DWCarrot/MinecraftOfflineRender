use std::sync::Arc;

use cgmath::Matrix3;
use cgmath::Matrix4;
use cgmath::Vector3;

use vulkano::device::Device;
use vulkano::memory::pool::StdMemoryPool;
use vulkano::memory::DeviceMemoryAllocError;
use vulkano::buffer::cpu_pool::CpuBufferPool;
use vulkano::buffer::cpu_pool::CpuBufferPoolSubbuffer;

pub mod vs {

    vulkano_shaders::shader!{
        ty: "vertex",
        path: "src/vkrender/shader/vert.glsl"
    }
}

pub mod fs {

    vulkano_shaders::shader!{
        ty: "fragment",
        path: "src/vkrender/shader/frag.glsl"
    }
}


pub type Position = vs::ty::Position;

#[inline]
pub fn uniform_position(x: f32, y: f32, z: f32) -> Position {
    vs::ty::Position {
        pos: [x, y, z]
    }
}


pub type Rotation = vs::ty::Rotation;

#[inline]
pub fn uniform_rotation(rotate: Matrix3<f32>, center: Vector3<f32>) -> Rotation {
    vs::ty::Rotation {
        _dummy0: [0u8; 4],
        center: center.into(),
        rotate: rotate.into(),
    }
}


pub type Light = vs::ty::Light;

#[inline]
pub fn uniform_light(normal: Vector3<f32>, block_light: f32, sky_light: f32) -> Light {
    vs::ty::Light {
        _dummy0: [0u8; 4],
        normal: normal.into(),
        lit_coord: [block_light, sky_light],
    }
}


pub type World = vs::ty::World;

#[inline]
pub fn uniform_world(proj: Matrix4<f32>) -> vs::ty::World {
    vs::ty::World {
        proj: proj.into(),
    }
}

#[derive(Default, Debug, Clone)]
pub struct MeshVertex {
    pub geo: [f32; 3],
    pub tex: [f32; 2],
    pub color: [f32; 4],
}

vulkano::impl_vertex!(MeshVertex, geo, tex, color);

// pub struct UniformBuffer {

//     pool_pos: CpuBufferPool<Position>,

//     pool_rot: CpuBufferPool<Rotation>,

//     pool_lit: CpuBufferPool<Light>,

// }

// impl UniformBuffer {

//     pub fn new(device: Arc<Device>) -> Self {
//         UniformBuffer {
//             pool_pos: CpuBufferPool::uniform_buffer(device.clone()),
//             pool_rot: CpuBufferPool::uniform_buffer(device.clone()),
//             pool_lit: CpuBufferPool::uniform_buffer(device.clone()),
//         }
//     }

//     #[inline]
//     pub fn position(
//         &self, 
//         x: f32, 
//         y: f32, 
//         z: f32
//     ) -> Result<CpuBufferPoolSubbuffer<Position, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
//         self.pool_pos.next(uniform_position(x, y, z))
//     }

//     #[inline]
//     pub fn rotation(
//         &self, 
//         rotate: Matrix3<f32>, 
//         center: Vector3<f32>
//     ) -> Result<CpuBufferPoolSubbuffer<Rotation, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
//         self.pool_rot.next(uniform_rotation(rotate, center))
//     }

//     #[inline]
//     pub fn light(
//         &self, 
//         normal: Vector3<f32>, 
//         block_light: f32, 
//         sky_light: f32
//     ) -> Result<CpuBufferPoolSubbuffer<Light, Arc<StdMemoryPool>>, DeviceMemoryAllocError> {
//         self.pool_lit.next(uniform_light(normal, block_light, sky_light))
//     }
// }