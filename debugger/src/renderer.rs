use std::ffi::{CStr, CString, c_char};

use ash::{self, Entry, Instance, vk::{self, AllocationCallbacks, Device, DeviceCreateInfo, DeviceQueueCreateInfo, Handle, PhysicalDevice, PhysicalDeviceFeatures, QueueFlags}};

pub struct Renderer {
    pub instance: ash::Instance,
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

impl Renderer {
    pub fn new(extensions: Vec<String>) -> Result<Self, RendererError> {
        let instance = Renderer::create_instance(extensions)?;
        let physical_device = Renderer::pick_physical_device(&instance)?;
        let device = Renderer::create_logical_device(&instance, &physical_device);

        Ok(
            Self {
                instance,
            }
        )
    }
    pub fn create_instance(extensions: Vec<String>) -> Result<Instance, RendererError> {
        let entry = unsafe { Entry::load()? };
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
    pub fn pick_physical_device(instance: &Instance) -> Result<PhysicalDevice, RendererError> {
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

            if Renderer::device_is_suitable(&instance, &physical_device)? {
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
    pub fn device_is_suitable(instance: &Instance, device: &PhysicalDevice) -> Result<bool, RendererError> {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*device) };
        let features = unsafe { instance.get_physical_device_features(*device) };
        let required_extensions: &[&CStr] = &[
            ash::khr::swapchain::NAME,
        ];
        let available_extensions = unsafe { instance.enumerate_device_extension_properties(*device) }?; 

        let supports_graphics = queue_families.iter().any(|family| {
            family.queue_flags.contains(QueueFlags::GRAPHICS)
        });
        let supports_required_extensions = required_extensions.iter().all(|required| {
            available_extensions.iter().any(|available| {
                *required == available.extension_name_as_c_str().unwrap()
            })
        });

        Ok(supports_graphics && supports_required_extensions)
    }
    pub fn create_logical_device(instance: &Instance, physical_device: &PhysicalDevice) -> Result<ash::Device, RendererError> {
        let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*physical_device) };
        let graphics_index = queue_families.iter().position(|family| {
            family.queue_flags.contains(QueueFlags::GRAPHICS)
        })
            .map(|index| index as u32)
            .unwrap();
        let queue_priority = [0.5f32];
        let device_queue_create_info = DeviceQueueCreateInfo::default()
            .queue_family_index(graphics_index)
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
}
