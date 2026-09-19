use ash::{
    self, Entry, Instance,
    vk::{
        self, AllocationCallbacks, Buffer, BufferCreateFlags, BufferCreateInfo, BufferUsageFlags,
        CommandBuffer, CommandBufferAllocateInfo, CommandBufferLevel, CommandPool,
        CommandPoolCreateFlags, CommandPoolCreateInfo, Device, DeviceCreateInfo, DeviceMemory,
        DeviceQueueCreateInfo, Extent2D, FenceCreateInfo, Handle, Image, ImageAspectFlags,
        ImageCreateInfo, ImageTiling, ImageUsageFlags, ImageView, ImageViewCreateInfo,
        ImageViewType, MemoryAllocateInfo, MemoryMapFlags, MemoryPropertyFlags, MemoryRequirements,
        PhysicalDevice, PhysicalDeviceFeatures, PresentModeKHR, QueueFlags, SampleCountFlags,
        SemaphoreCreateInfo, SharingMode, SubresourceHostMemcpySizeEXT, SurfaceFormatKHR,
        SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR,
    },
};
use std::{
    ffi::{CStr, CString, c_char},
    fmt::format,
};

use crate::bitmap::{Bitmap, Rgba};

pub struct Renderer {
    context: VulkanContext,
    swapchain: Swapchain,
    command: Command,
    sync: Sync,
}

pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub surface: SurfaceKHR,
    pub surface_instance: ash::khr::surface::Instance,
    pub surface_format: SurfaceFormatKHR,
    pub surface_extent: Extent2D,
    pub physical_device: PhysicalDevice,
    pub queue_index: u32,
    pub device: ash::Device,
}

pub struct Swapchain {
    device: ash::khr::swapchain::Device,
    swapchain: SwapchainKHR,
    images: Vec<Image>,
    next_image_index: u32,
}

struct Command {
    pool: CommandPool,
    buffer: CommandBuffer,
}

struct Sync {
    image_available_semaphore: vk::Semaphore,
    copy_finished_semaphore: vk::Semaphore,
    in_flight_fence: vk::Fence,
}

#[derive(Debug)]
pub enum RendererError {
    VulkanLoad(ash::LoadingError),
    Vulkan(ash::vk::Result),
    Nul(std::ffi::NulError),
}

impl From<ash::LoadingError> for RendererError {
    fn from(err: ash::LoadingError) -> Self {
        Self::VulkanLoad(err)
    }
}
impl From<ash::vk::Result> for RendererError {
    fn from(err: ash::vk::Result) -> Self {
        Self::Vulkan(err)
    }
}
impl From<std::ffi::NulError> for RendererError {
    fn from(err: std::ffi::NulError) -> Self {
        Self::Nul(err)
    }
}

trait CStrHelper {
    fn as_c_char_array(&self) -> Vec<*const c_char>;
}

impl CStrHelper for &[&CStr] {
    fn as_c_char_array(&self) -> Vec<*const c_char> {
        self.iter().map(|name| name.as_ptr()).collect()
    }
}

