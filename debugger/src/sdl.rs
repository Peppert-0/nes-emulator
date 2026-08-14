use ash::vk::SurfaceKHR;
use ash::Entry;
use dear_imgui_rs::render;
use sdl3::{Sdl, VideoSubsystem, event::Event, keyboard::Keycode, surface::Surface, video::{Window, WindowBuildError}};
use crate::renderer::{self, RendererError, VulkanContext};
use std::{ffi::{CString, NulError}, io::Error, time::Duration};

pub struct Context {
    pub sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    pub window: Window,
}

#[derive(Debug)]
pub enum ContextError {
    Sdl(sdl3::Error),
    WindowBuild(WindowBuildError),
    FfiNul(NulError),
    Renderer(RendererError),
    Io(std::io::Error),
    VulkanLoad(ash::LoadingError),
}

impl From<sdl3::Error> for ContextError {
    fn from(err: sdl3::Error) -> Self {
        Self::Sdl(err)
    }
}
impl From<WindowBuildError> for ContextError {
    fn from(err: WindowBuildError) -> Self {
        Self::WindowBuild(err)
    }
}
impl From<NulError> for ContextError {
    fn from(err: NulError) -> Self {
        Self::FfiNul(err)
    }
}
impl From<RendererError> for ContextError {
    fn from(err: RendererError) -> Self {
        Self::Renderer(err)
    }
}
impl From<std::io::Error> for ContextError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}
impl From<ash::LoadingError> for ContextError {
    fn from(err: ash::LoadingError) -> Self {
        Self::VulkanLoad(err)
    }
}

impl Context {
    pub fn new() -> Result<Self, ContextError>  {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem.window("NES Debugger", 800, 600)
            .resizable()
            .vulkan()
            .build()?;
        let extensions = window.vulkan_instance_extensions()?;
        let entry = unsafe { Entry::load()? };
        let instance = VulkanContext::create_instance(&entry, extensions)?;
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let raw_instance = instance.handle();
        let vulkan_surface = unsafe {window.vulkan_create_surface(raw_instance)?};
        let vulkan_context = VulkanContext::new(entry, instance, vulkan_surface)?;
        let (width, height) = window.size_in_pixels();

        Ok(
            Self {
                sdl_context,
                video_subsystem,
                window,
            }
        )
    }
    pub fn main_loop(&mut self) -> Result<(), ContextError> {
        let mut event_pump = self.sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running Ok(())
                    },
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
} 

