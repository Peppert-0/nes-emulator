use std::ffi::CStr;

use ash::{self, Entry, Instance, vk::{self, PhysicalDevice, QueueFlags}};

pub struct Renderer {
    pub instance: ash::Instance,
}

#[derive(Debug)]
pub enum RendererError {
    VulkanLoad(ash::LoadingError),
    Vulkan(ash::vk::Result),
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

impl Renderer {
    pub fn new(extensions: &[*const i8]) -> Result<Self, RendererError> {
        let entry = unsafe { Entry::load()? };
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo::default()
            .application_info(&app_info)
            .enabled_extension_names(extensions);

        let instance = unsafe { entry.create_instance(&create_info, None)? };
        let device = Renderer::pick_physical_device(&instance);

        Ok(
            Self {
                instance,
            }
        )
    }
    pub fn pick_physical_device(instance: &Instance) -> Result<PhysicalDevice, RendererError> {
        let physical_devices = unsafe { instance.enumerate_physical_devices()? };
        if physical_devices.is_empty() {
            panic!("No Vulkan-supported device found");
        }

        let mut candidates: Vec<(PhysicalDevice, u16)> = vec![];

        for physical_device in physical_devices {
            let features = unsafe { instance.get_physical_device_features(physical_device) };
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
        let required_extensions: &[&CStr] = &[
            ash::khr::swapchain::NAME,
        ];
        let available_extensions = unsafe { instance.enumerate_device_extension_properties(*device) }?; 
        let mut supports_graphics: bool = false;

        for queue_family in queue_families {
            if queue_family.queue_flags.contains(QueueFlags::GRAPHICS) {
                supports_graphics = true;
                continue;
            } else {}
        };

        let supports_required_extensions = required_extensions.iter().all(|required| {
            available_extensions.iter().any(|available| {
                *required == available.extension_name_as_c_str().unwrap()
            })
        });

        Ok(supports_graphics && supports_required_extensions)
    }
}
