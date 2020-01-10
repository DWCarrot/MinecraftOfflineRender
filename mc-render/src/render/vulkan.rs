use std::sync::Arc;

use vulkano::instance::Instance;
use vulkano::instance::ApplicationInfo;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::Queue;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::pipeline::vertex::Vertex;
use vulkano::framebuffer::Framebuffer;
use vulkano::framebuffer::Subpass;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::command_buffer::CommandBuffer;
use vulkano::command_buffer::DynamicState;
use vulkano::sync::GpuFuture;

use image::RgbaImage;

type VKResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;


pub struct VulkanRender {

    device: Arc<Device>,

    queue: Arc<Queue>,
}


impl VulkanRender {

    pub fn new() -> VKResult<Self> {
        let instance = Instance::new(
            Some(&ApplicationInfo::default()), 
            &InstanceExtensions::none(), 
            None
        )
        .map_err(Box::new)?;
        let physical_device = {
            let mut physical_device = None;
            for p in PhysicalDevice::enumerate(&instance) {
                println!("{}", p.name());
                physical_device = Some(p);
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
        Ok(VulkanRender {
            device,
            queue
        })
    }

    pub fn render_image<V: Vertex + Clone>(&self, dim: (u32, u32), data: &[V]) -> VKResult<RgbaImage> {

        let image = StorageImage::new(
            self.device.clone(), 
            Dimensions::Dim2d { width: dim.0, height: dim.1 }, 
            Format::R8G8B8A8Unorm,
            Some(self.queue.family())
        )
        .map_err(Box::new)?;

        let buf = CpuAccessibleBuffer::from_iter(
            self.device.clone(), 
            BufferUsage::all(),
            (0 .. dim.0 * dim.1 * 4).map(|_| 0u8)
        )
        .map_err(Box::new)?;

        let render_pass = Arc::new(
            vulkano::single_pass_renderpass!(self.device.clone(),
                attachments: {
                    color: {
                        load: Clear,
                        store: Store,
                        format: Format::R8G8B8A8Unorm,
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            ).map_err(Box::new)?
        );

        let framebuffer = Arc::new(
            Framebuffer::start(render_pass.clone())
            .add(image.clone()).map_err(Box::new)?
            .build().map_err(Box::new)?
        );

        let pipeline = Arc::new(
            GraphicsPipeline::start()
            .vertex_input_single_buffer::<V>()
            //.vertex_shader(vs.main_entry_point(), ())
            .viewports_dynamic_scissors_irrelevant(1)
            //.fragment_shader(fs.main_entry_point(), ())
            .render_pass(Subpass::from(render_pass.clone(), 0).expect("unable to build subpass"))
            .build(self.device.clone())
            .map_err(Box::new)?
        );

        let dynamic_state = DynamicState {
            viewports: Some(vec![Viewport {
                origin: [0.0, 0.0],
                dimensions: [dim.0 as f32, dim.1 as f32],
                depth_range: 0.0 .. 1.0,
            }]),
            .. DynamicState::none()
        };

        let vertex_buffer = CpuAccessibleBuffer::from_iter(
            self.device.clone(), BufferUsage::all(),
            Vec::from(data).into_iter()
        )
        .unwrap();

        let command_buffer = AutoCommandBufferBuilder::primary_one_time_submit(self.device.clone(), self.queue.family())
            .unwrap()

            .begin_render_pass(framebuffer.clone(), false, vec![[0.0, 0.0, 1.0, 1.0].into()])
            .unwrap()

            .draw(pipeline.clone(), &dynamic_state, vertex_buffer.clone(), (), ())
            .unwrap()

            .end_render_pass()
            .unwrap()

            .copy_image_to_buffer(image.clone(), buf.clone())
            .unwrap()

            .build()
            .unwrap();

        let finished = command_buffer.execute(self.queue.clone()).unwrap();
        finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

        let buffer_content = buf.read().unwrap();
        let buffer_content = buffer_content.to_vec();

        let img = RgbaImage::from_raw(dim.0, dim.1, buffer_content).unwrap();

        Ok(img)
    }
}

