//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/drm_module.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: MIT

//
// DOC: overview
//
// This library provides helpers registering DRM drivers during module
// initialization and shutdown. The provided helpers act like bus-specific
// module helpers, such as module_pci_driver(), but respect additional
// parameters that control DRM driver registration.
//
// Below is an example of initializing a DRM driver for a device on the
// PCI bus.
//
// .. code-block:: c
//
// struct pci_driver my_pci_drv = {
// };
//
// drm_module_pci_driver(my_pci_drv);
//
// The generated code will test if DRM drivers are enabled and register
// the PCI driver my_pci_drv. For more complex module initialization, you
// can still use module_init() and module_exit() in your driver.
//
// PCI drivers
//
extern "C" {
    pub fn pci_register_driver(_arg: pci_drv) -> return;
}
//
// drm_module_pci_driver - Register a DRM driver for PCI-based devices
// @__pci_drv: the PCI driver structure
//
// Registers a DRM driver for devices on the PCI bus. The helper
// macro behaves like module_pci_driver() but tests the state of
// drm_firmware_drivers_only(). For more complex module initialization,
// use module_init() and module_exit() directly.
//
// Each module may only use this macro once. Calling it replaces
// module_init() and module_exit().
//

extern "C" {
    pub fn pci_register_driver(_arg: pci_drv) -> return;
}
//
// drm_module_pci_driver_if_modeset - Register a DRM driver for PCI-based devices
// @__pci_drv: the PCI driver structure
// @__modeset: an additional parameter that disables the driver
//
// This macro is deprecated and only provided for existing drivers. For
// new drivers, use drm_module_pci_driver().
//
// Registers a DRM driver for devices on the PCI bus. The helper macro
// behaves like drm_module_pci_driver() with an additional driver-specific
// flag. If __modeset is 0, the driver has been disabled, if __modeset is
// -1 the driver state depends on the global DRM state. For all other
// values, the PCI driver has been enabled. The default should be -1.
//

//
// Platform drivers
//
extern "C" {
    pub fn platform_driver_register(_arg: platform_drv) -> return;
}
//
// drm_module_platform_driver - Register a DRM driver for platform devices
// @__platform_drv: the platform driver structure
//
// Registers a DRM driver for devices on the platform bus. The helper
// macro behaves like module_platform_driver() but tests the state of
// drm_firmware_drivers_only(). For more complex module initialization,
// use module_init() and module_exit() directly.
//
// Each module may only use this macro once. Calling it replaces
// module_init() and module_exit().
//

