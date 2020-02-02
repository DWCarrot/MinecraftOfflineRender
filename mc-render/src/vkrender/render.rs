use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::instance::QueueFamily;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuBufferPool;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;
use vulkano::image::ImmutableImage;
use vulkano::image::AttachmentImage;
use vulkano::sampler::Sampler;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::GraphicsPipelineAbstract;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::FramebufferAbstract;
use vulkano::framebuffer::Subpass;
use vulkano::framebuffer::RenderPassAbstract;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::command_buffer::DynamicState;
use vulkano::sync::GpuFuture;

use image::RgbaImage;

use super::shader;
use super::shader::MeshVertex;
use super::shader::Position;
use super::shader::Rotation;
use super::shader::Light;
use super::shader::World;

use super::uuid::UuidStorage;

type GEResult<T> = Result<T, Box<dyn std::error::Error>>;


pub fn vk_device(id: Option<UuidStorage>) -> GEResult<(Arc<Device>, Arc<Queue>)> {

    let instance = Instance::new(
        Some(&ApplicationInfo::default()), 
        &InstanceExtensions::none(), 
        None
    )
    .map_err(Box::new)?;
    let physical_device = {
        let mut physical_device = None;
        for p in PhysicalDevice::enumerate(&instance) {
            let uuid = UuidStorage::from(p.uuid());
            println!("{} [{}]", p.name(), &uuid);
            if id.is_none() || id == Some(uuid) {
                physical_device = Some(p);
            }
        }
        physical_device.expect("couldn't find a physical device")
    };
    let queue_family = physical_device.queue_families()
        .find(|&q| q.supports_graphics() && q.supports_compute() )
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(physical_device, &Features::none(), &DeviceExtensions::none(),
                [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")
    };
    let queue = queues.next().expect("could not get queue");
    Ok((
        device,
        queue
    ))
}


pub struct Context {

    device: Arc<Device>,

    queue: Arc<Queue>, 

    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,    

    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,
}

impl Context {

    pub fn new(device: Arc<Device>, queue: Arc<Queue>) -> GEResult<Self> {

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: Format::R8G8B8A8Unorm,
                        samples: 1,
                    },
                    depth: {
                        load: Clear,
                        store: DontCare,
                        format: Format::D16Unorm,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {depth}
                }
            ).map_err(Box::new)?
        );

        let vs = shader::vs::Shader::load(device.clone()).map_err(Box::new)?;
        let fs = shader::fs::Shader::load(device.clone()).map_err(Box::new)?;
        
        let pipeline = Arc::new(
            GraphicsPipeline::start()
            .vertex_input_single_buffer::<MeshVertex>()
            .vertex_shader(vs.main_entry_point(), ())
            .triangle_strip()   // x-y-, x+y-, x-y+, x+y+
            .viewports_dynamic_scissors_irrelevant(1)
            .fragment_shader(fs.main_entry_point(), ())
            .blend_alpha_blending()
            .render_pass(Subpass::from(render_pass.clone(), 0).expect("unable to build subpass"))
            .build(device.clone())
            .map_err(Box::new)?
        );
    
        
        Ok( Context {
            device,
            queue,
            pipeline,
            render_pass
        } )
    }

    pub fn renderer(&self, width: u32, height: u32, world: World) -> GEResult<Renderer> {

        let image = StorageImage::new(
            self.device.clone(), 
            Dimensions::Dim2d { width, height }, 
            Format::R8G8B8A8Unorm,
            Some(self.queue.family())
        )
        .map_err(Box::new)?;

        let depth_buffer = AttachmentImage::transient(
            self.device.clone(),
            [width, height], 
            Format::D16Unorm
        )
        .map_err(Box::new)?;

        let framebuffer = Arc::new(
            Framebuffer::start(self.render_pass.clone())
            .add(image.clone())
            .map_err(Box::new)?
            .add(depth_buffer)
            .map_err(Box::new)?
            .build()
            .map_err(Box::new)?
        );

        let dynamic_state = DynamicState {
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [width as f32, height as f32],
                depth_range: 0.0 .. 1.0,
            }]),
            .. DynamicState::none()
        };

        let command_pool = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
            .map_err(Box::new)?

            .begin_render_pass(framebuffer, false, vec![[0.0, 0.0, 1.0, 1.0].into(), 1f32.into()])
            .map_err(Box::new)?;

        Ok(Renderer {
            device: self.device.clone(),
            queue: self.queue.clone(),
            pipeline: self.pipeline.clone(),
            command_pool,
            image,
            dynamic_state,
            pool_pos: CpuBufferPool::uniform_buffer(self.device.clone()),
            pool_rot: CpuBufferPool::uniform_buffer(self.device.clone()),
            pool_lit: CpuBufferPool::uniform_buffer(self.device.clone()),
            buf_world: CpuAccessibleBuffer::from_data(self.device.clone(), BufferUsage::uniform_buffer(), world).map_err(Box::new)?,
        })
    }
}


