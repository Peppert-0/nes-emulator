use crate::renderer::{self, Renderer, RendererError, Swapchain, VulkanContext};
use ash::Entry;
use ash::vk::SurfaceKHR;
use egui::PointerButton;
use sdl3::{
    Sdl, VideoSubsystem,
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    surface::Surface,
    video::{Window, WindowBuildError},
};
use std::{
    ffi::{CString, NulError},
    io::Error,
    time::Duration,
};

pub struct Context {
    pub sdl_context: Sdl,
    video_subsystem: VideoSubsystem,
    pub window: Window,
    pub renderer: Renderer,
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
    pub fn new() -> Result<Self, ContextError> {
        let sdl_context = sdl3::init()?;
        let video_subsystem = sdl_context.video()?;
        let window = video_subsystem
            .window("NES Debugger", 800, 600)
            .resizable()
            .vulkan()
            .build()?;
        let extensions = window.vulkan_instance_extensions()?;
        let entry = unsafe { Entry::load()? };
        let instance = VulkanContext::create_instance(&entry, extensions)?;
        let raw_instance = instance.handle();
        let vulkan_surface = unsafe { window.vulkan_create_surface(raw_instance) }?;
        let mut vulkan_context = VulkanContext::new(entry, instance, vulkan_surface)?;
        let (width, height) = window.size_in_pixels();
        let swapchain_khr = vulkan_context.create_swap_chain((width, height))?;
        let swapchain = Swapchain::new(&vulkan_context, swapchain_khr)?;
        let renderer = Renderer::new(vulkan_context, swapchain)?;

        Ok(Self {
            sdl_context,
            video_subsystem,
            window,
            renderer,
        })
    }
    pub fn main_loop(&mut self) -> Result<(), ContextError> {
        let mut event_pump = self.sdl_context.event_pump()?;
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => break 'running Ok(()),
                    _ => {}
                }
            }

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    fn map_events(self) -> Result<Vec<egui::Event>, ContextError> {
        let mut event_pump = self.sdl_context.event_pump()?;
        let events = event_pump.poll_iter();
        let mut egui_events: Vec<egui::Event> = vec![];
        for event in events {
            match event {
                Event::MouseMotion { x, y, .. } => {
                    egui_events.push(egui::Event::PointerMoved(egui::pos2(x as f32, y as f32)));
                }
                Event::MouseButtonDown {
                    x, y, mouse_btn, ..
                } => egui_events.push(egui::Event::PointerButton {
                    pos: egui::pos2(x as f32, y as f32),
                    button: Self::map_mouse_button(mouse_btn)?,
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }),
                Event::MouseButtonUp {
                    mouse_btn, x, y, ..
                } => egui_events.push(egui::Event::PointerButton {
                    pos: egui::pos2(x as f32, y as f32),
                    button: Self::map_mouse_button(mouse_btn)?,
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }),
                _ => {}
            }
        }
        Ok(egui_events)
    }
    fn map_mouse_button(
        button: sdl3::mouse::MouseButton,
    ) -> Result<egui::PointerButton, ContextError> {
        Ok(match button {
            MouseButton::Left => PointerButton::Primary,
            MouseButton::Right => PointerButton::Secondary,
            MouseButton::Middle => PointerButton::Middle,
            MouseButton::X1 => PointerButton::Extra1,
            MouseButton::X2 => PointerButton::Extra2,
            MouseButton::Unknown => PointerButton::Primary,
        })
    }
}