impl VulkanContext {
    pub fn new(
        entry: Entry,
        instance: Instance,
        window_surface: SurfaceKHR,
    ) -> Result<Self, RendererError> {
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let physical_device =
            VulkanContext::pick_physical_device(&instance, &surface_instance, &window_surface)?;
        let surface_format =
            Self::pick_surface_format(&surface_instance, &physical_device, &window_surface)?;
        let queue_index = VulkanContext::get_queue_family_index(
            &instance,
            &physical_device,
            &surface_instance,
            &window_surface,
        )?;
        let device = VulkanContext::create_logical_device(
            &instance,
            &physical_device,
            &surface_instance,
            &window_surface,
        )?;
        let surface_extent = Extent2D::default();

        Ok(Self {
            entry,
            instance,
            surface: window_surface,
            surface_instance,
            surface_format,
            surface_extent,
            physical_device,
            queue_index,
            device,
        })
    }
    pub fn create_instance(
        entry: &Entry,
        extensions: Vec<String>,
    ) -> Result<Instance, RendererError> {
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 3, 0),
            ..Default::default()
        };
        let extension_names: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()))
            .collect::<Result<_, _>>()?;

        let extension_ptrs: Vec<*const c_char> =
            extension_names.iter().map(|name| name.as_ptr()).collect();
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_ptrs);

        Ok(unsafe { entry.create_instance(&create_info, None)? })
    }
    pub fn pick_physical_device(
        instance: &Instance,
        surface_instance: &ash::khr::surface::Instance,
        surface: &SurfaceKHR,
    ) -> Result<PhysicalDevice, RendererError> {
        let physical_devices = unsafe { instance.enumerate_physical_devices()? };
        if physical_devices.is_empty() {
            panic!("No Vulkan-supported device found");
        }

        let mut candidates: Vec<(PhysicalDevice, u16)> = vec![];

        for physical_device in physical_devices {
            let properties = unsafe { instance.get_physical_device_properties(physical_device) };
            let mut score: u16 = 0;

            if properties.device_type == vk::PhysicalDeviceType::DISCRETE_GPU {
                score += 1000;
            }

            if VulkanContext::device_is_suitable(
                &instance,
                &physical_device,
                surface_instance,
                surface,
            )? {
                candidates.push((physical_device, score));
            }
        }

        let device = if !candidates.is_empty() {
            candidates
                .into_iter()
                .max_by_key(|(_, score)| *score)
                .unwrap()
        } else {
            panic!("No suitable Vulkan-supported device found");
        };

        Ok(device.0)
    }
    pub fn device_is_suitable(
        instance: &Instance,
        device: &PhysicalDevice,
        surface_instance: &ash::khr::surface::Instance,
        surface: &SurfaceKHR,
    ) -> Result<bool, RendererError> {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(*device) };
        let features = unsafe { instance.get_physical_device_features(*device) };
        let required_extensions: &[&CStr] = &[ash::khr::swapchain::NAME];
        let available_extensions =
            unsafe { instance.enumerate_device_extension_properties(*device) }?;

        let supports_required_queue_family_properties =
            queue_families.iter().enumerate().any(|(index, family)| {
                family.queue_flags.contains(QueueFlags::GRAPHICS)
                    && unsafe {
                        surface_instance.get_physical_device_surface_support(
                            *device,
                            index as u32,
                            *surface,
                        )
                    }
                    .unwrap()
            });
        let supports_required_extensions = required_extensions.iter().all(|required| {
            available_extensions
                .iter()
                .any(|available| *required == available.extension_name_as_c_str().unwrap())
        });

        Ok(supports_required_queue_family_properties && supports_required_extensions)
    }
    pub fn get_queue_family_index(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        surface_instance: &ash::khr::surface::Instance,
        surface: &SurfaceKHR,
    ) -> Result<u32, RendererError> {
        let queue_families =
            unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
        let queue_family_index = queue_families
            .iter()
            .enumerate()
            .position(|(index, family)| {
                family.queue_flags.contains(QueueFlags::GRAPHICS)
                    && unsafe {
                        surface_instance.get_physical_device_surface_support(
                            *physical_device,
                            index as u32,
                            *surface,
                        )
                    }
                    .unwrap()
            })
            .map(|index| index as u32)
            .unwrap();
        Ok(queue_family_index)
    }
    pub fn create_logical_device(
        instance: &Instance,
        physical_device: &PhysicalDevice,
        surface_instance: &ash::khr::surface::Instance,
        surface: &SurfaceKHR,
    ) -> Result<ash::Device, RendererError> {
        let queue_family_index = VulkanContext::get_queue_family_index(
            instance,
            physical_device,
            surface_instance,
            surface,
        )?;
        let queue_priority = [0.5f32];
        let device_queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&queue_priority);
        let device_queue_create_infos = [device_queue_create_info];
        let device_features = PhysicalDeviceFeatures::default();
        let mut sync2_features =
            vk::PhysicalDeviceSynchronization2Features::default().synchronization2(true);
        let enabled_extensions: &[&CStr] = &[ash::khr::swapchain::NAME];
        let enabled_extensions_ptrs = enabled_extensions.as_c_char_array();
        let device_create_info = DeviceCreateInfo::default()
            .enabled_features(&device_features)
            .enabled_extension_names(&enabled_extensions_ptrs)
            .queue_create_infos(&device_queue_create_infos)
            .push_next(&mut sync2_features);
        Ok(unsafe { instance.create_device(*physical_device, &device_create_info, None) }?)
    }
    fn pick_surface_format(
        surface_instance: &ash::khr::surface::Instance,
        physical_device: &PhysicalDevice,
        surface: &SurfaceKHR,
    ) -> Result<SurfaceFormatKHR, RendererError> {
        let available_formats = unsafe {
            surface_instance.get_physical_device_surface_formats(*physical_device, *surface)
        }?;
        let format = available_formats
            .iter()
            .find(|format| {
                format.format == vk::Format::R8G8B8A8_SRGB
                    && format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
            })
            .unwrap_or(&available_formats[0]);

        Ok(*format)
    }
    fn pick_presentation_mode(&self) -> Result<PresentModeKHR, RendererError> {
        let available_presentation_modes = unsafe {
            self.surface_instance
                .get_physical_device_surface_present_modes(self.physical_device, self.surface)
        }?;
        let presentation_mode = available_presentation_modes
            .iter()
            .find(|mode| **mode == vk::PresentModeKHR::MAILBOX)
            .unwrap_or(&vk::PresentModeKHR::FIFO);

        Ok(*presentation_mode)
    }
    pub fn get_image_extent(
        &mut self,
        (width, height): (u32, u32),
    ) -> Result<Extent2D, RendererError> {
        let capabilities = unsafe {
            self.surface_instance
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
        }?;
        if capabilities.current_extent.width != u32::MAX {
            Ok(capabilities.current_extent)
        } else {
            let extent = Extent2D {
                width: width.clamp(
                    capabilities.min_image_extent.width,
                    capabilities.max_image_extent.width,
                ),
                height: height.clamp(
                    capabilities.min_image_extent.height,
                    capabilities.max_image_extent.height,
                ),
            };
            self.surface_extent = extent;
            Ok(extent)
        }
    }
    fn pick_min_image_count(&self) -> Result<u32, RendererError> {
        let capabilities = unsafe {
            self.surface_instance
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
        }?;
        let mut min_image_count = capabilities.min_image_count;
        if (0 < capabilities.max_image_count) && (capabilities.max_image_count < min_image_count) {
            min_image_count = capabilities.max_image_count;
        }

        Ok(min_image_count)
    }
    pub fn create_swap_chain(
        &mut self,
        (extent_width, extent_height): (u32, u32),
    ) -> Result<SwapchainKHR, RendererError> {
        let capabilities = unsafe {
            self.surface_instance
                .get_physical_device_surface_capabilities(self.physical_device, self.surface)
        }?;
        let surface_format = self.surface_format;
        let presentation_mode = self.pick_presentation_mode()?;
        let image_extent = self.get_image_extent((extent_width, extent_height))?;
        let min_image_count = self.pick_min_image_count()?;

        let swapchain_device = ash::khr::swapchain::Device::new(&self.instance, &self.device);
        let swapchain_create_info = SwapchainCreateInfoKHR::default()
            .min_image_count(min_image_count)
            .image_format(surface_format.format)
            .image_color_space(surface_format.color_space)
            .image_extent(image_extent)
            .image_array_layers(1)
            .image_usage(vk::ImageUsageFlags::TRANSFER_DST)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(presentation_mode)
            .surface(self.surface)
            .clipped(true);
        let swapchain = unsafe { swapchain_device.create_swapchain(&swapchain_create_info, None) }?;

        Ok(swapchain)
    }
}

