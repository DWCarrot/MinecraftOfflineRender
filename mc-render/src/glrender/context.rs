

use std::path::Path;
use std::io;
use std::io::Read;

use glium::glutin;
use glutin::GlRequest;
use glutin::EventsLoop;
use glium::Frame;
use glium::HeadlessRenderer;
use glium::Display;
use glium::backend::Facade;
use glium::backend::Context as BContext;


pub type GEResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn parse_version() -> GlRequest {

    match std::env::var("GLIUM_GL_VERSION") {
        Ok(version) => {
            // expects "OpenGL 3.3" for example
            let mut iter = version.rsplitn(2, ' ');
            let version = iter.next().unwrap();
            let ty = iter.next().unwrap();
            let mut iter = version.split('.');
            let major = iter.next().unwrap().parse().unwrap();
            let minor = iter.next().unwrap().parse().unwrap();
            let ty = if ty == "OpenGL" {
                glutin::Api::OpenGl
            } else if ty == "OpenGL ES" {
                glutin::Api::OpenGlEs
            } else if ty == "WebGL" {
                glutin::Api::WebGl
            } else {
                panic!();
            };
            glutin::GlRequest::Specific(ty, (major, minor))
        },
        Err(_) => glutin::GlRequest::Latest,
    }
}


pub fn build_display_headless(size: (u32, u32), event_loop: &EventsLoop, version: GlRequest) -> HeadlessRenderer {

    let ctx = glutin::ContextBuilder::new()
        .with_gl(version)
        .with_depth_buffer(24)
        .build_headless(&event_loop, glutin::dpi::PhysicalSize::from(size))
        .unwrap();
    glium::HeadlessRenderer::new(ctx).unwrap()
}


pub fn build_display(size: (u32, u32), event_loop: &EventsLoop, version: GlRequest) -> Display {

    let wb = glutin::WindowBuilder::new()
        .with_visibility(false)
        .with_dimensions(glutin::dpi::LogicalSize::from(size));
    let cb = glutin::ContextBuilder::new()
        .with_gl(version)
        .with_depth_buffer(24);

    glium::Display::new(wb, cb, &event_loop).unwrap()

}


pub trait Context {
    type F: Facade;

    fn facade(&self) -> &Self::F;

    fn context(&self) -> &BContext;

    fn surface(&self) -> Frame;

    fn build(width: u32, height: u32, version: GlRequest) -> Self;

}


pub struct WindowHideContext {
    display: Display,
    event_loop: EventsLoop,
}

impl WindowHideContext {

    pub fn wait(&mut self) {
        let mut working = true;
        let itv = std::time::Duration::from_millis(1);
        while working {
            self.event_loop.poll_events(|e| {
                match e {
                    glutin::Event::WindowEvent{window_id, event} => {
                        match event {
                            glutin::WindowEvent::CloseRequested => {
                                working = false;
                            },
                            _ => {

                            }
                        }
                    },
                    _ => {

                    }
                }
            });
            std::thread::sleep(itv);
        }
    }
}

impl Context for WindowHideContext {
    type F = Display;

    fn facade(&self) -> &Self::F {
        &self.display
    }

    fn surface(&self) -> Frame {
        self.display.draw()
    }

    fn context(&self) -> &BContext {
        &self.display
    }

    fn build(width: u32, height: u32, version: GlRequest) -> Self {
        let event_loop = EventsLoop::new();
        let dpi = {
            let w = glutin::WindowBuilder::new()
                .with_visibility(false)
                .build(&event_loop)
                .unwrap();
            w.get_hidpi_factor()
        };
        let wb = glutin::WindowBuilder::new()
            .with_visibility(true)
            .with_dimensions(glutin::dpi::PhysicalSize::from((width, height)).to_logical(dpi));
        let cb = glutin::ContextBuilder::new()
            .with_pixel_format(24, 8)
            .with_depth_buffer(24)
            .with_stencil_buffer(8)
            .with_gl(version);
        let display = Display::new(wb, cb, &event_loop).unwrap();
        WindowHideContext {
            display,
            event_loop
        }
    }

}