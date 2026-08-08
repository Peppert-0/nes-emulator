use ash::{self, vk, Entry};

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

        Ok(
            Self {
                instance,
            }
        )
    }
}
