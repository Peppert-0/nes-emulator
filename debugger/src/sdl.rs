use crate::{
    gui,
    renderer::{self, Renderer, RendererError, Swapchain, VulkanContext},
};
use ash::vk::SurfaceKHR;
use ash::{
    Entry,
    vk::{AccessFlags2, ImageLayout, PipelineStageFlags2},
};
use egui::{FullOutput, PointerButton, Pos2};
use egui_ash_renderer::{DynamicRendering, allocator::DefaultAllocator};
use sdl3::{
    EventPump, Sdl, VideoSubsystem,
    event::Event,
    keyboard::Keycode,
    mouse::MouseButton,
    surface::Surface,
    sys::metadata::render,
    video::{Window, WindowBuildError},
};
use std::{
    ffi::{CString, NulError},
    io::Error,
    time::Duration,
    u64,
};

pub struct Context {
    pub sdl_context: Sdl,
    egui_context: egui::Context,
    event_pump: EventPump,
    video_subsystem: VideoSubsystem,
    pub window: Window,
    pub renderer: Renderer,
    egui_renderer: egui_ash_renderer::Renderer<DefaultAllocator>,
}

#[derive(Debug)]
pub enum ContextError {
    Sdl(sdl3::Error),
    WindowBuild(WindowBuildError),
    FfiNul(NulError),
    Renderer(RendererError),
    Vulkan(ash::vk::Result),
    EguiRenderer(egui_ash_renderer::RendererError),
    Io(std::io::Error),
    VulkanLoad(ash::LoadingError),
    Gui(gui::GuiError),
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
impl From<ash::vk::Result> for ContextError {
    fn from(err: ash::vk::Result) -> Self {
        Self::Vulkan(err)
    }
}
impl From<egui_ash_renderer::RendererError> for ContextError {
    fn from(err: egui_ash_renderer::RendererError) -> Self {
        Self::EguiRenderer(err)
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
impl From<gui::GuiError> for ContextError {
    fn from(err: gui::GuiError) -> Self {
        Self::Gui(err)
    }
}

impl Context {
    pub fn new() -> Result<Self, ContextError> {
        let sdl_context = sdl3::init()?;
        let egui_context = egui::Context::default();
        let event_pump = sdl_context.event_pump()?;
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
        let egui_renderer = Self::build_egui_renderer(&renderer)?;

        Ok(Self {
            sdl_context,
            egui_context,
            event_pump,
            video_subsystem,
            window,
            renderer,
            egui_renderer,
        })
    }
    fn build_egui_renderer(
        renderer: &Renderer,
    ) -> Result<egui_ash_renderer::Renderer<DefaultAllocator>, ContextError> {
        let render_mode = egui_ash_renderer::DynamicRendering {
            color_attachment_format: ash::vk::Format::R8G8B8A8_SRGB,
            depth_attachment_format: None,
            stencil_attachment_format: None,
        };
        let options = egui_ash_renderer::Options {
            in_flight_frames: 1,
            enable_depth_test: false,
            enable_depth_write: false,
            srgb_framebuffer: true,
        };
        let egui_renderer = egui_ash_renderer::Renderer::with_default_allocator(
            &renderer.context.instance,
            renderer.context.physical_device,
            renderer.context.device.clone(),
            egui_ash_renderer::RenderMode::DynamicRendering(render_mode),
            options,
        )?;

        Ok(egui_renderer)
    }
    pub fn render_gui(&mut self, events: Vec<egui::Event>) -> Result<(), ContextError> {
        let device = self.renderer.context.device.clone();
        let command_buffer = self.renderer.command.buffer;
        unsafe {
            device.wait_for_fences(&[self.renderer.sync.in_flight_fence], true, u64::MAX)?;
            device.reset_fences(&[self.renderer.sync.in_flight_fence])?
        };
        let raw_input = self.build_raw_input(events)?;
        let mut full_output = gui::Gui::draw_gui(&self.egui_context, raw_input)?;
        for (id, deltas) in full_output.textures_delta.set.drain() {
            for delta in deltas {
                self.egui_renderer.set_texture(
                    self.renderer.context.queue,
                    self.renderer.command.pool,
                    id,
                    &delta,
                )?;
            }
        }
        let clipped_primitives = self
            .egui_context
            .tessellate(full_output.shapes, full_output.pixels_per_point);
        unsafe {
            device.reset_command_buffer(command_buffer, ash::vk::CommandBufferResetFlags::empty())
        }?;
        self.renderer.get_next_image_index()?;
        self.renderer.begin_recording_commands()?;
        self.renderer.transition_image_layout(
            ImageLayout::UNDEFINED,
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            AccessFlags2::empty(),
            AccessFlags2::COLOR_ATTACHMENT_WRITE,
            PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
        )?;
        let attachment_info = ash::vk::RenderingAttachmentInfo::default()
            .image_view(
                self.renderer.swapchain.image_views
                    [self.renderer.swapchain.next_image_index as usize],
            )
            .image_layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .load_op(ash::vk::AttachmentLoadOp::CLEAR)
            .store_op(ash::vk::AttachmentStoreOp::STORE)
            .clear_value(ash::vk::ClearValue {
                color: ash::vk::ClearColorValue {
                    float32: [1.0, 0.0, 1.0, 1.0],
                },
            });
        let attachment_infos = &[attachment_info];
        let rendering_info = ash::vk::RenderingInfo::default()
            .render_area(ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: self.renderer.context.surface_extent,
            })
            .layer_count(1)
            .color_attachments(attachment_infos);
        unsafe { device.cmd_begin_rendering(command_buffer, &rendering_info) };
        self.egui_renderer.cmd_draw(
            command_buffer,
            self.renderer.context.surface_extent,
            full_output.pixels_per_point,
            &clipped_primitives,
        )?;
        unsafe {
            device.cmd_end_rendering(command_buffer);
        }
        self.renderer.transition_image_layout(
            ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
            ImageLayout::PRESENT_SRC_KHR,
            AccessFlags2::COLOR_ATTACHMENT_WRITE,
            AccessFlags2::empty(),
            PipelineStageFlags2::COLOR_ATTACHMENT_OUTPUT,
            PipelineStageFlags2::BOTTOM_OF_PIPE_KHR,
        )?;
        self.renderer.finish_recording_commands()?;
        self.renderer.submit_queue()?;
        for id in full_output.textures_delta.free.drain() {
            self.egui_renderer.free_texture(id)?;
        }
        self.renderer.present()?;

        Ok(())
    }
    pub fn main_loop(&mut self) -> Result<(), ContextError> {
        'running: loop {
            let mut egui_events: Vec<egui::Event> = vec![];
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running Ok(()),
                    _ => {
                        if let Some(egui_event) = Self::map_event(event)? {
                            egui_events.push(egui_event);
                        }
                    }
                }
            }
            self.render_gui(egui_events)?;

            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
    fn map_event(event: Event) -> Result<Option<egui::Event>, ContextError> {
        match event {
            Event::MouseMotion { x, y, .. } => {
                return Ok(Some(egui::Event::PointerMoved(egui::pos2(
                    x as f32, y as f32,
                ))));
            }
            Event::MouseButtonDown {
                x, y, mouse_btn, ..
            } => {
                return Ok(Some(egui::Event::PointerButton {
                    pos: egui::pos2(x, y),
                    button: Self::map_mouse_button(mouse_btn)?.unwrap(),
                    pressed: true,
                    modifiers: egui::Modifiers::NONE,
                }));
            }
            Event::MouseButtonUp {
                mouse_btn, x, y, ..
            } => {
                return Ok(Some(egui::Event::PointerButton {
                    pos: egui::pos2(x, y),
                    button: Self::map_mouse_button(mouse_btn)?.unwrap(),
                    pressed: false,
                    modifiers: egui::Modifiers::NONE,
                }));
            }
            _ => return Ok(None),
        }
    }
    fn map_mouse_button(
        button: sdl3::mouse::MouseButton,
    ) -> Result<Option<egui::PointerButton>, ContextError> {
        Ok(match button {
            MouseButton::Left => Some(PointerButton::Primary),
            MouseButton::Right => Some(PointerButton::Secondary),
            MouseButton::Middle => Some(PointerButton::Middle),
            MouseButton::X1 => Some(PointerButton::Extra1),
            MouseButton::X2 => Some(PointerButton::Extra2),
            MouseButton::Unknown => None,
        })
    }
    fn build_raw_input(&self, events: Vec<egui::Event>) -> Result<egui::RawInput, ContextError> {
        let (width, height) = self.window.size();
        let size = egui::Vec2 {
            x: width as f32,
            y: height as f32,
        };
        let screen_rect = Some(egui::Rect::from_min_size(Default::default(), size));
        let raw_input = egui::RawInput {
            events,
            screen_rect,
            ..Default::default()
        };

        Ok(raw_input)
    }
}
