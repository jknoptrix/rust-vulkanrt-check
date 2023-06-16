use ash::{Instance};
use std::ffi::{CStr};

pub fn is_extension_supported(
    instance: &Instance,
    physical_devices: &[ash::vk::PhysicalDevice],
    _extension_name: &str,
) -> bool {
    physical_devices.iter().any(|physical_device| {
        let device_extensions = unsafe {
            instance
                .enumerate_device_extension_properties(*physical_device)
                .expect("Failed to enumerate device extensions.")
        };

        device_extensions.iter().any(|extension| {
            let extension_name = unsafe { CStr::from_ptr(extension.extension_name.as_ptr()) }
                .to_str()
                .unwrap();
            extension_name == extension_name
        })
    })
}
