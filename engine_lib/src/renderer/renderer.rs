use super::vulkan;
use ash::{vk, Entry};
use winit::raw_window_handle::{HasDisplayHandle, HasWindowHandle, RawWindowHandle};

//#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct RenderVert {
    //#[format(R32G32_SFLOAT)]
    position: [f32; 2],
}
impl RenderVert {
    pub fn new(position: [f32; 2]) -> RenderVert {
        RenderVert { position }
    }
}

pub struct ShaderHandle;
pub struct ShaderParam;

pub struct Material {
    shader: ShaderHandle,
    params: Vec<ShaderParam>,
}

pub trait VertexBuffer: Iterator<Item = RenderVert> {}

pub struct Renderer {
    instance: ash::Instance,
    device: ash::Device,
    graphics_queue: ash::vk::Queue,
}

impl Renderer {
    pub fn init(window: &winit::window::Window) -> Renderer {
        let display_handle = window.display_handle().unwrap();
        let entry = unsafe { Entry::load() }.expect("could not load vulkan library/DLL");

        if vulkan::validation_layers::ENABLE_VALIDATION_LAYERS
            && !vulkan::validation_layers::check_validation_layer_support(&entry)
        {
            panic!("validation layers not available");
        }

        let extensions = vulkan::extension_layers::get_required_extensions(&display_handle);

        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = {
            let mut create_info = vk::InstanceCreateInfo {
                p_application_info: &app_info,
                enabled_extension_count: extensions.len() as u32,
                pp_enabled_extension_names: extensions.first().unwrap(),
                ..Default::default()
            };
            if vulkan::validation_layers::ENABLE_VALIDATION_LAYERS {
                let validation_layers = vulkan::validation_layers::get_validation_layers();

                create_info.enabled_layer_count = validation_layers.0 as u32;
                create_info.pp_enabled_layer_names = validation_layers.1;
            }
            create_info
        };

        let instance = unsafe { entry.create_instance(&create_info, None) }
            .expect("failed to create vulkan instance");

        let device = vulkan::devices::get_physical_device(&instance);

        let indices = vulkan::devices::find_queue_families(&instance, &device);

        let device = vulkan::devices::create_logical_device(&instance, &device, &indices);
        let graphics_queue =
            unsafe { device.get_device_queue(indices.graphics_family.unwrap(), 0) };

        let hwnd = match window.window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(handle) => handle.hwnd.get(),
            _ => panic!("not running on windows :D"),
        };
        let create_info = ash::vk::Win32SurfaceCreateInfoKHR {
            s_type: ash::vk::StructureType::WIN32_SURFACE_CREATE_INFO_KHR,
            hwnd,
            hinstance: todo!(),
            ..Default::default()
        };

        todo!()
    }
}

impl Renderer {
    pub fn submit<T: VertexBuffer>(&self, buffer: T, material: Material) {}

    pub fn render(&self) {}
}

impl Drop for Renderer {
    fn drop(&mut self) {}
}
