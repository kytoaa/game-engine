pub(crate) fn get_required_extensions(
    handle: &dyn winit::raw_window_handle::HasDisplayHandle,
) -> Vec<*const i8> {
    let base_extensions =
        ash_window::enumerate_required_extensions(handle.display_handle().unwrap().into()).unwrap();

    let mut extensions = vec![];

    for extension in base_extensions {
        extensions.push(*extension);
    }

    if super::validation_layers::ENABLE_VALIDATION_LAYERS {
        extensions.push(
            ash::vk::EXT_DEBUG_UTILS_NAME.to_bytes().first().unwrap() as *const u8 as *const i8,
        );
        for extension in &extensions {
            println!(
                "{}",
                unsafe { std::ffi::CStr::from_ptr(*extension) }
                    .to_str()
                    .unwrap()
            )
        }
    }

    extensions
}