pub struct Renderer {

    device: Arc<Device>,

    queue: Arc<Queue>,

    command_pool: AutoCommandBufferBuilder,

    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>,

    image: Arc<StorageImage<Format>>,

    dynamic_state: DynamicState,

    pool_pos: CpuBufferPool<Position>,

    pool_rot: CpuBufferPool<Rotation>,

    pool_lit: CpuBufferPool<Light>,

    buf_world: Arc<CpuAccessibleBuffer<World>>,
}


impl Renderer {

    pub fn draw<I>(
        self,
        vertexs: I,
        position: Position,
        rotation: Rotation,
        light: Light,
        tex: (Arc<ImmutableImage<Format>>, Arc<Sampler>),
        lmmp: (Arc<ImmutableImage<Format>>, Arc<Sampler>)
    ) -> GEResult<Self> 
    where
        I: ExactSizeIterator<Item = MeshVertex>,
    {
        let vb = CpuAccessibleBuffer::from_iter(self.device.clone(), BufferUsage::vertex_buffer(), vertexs).map_err(Box::new)?;
        let set = Arc::new(
            PersistentDescriptorSet::start(self.pipeline.clone(), 0)
                .add_buffer(self.buf_world.clone())
                .map_err(Box::new)?
                .add_buffer(self.pool_pos.next(position).map_err(Box::new)?)
                .map_err(Box::new)?
                .add_buffer(self.pool_rot.next(rotation).map_err(Box::new)?)
                .map_err(Box::new)?
                .add_buffer(self.pool_lit.next(light).map_err(Box::new)?)
                .map_err(Box::new)?
                .add_sampled_image(tex.0, tex.1)
                .map_err(Box::new)?
                .add_sampled_image(lmmp.0, lmmp.1)
                .map_err(Box::new)?
                .build()
                .map_err(Box::new)?
        );
        let command_pool = self.command_pool.draw(
            self.pipeline.clone(), 
            &self.dynamic_state, 
            vec!(vb),
            set, 
            ()
        )
        .map_err(Box::new)?;
        Ok( Renderer {
            command_pool,
            ..self
        } )
    }

    pub fn flush(self) -> GEResult<RgbaImage> {

        let dim = self.image.dimensions();

        let buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(), 
            BufferUsage::all(), 
            (0 .. dim.width() * dim.height() * 4).map(|_| 0u8)
        )
        .map_err(Box::new)?;

        let command_buffer = self.command_pool
            .end_render_pass()
            .map_err(Box::new)?

            .copy_image_to_buffer(self.image, buf.clone())
            .map_err(Box::new)?

            .build()
            .map_err(Box::new)?;

        let future = command_buffer.execute(self.queue).map_err(Box::new)?;
        future.then_signal_fence_and_flush()
            .map_err(Box::new)?
            .wait(None)
            .map_err(Box::new)?;

        let buffer_content = buf.read().map_err(Box::new)?;
        let buffer_content = buffer_content.to_vec();

        let img = RgbaImage::from_raw(dim.width(), dim.height(), buffer_content).unwrap();

        Ok(img)
    }
}