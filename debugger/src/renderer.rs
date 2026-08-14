use std::{ffi::{CStr, CString, c_char}, fmt::format};

use ash::{self, Entry, Instance, vk::{self, AllocationCallbacks, Device, DeviceCreateInfo, DeviceQueueCreateInfo, Extent2D, Handle, PhysicalDevice, PhysicalDeviceFeatures, PresentModeKHR, QueueFlags, SurfaceFormatKHR, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR}};

pub struct VulkanContext {
    pub entry: ash::Entry,
    pub instance: ash::Instance,
    pub surface: SurfaceKHR,
    pub surface_instance: ash::khr::surface::Instance,
    pub physical_device: PhysicalDevice,
    pub device: ash::Device,
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
    pub fn new(entry: Entry, instance: Instance, window_surface: SurfaceKHR) -> Result<Self, RendererError> {
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let physical_device = VulkanContext::pick_physical_device(&instance, &surface_instance, &window_surface)?;
        let device = VulkanContext::create_logical_device(&instance, &physical_device, &surface_instance, &window_surface)?;

        Ok(
            Self {
                entry,
                instance,
                surface: window_surface,
                surface_instance,
                physical_device,
                device,
            }
        )
    }
    pub fn create_instance(entry: &Entry, extensions: Vec<String>) -> Result<Instance, RendererError> {
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let extension_names: Vec<CString> = extensions
            .iter()
            .map(|name| CString::new(name.as_str()))
            .collect::<Result<_, _>>()?;

        let extension_ptrs: Vec<*const c_char> = extension_names
            .iter()
            .map(|name| name.as_ptr())
            .collect();
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(&extension_ptrs);

        Ok(unsafe { entry.create_instance(&create_info, None)? })
    }
    pub fn pick_physical_device(instance: &Instance, surface_instance: &ash::khr::surface::Instance, surface: &SurfaceKHR) -> Result<PhysicalDevice, RendererError> {
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

            if VulkanContext::device_is_suitable(&instance, &physical_device, surface_instance, surface)? {
                candidates.push((physical_device, score));
            }
        }

        let device = if !candidates.is_empty() {
            candidates
            .into_iter()
            .max_by_key(|(_, score)| *score).unwrap()
        } else {
            panic!("No suitable Vulkan-supported device found");
        };

        Ok(device.0)
    }
    pub fn device_is_suitable(instance: &Instance, device: &PhysicalDevice, surface_instance: &ash::khr::surface::Instance, surface: &SurfaceKHR) -> Result<bool, RendererError> {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*device) };
        let features = unsafe { instance.get_physical_device_features(*device) };
        let required_extensions: &[&CStr] = &[
            ash::khr::swapchain::NAME,
        ];
        let available_extensions = unsafe { instance.enumerate_device_extension_properties(*device) }?; 

        let supports_required_queue_family_properties = queue_families.iter().enumerate()
            .any(|(index, family)| {
                family.queue_flags.contains(QueueFlags::GRAPHICS) &&
                unsafe { surface_instance.get_physical_device_surface_support(*device, index as u32, *surface) }
                .unwrap()
            });
        let supports_required_extensions = required_extensions.iter().all(|required| {
            available_extensions.iter().any(|available| {
                *required == available.extension_name_as_c_str().unwrap()
            })
        });

        Ok(supports_required_queue_family_properties && supports_required_extensions)
    }
    pub fn create_logical_device(instance: &Instance, physical_device: &PhysicalDevice, surface_instance: &ash::khr::surface::Instance, surface: &SurfaceKHR) -> Result<ash::Device, RendererError> {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
        let queue_family_index = queue_families.iter().enumerate()
            .position(|(index, family)| {
                family.queue_flags.contains(QueueFlags::GRAPHICS) &&
                unsafe { surface_instance.get_physical_device_surface_support(*physical_device, index as u32, *surface) }
                .unwrap()
            })
            .map(|index| index as u32)
            .unwrap();
        let queue_priority = [0.5f32];
        let device_queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(queue_family_index)
            .queue_priorities(&queue_priority);
        let device_queue_create_infos = [device_queue_create_info];
        let device_features = PhysicalDeviceFeatures::default();
        let enabled_extensions: &[&CStr] = &[
            ash::khr::swapchain::NAME,
        ];
        let enabled_extensions_ptrs = enabled_extensions.as_c_char_array();
        let device_create_info = DeviceCreateInfo::default()
            .enabled_features(&device_features)
            .enabled_extension_names(&enabled_extensions_ptrs)
            .queue_create_infos(&device_queue_create_infos);
        Ok (unsafe { instance.create_device(*physical_device, &device_create_info, None) }?)
    }
    fn pick_surface_format(&self) -> Result<SurfaceFormatKHR, RendererError> {
        let available_formats = unsafe { self.surface_instance.get_physical_device_surface_formats(self.physical_device, self.surface) }?;
        let format = available_formats.iter().find(|format| {
            format.format == vk::Format::B8G8R8A8_SRGB &&
            format.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        }).unwrap_or(&available_formats[0]);

        Ok(*format)
    }
    fn pick_presentation_mode(&self) -> Result<PresentModeKHR, RendererError> {
        let available_presentation_modes = unsafe { self.surface_instance.get_physical_device_surface_present_modes(self.physical_device, self.surface) }?;
        let presentation_mode = available_presentation_modes.iter().find(|mode| {
            **mode == vk::PresentModeKHR::MAILBOX
        }).unwrap_or(&vk::PresentModeKHR::FIFO);

        Ok(*presentation_mode)
    }
    pub fn get_image_extent(&self, (width, height): (u32, u32)) -> Result<Extent2D, RendererError> {
        let capabilities = unsafe { self.surface_instance.get_physical_device_surface_capabilities(self.physical_device, self.surface) }?;
        if capabilities.current_extent.width != u32::MAX {
            Ok(capabilities.current_extent)
        } else {
            Ok( Extent2D {
            width : width.clamp(capabilities.min_image_extent.width, capabilities.max_image_extent.width),
            height : height.clamp(capabilities.min_image_extent.height, capabilities.max_image_extent.height),
            })
        }
    }
    fn pick_min_image_count(&self) -> Result<u32, RendererError> {
        let capabilities = unsafe { self.surface_instance.get_physical_device_surface_capabilities(self.physical_device, self.surface) }?;
        let mut min_image_count = capabilities.min_image_count;
        if (0 < capabilities.max_image_count) && (capabilities.max_image_count < min_image_count) {
            min_image_count = capabilities.max_image_count;
        }

        Ok(min_image_count)
    }
    pub fn create_swap_chain(&self, (extent_width, extent_height): (u32, u32)) -> Result<SwapchainKHR, RendererError> {
        let capabilities = unsafe { self.surface_instance.get_physical_device_surface_capabilities(self.physical_device, self.surface) }?;
        let surface_format = self.pick_surface_format()?;
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
            .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
            .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
            .pre_transform(capabilities.current_transform)
            .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
            .present_mode(presentation_mode)
            .clipped(true);
        let swapchain = unsafe { swapchain_device.create_swapchain(&swapchain_create_info, None) }?;

        Ok(swapchain)
    }
}
