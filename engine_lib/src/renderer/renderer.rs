//use ash::{vk, Entry};

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

pub struct Renderer;

impl Renderer {
    /*pub fn init() -> Renderer {
        let entry = Entry::linked();
        let app_info = vk::ApplicationInfo {
            api_version: vk::make_api_version(0, 1, 0, 0),
            ..Default::default()
        };
        let create_info = vk::InstanceCreateInfo {
            p_application_info: &app_info,
            ..Default::default()
        };
        let instance = unsafe { entry.create_instance(&create_info, None) }
            .expect("failed to create vulkan instance");
        let devices = unsafe { instance.enumerate_physical_devices() }
            .expect("no physical devices available");
        let device = unsafe {
            instance.create_device(
                devices[0],
                &vk::DeviceCreateInfo {
                    ..Default::default()
                },
                None,
            )
        }
        .expect("failed to create logical device");

        todo!()
    }*/
}

impl Renderer {
    pub fn submit(&self, buffer: Box<dyn VertexBuffer>, material: Material) {}

    pub fn render(&self) {}
}
