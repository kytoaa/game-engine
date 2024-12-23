use ash::Instance;

pub struct QueueFamilyIndices {
    pub graphics_family: Option<u32>,
}
impl QueueFamilyIndices {
    pub fn is_complete(&self) -> bool {
        self.graphics_family.is_some()
    }
}

pub fn get_physical_device(instance: &Instance) -> ash::vk::PhysicalDevice {
    let (_, device) = unsafe { instance.enumerate_physical_devices() }
        .expect("failed to get devices")
        .into_iter()
        .filter_map(|d| {
            let score = physical_device_score(instance, &d);
            if score == 0 {
                None
            } else {
                Some((score, d))
            }
        })
        .max_by(|(s1, _), (s2, _)| s1.cmp(s2))
        .expect("failed to get device");
    device
}

fn physical_device_score(instance: &Instance, device: &ash::vk::PhysicalDevice) -> u32 {
    let device_properties = unsafe { instance.get_physical_device_properties(*device) };
    let _device_features = unsafe { instance.get_physical_device_features(*device) };

    let mut score = match device_properties.device_type {
        ash::vk::PhysicalDeviceType::DISCRETE_GPU => 10000,
        ash::vk::PhysicalDeviceType::INTEGRATED_GPU => 500,
        _ => 0,
    };

    score += device_properties.limits.max_compute_shared_memory_size;

    /*if device_features.geometry_shader == 0 {
        score = 0;
    }*/
    if !find_queue_families(instance, device).is_complete() {
        score = 0;
    }

    score
}

pub fn find_queue_families(
    instance: &Instance,
    device: &ash::vk::PhysicalDevice,
) -> QueueFamilyIndices {
    let mut indices = QueueFamilyIndices {
        graphics_family: None,
    };

    let queue_families = unsafe { instance.get_physical_device_queue_family_properties(*device) };

    for (i, family) in queue_families.iter().enumerate() {
        if family.queue_flags.contains(ash::vk::QueueFlags::GRAPHICS) {
            indices.graphics_family = Some(i as u32);
        }
        if indices.is_complete() {
            break;
        }
    }

    indices
}

pub fn create_logical_device(
    instance: &Instance,
    device: &ash::vk::PhysicalDevice,
    indices: &QueueFamilyIndices,
) -> ash::Device {
    let queue_priority = 1.0;

    let queue_create_info = ash::vk::DeviceQueueCreateInfo {
        s_type: ash::vk::StructureType::DEVICE_QUEUE_CREATE_INFO,
        queue_family_index: indices.graphics_family.unwrap(),
        queue_count: 1,
        p_queue_priorities: &queue_priority,
        ..Default::default()
    };

    let device_features = ash::vk::PhysicalDeviceFeatures::default();

    let create_info = {
        let mut create_info = ash::vk::DeviceCreateInfo {
            s_type: ash::vk::StructureType::DEVICE_CREATE_INFO,
            p_queue_create_infos: &queue_create_info,
            queue_create_info_count: 1,
            p_enabled_features: &device_features,

            enabled_extension_count: 0,

            ..Default::default()
        };
        if super::validation_layers::ENABLE_VALIDATION_LAYERS {
            let layers = super::validation_layers::get_validation_layers();
            create_info.enabled_layer_count = layers.0 as u32;
            create_info.pp_enabled_layer_names = layers.1;
        }
        create_info
    };

    unsafe { instance.create_device(*device, &create_info, None) }
        .expect("could not create logical device")
}