impl Swapchain {
    pub fn new(
        vulkan_context: &VulkanContext,
        swapchain: SwapchainKHR,
    ) -> Result<Self, RendererError> {
        let device =
            ash::khr::swapchain::Device::new(&vulkan_context.instance, &vulkan_context.device);
        let images = unsafe { device.get_swapchain_images(swapchain) }?;
        let next_image_index = 0u32;

        Ok(Self {
            device,
            swapchain,
            images,
            next_image_index,
        })
    }
    fn create_image_views(
        vulkan_context: &VulkanContext,
        swapchain_images: Vec<Image>,
    ) -> Result<Vec<ImageView>, RendererError> {
        let format = vulkan_context.surface_format;
        let create_info = ImageViewCreateInfo::default()
            .view_type(ImageViewType::TYPE_2D)
            .format(format.format)
            .subresource_range(vk::ImageSubresourceRange {
                aspect_mask: ImageAspectFlags::COLOR,
                base_mip_level: 0,
                level_count: 1,
                base_array_layer: 0,
                layer_count: 1,
            });
        let mut image_views: Vec<ImageView> = vec![];
        for image in swapchain_images {
            let create_info = create_info.image(image);
            let image_view =
                unsafe { vulkan_context.device.create_image_view(&create_info, None) }?;
            image_views.push(image_view);
        }

        Ok(image_views)
    }
}

