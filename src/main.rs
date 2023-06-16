#![allow(deprecated)]
mod check;

use ash::{Entry, Instance};
use std::ffi::CString;
use check::is_extension_supported;

fn main() {
    let entry = unsafe { Entry::load().unwrap() };

    let app_name = CString::new("ExtensionCheck").unwrap();
    let engine_name = CString::new("Ash").unwrap();

    let app_info = ash::vk::ApplicationInfo::builder()
        .application_name(&app_name)
        .application_version(0)
        .engine_name(&engine_name)
        .engine_version(0)
        .api_version(ash::vk::make_version(1, 0, 0));

    let create_info = ash::vk::InstanceCreateInfo::builder()
        .application_info(&app_info)
        .build();

    let instance: Instance = unsafe {
        entry
            .create_instance(&create_info, None)
            .expect("Failed to create instance")
    };

    let physical_devices = unsafe {
        instance
            .enumerate_physical_devices()
            .expect("Failed to enumerate physical devices.")
    };

    let is_nv_ray_tracing_supported = is_extension_supported(&instance, &physical_devices, "VK_NV_ray_tracing");
    let is_khr_acceleration_structure_supported = is_extension_supported(&instance, &physical_devices, "VK_KHR_acceleration_structure");

    println!(
        "VK_NV_ray_tracing support: {}",
        if is_nv_ray_tracing_supported {
            "Yes"
        } else {
            "No"
        }
    );

    println!(
        "VK_KHR_acceleration_structure support: {}",
        if is_khr_acceleration_structure_supported {
            "Yes"
        } else {
            "No"
        }
    );
}
