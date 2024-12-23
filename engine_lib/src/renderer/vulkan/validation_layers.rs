use ash::Entry;

pub(crate) const ENABLE_VALIDATION_LAYERS: bool = cfg!(debug_assertions);

/// all validation layers in use as null-terminated strings
const VALIDATION_LAYERS: [&'static str; 1] = ["VK_LAYER_KHRONOS_validation\0"];

pub(crate) fn check_validation_layer_support(entry: &Entry) -> bool {
    let available_layers = match unsafe { entry.enumerate_instance_layer_properties() } {
        Ok(l) => l,
        Err(_) => return false,
    };

    for layer_name in VALIDATION_LAYERS {
        if available_layers
            .iter()
            .find(|layer| {
                layer.layer_name_as_c_str().unwrap().to_str().unwrap()
                    == &layer_name[..layer_name.len() - 1]
            })
            .is_none()
        {
            return false;
        }
    }

    true
}

pub(crate) const fn get_validation_layers() -> (usize, *const *const i8) {
    (
        VALIDATION_LAYERS.len(),
        VALIDATION_LAYERS.as_ptr() as *const *const i8,
    )
}