impl Command {
    fn new(vulkan_context: &VulkanContext) -> Result<Self, RendererError> {
        let pool = Self::create_command_pool(vulkan_context)?;
        let buffer = Self::create_command_buffer(vulkan_context, pool)?;

        Ok(Self { pool, buffer })
    }
    fn create_command_pool(vulkan_context: &VulkanContext) -> Result<CommandPool, RendererError> {
        let queue_family_index = vulkan_context.queue_index;
        let create_info = CommandPoolCreateInfo::default()
            .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index);
        let command_pool = unsafe {
            vulkan_context
                .device
                .create_command_pool(&create_info, None)
        }?;

        Ok(command_pool)
    }
    fn create_command_buffer(
        vulkan_context: &VulkanContext,
        pool: CommandPool,
    ) -> Result<CommandBuffer, RendererError> {
        let allocate_info = CommandBufferAllocateInfo::default()
            .command_pool(pool)
            .level(CommandBufferLevel::PRIMARY)
            .command_buffer_count(1);
        let buffers = unsafe {
            vulkan_context
                .device
                .allocate_command_buffers(&allocate_info)
        }?;
        let buffer = buffers[0];

        Ok(buffer)
    }
}

impl Sync {
    fn new(context: &VulkanContext) -> Result<Self, RendererError> {
        let image_available_semaphore = Self::create_semaphore(&context)?;
        let copy_finished_semaphore = Self::create_semaphore(&context)?;
        let in_flight_fence = Self::create_fence(&context)?;

        Ok(Self {
            image_available_semaphore,
            copy_finished_semaphore,
            in_flight_fence,
        })
    }
    fn create_semaphore(context: &VulkanContext) -> Result<vk::Semaphore, RendererError> {
        Ok(unsafe {
            context
                .device
                .create_semaphore(&SemaphoreCreateInfo::default(), None)
        }?)
    }
    fn create_fence(context: &VulkanContext) -> Result<vk::Fence, RendererError> {
        let create_info = FenceCreateInfo::default().flags(vk::FenceCreateFlags::SIGNALED);
        Ok(unsafe { context.device.create_fence(&create_info, None) }?)
    }
}

impl Renderer {
    pub fn new(context: VulkanContext, mut swapchain: Swapchain) -> Result<Self, RendererError> {
        let command = Command::new(&context)?;
        let sync = Sync::new(&context)?;

        Ok(Self {
            context,
            swapchain,
            command,
            sync,
        })
    }
    pub fn render(&mut self, bitmap: Bitmap) -> Result<(), RendererError> {
        self.load_bitmap(bitmap)?;
        self.present()?;
        Ok(())
    }
    fn get_next_image_index(&self) -> Result<u32, RendererError> {
        let (index, suboptimal) = unsafe {
            self.swapchain.device.acquire_next_image(
                self.swapchain.swapchain,
                100,
                self.sync.image_available_semaphore,
                vk::Fence::null(),
            )
        }?;
        Ok(index)
    }
    fn present(&self) -> Result<(), RendererError> {
        let index = self.swapchain.next_image_index;
        let indices = &[index];
        let swapchains = &[self.swapchain.swapchain];
        let semaphores = &[self.sync.copy_finished_semaphore];
        let present_info = vk::PresentInfoKHR::default()
            .swapchains(swapchains)
            .wait_semaphores(semaphores)
            .image_indices(indices);
        let queue = unsafe {
            self.context
                .device
                .get_device_queue(self.context.queue_index, 0)
        };
        let result = unsafe { self.swapchain.device.queue_present(queue, &present_info) }?;
        println!("{:?}", result);
        Ok(())
    }
    fn begin_recording_commands(&self) -> Result<(), RendererError> {
        let begin_info = vk::CommandBufferBeginInfo::default()
            .flags(vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT);
        unsafe {
            self.context
                .device
                .begin_command_buffer(self.command.buffer, &begin_info)
        }?;
        Ok(())
    }
    fn finish_recording_commands(&self) -> Result<(), RendererError> {
        unsafe { self.context.device.end_command_buffer(self.command.buffer) }?;
        Ok(())
    }
    fn submit_queue(&self) -> Result<(), RendererError> {
        let queue = unsafe {
            self.context
                .device
                .get_device_queue(self.context.queue_index, 0)
        };
        let buffer_submit_info = vk::CommandBufferSubmitInfo::default()
            .command_buffer(self.command.buffer)
            .device_mask(0);
        let buffer_submit_infos = &[buffer_submit_info];
        let image_available_info =
            vk::SemaphoreSubmitInfo::default().semaphore(self.sync.image_available_semaphore);
        let image_available_infos = &[image_available_info];
        let copy_finished_info =
            vk::SemaphoreSubmitInfo::default().semaphore(self.sync.copy_finished_semaphore);
        let copy_finished_infos = &[copy_finished_info];
        let submit_info = vk::SubmitInfo2::default()
            .command_buffer_infos(buffer_submit_infos)
            .wait_semaphore_infos(image_available_infos)
            .signal_semaphore_infos(copy_finished_infos);
        unsafe {
            self.context
                .device
                .queue_submit2(queue, &[submit_info], self.sync.in_flight_fence)
        }?;
        Ok(())
    }
    fn transition_image_layout(
        &self,
        old_layout: vk::ImageLayout,
        new_layout: vk::ImageLayout,
        src_access_mask: vk::AccessFlags2,
        dst_access_mask: vk::AccessFlags2,
        src_stage_mask: vk::PipelineStageFlags2,
        dst_stage_mask: vk::PipelineStageFlags2,
    ) -> Result<(), RendererError> {
        let barrier = vk::ImageMemoryBarrier2::default()
            .src_stage_mask(src_stage_mask)
            .src_access_mask(src_access_mask)
            .dst_stage_mask(dst_stage_mask)
            .dst_access_mask(dst_access_mask)
            .old_layout(old_layout)
            .new_layout(new_layout)
            .src_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .dst_queue_family_index(vk::QUEUE_FAMILY_IGNORED)
            .image(self.swapchain.images[self.swapchain.next_image_index as usize])
            .subresource_range(
                vk::ImageSubresourceRange::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );
        let barriers = &[barrier];
        let dependency_info = vk::DependencyInfo::default().image_memory_barriers(barriers);
        unsafe {
            self.context
                .device
                .cmd_pipeline_barrier2(self.command.buffer, &dependency_info)
        };
        Ok(())
    }
    fn copy_buffer_to_swapchain(&self, buffer: vk::Buffer) -> Result<(), RendererError> {
        let region = vk::BufferImageCopy2::default()
            .buffer_offset(0)
            .buffer_row_length(0)
            .buffer_image_height(0)
            .image_subresource(
                vk::ImageSubresourceLayers::default()
                    .aspect_mask(vk::ImageAspectFlags::COLOR)
                    .mip_level(0)
                    .base_array_layer(0)
                    .layer_count(1),
            )
            .image_extent(vk::Extent3D {
                width: self.context.surface_extent.width,
                height: self.context.surface_extent.height,
                depth: 1,
            });
        let regions = &[region];
        let images = &self.swapchain.images;
        self.transition_image_layout(
            vk::ImageLayout::UNDEFINED,
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::AccessFlags2::empty(),
            vk::AccessFlags2::TRANSFER_WRITE_KHR,
            vk::PipelineStageFlags2::empty(),
            vk::PipelineStageFlags2::TRANSFER_KHR,
        )?;
        let copy_info = vk::CopyBufferToImageInfo2::default()
            .src_buffer(buffer)
            .dst_image(images[self.swapchain.next_image_index as usize])
            .dst_image_layout(vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .regions(regions);
        unsafe {
            self.context
                .device
                .cmd_copy_buffer_to_image2(self.command.buffer, &copy_info)
        }
        self.transition_image_layout(
            vk::ImageLayout::TRANSFER_DST_OPTIMAL,
            vk::ImageLayout::PRESENT_SRC_KHR,
            vk::AccessFlags2::TRANSFER_WRITE_KHR,
            vk::AccessFlags2::empty(),
            vk::PipelineStageFlags2::TRANSFER_KHR,
            vk::PipelineStageFlags2::BOTTOM_OF_PIPE_KHR,
        )?;
        Ok(())
    }
    pub fn load_bitmap(&mut self, bitmap: Bitmap) -> Result<(), RendererError> {
        unsafe {
            self.context
                .device
                .wait_for_fences(&[self.sync.in_flight_fence], true, 5)?;
            self.context
                .device
                .reset_fences(&[self.sync.in_flight_fence])?;
        };
        let bitmap_size = u64::from(bitmap.width * bitmap.height * 4);
        let (buffer, memory) = self.create_staging_buffer(bitmap_size)?;
        let data = unsafe {
            self.context
                .device
                .map_memory(memory, 0, bitmap_size, MemoryMapFlags::empty())
        }?;
        unsafe {
            std::ptr::copy_nonoverlapping(
                bitmap.pixels.as_ptr(),
                data as *mut [u8; 4],
                (bitmap.height * bitmap.width) as usize,
            );
            self.context.device.unmap_memory(memory);
        };
        self.swapchain.next_image_index = self.get_next_image_index()?;
        self.begin_recording_commands()?;
        self.copy_buffer_to_swapchain(buffer)?;
        self.finish_recording_commands()?;
        self.submit_queue()?;
        Ok(())
    }
    fn create_staging_buffer(
        &self,
        size: u64,
    ) -> Result<(vk::Buffer, DeviceMemory), RendererError> {
        let create_info = BufferCreateInfo::default()
            .size(size)
            .usage(BufferUsageFlags::TRANSFER_SRC)
            .sharing_mode(SharingMode::EXCLUSIVE);
        let buffer = unsafe { self.context.device.create_buffer(&create_info, None) }?;
        let memory_requirements =
            unsafe { self.context.device.get_buffer_memory_requirements(buffer) };
        let allocate_info = MemoryAllocateInfo::default()
            .allocation_size(memory_requirements.size)
            .memory_type_index(self.get_memory_type_index(
                memory_requirements.memory_type_bits,
                MemoryPropertyFlags::HOST_VISIBLE | MemoryPropertyFlags::HOST_COHERENT,
            )?);
        let buffer_memory = unsafe { self.context.device.allocate_memory(&allocate_info, None) }?;
        unsafe {
            self.context
                .device
                .bind_buffer_memory(buffer, buffer_memory, 0)
        }?;
        Ok((buffer, buffer_memory))
    }
    fn get_memory_type_index(
        &self,
        type_filter: u32,
        properties: MemoryPropertyFlags,
    ) -> Result<u32, RendererError> {
        let memory_properties = unsafe {
            self.context
                .instance
                .get_physical_device_memory_properties(self.context.physical_device)
        };
        let property_index = memory_properties
            .memory_types
            .iter()
            .enumerate()
            .find(|(index, property)| {
                (type_filter & (1 << index)) != 0 && property.property_flags.contains(properties)
            })
            .map(|(index, property)| index as u32)
            .expect("No suitable memory type found");

        Ok(property_index)
    }
}
