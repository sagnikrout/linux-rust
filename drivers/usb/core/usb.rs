//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/core/usb.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Released under the GPLv2 only.
//

// Functions local to drivers/usb/core/
extern "C" {
    pub fn usb_create_sysfs_dev_files(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_remove_sysfs_dev_files(dev: *mut usb_device);
}
extern "C" {
    pub fn usb_create_sysfs_intf_files(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_remove_sysfs_intf_files(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_update_wireless_status_attr(intf: *mut usb_interface) -> c_int;
}
extern "C" {
    pub fn usb_remove_ep_devs(endpoint: *mut usb_host_endpoint);
}
extern "C" {
    pub fn usb_release_interface_cache(ref: *mut kref);
}
extern "C" {
    pub fn usb_disable_device(dev: *mut usb_device, skip_ep0: c_int);
}
extern "C" {
    pub fn usb_deauthorize_device(: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_authorize_device(: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_deauthorize_interface(: *mut usb_interface);
}
extern "C" {
    pub fn usb_authorize_interface(: *mut usb_interface);
}
extern "C" {
    pub fn usb_detect_quirks(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_detect_interface_quirks(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_release_quirk_list();
}
extern "C" {
    pub fn usb_remove_device(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_set_isoch_delay(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_get_bos_descriptor(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_release_bos_descriptor(dev: *mut usb_device);
}
extern "C" {
    pub fn usb_set_configuration(dev: *mut usb_device, configuration: c_int) -> c_int;
}
extern "C" {
    pub fn usb_choose_configuration(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_generic_driver_probe(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_generic_driver_disconnect(udev: *mut usb_device);
}
// SuperSpeed power is in 8 mA units; others are in 2 mA units
extern "C" {
    pub fn usb_kick_hub_wq(dev: *mut usb_device);
}
extern "C" {
    pub fn usb_forced_unbind_intf(intf: *mut usb_interface);
}
extern "C" {
    pub fn usb_unbind_and_rebind_marked_interfaces(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_device_is_owned(udev: *mut usb_device) -> bool;
}
extern "C" {
    pub fn usb_hub_init() -> c_int;
}
extern "C" {
    pub fn usb_hub_cleanup();
}
extern "C" {
    pub fn usb_major_init() -> c_int;
}
extern "C" {
    pub fn usb_major_cleanup();
}
extern "C" {
    pub fn usb_device_supports_lpm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_port_disable(udev: *mut usb_device) -> c_int;
}

extern "C" {
    pub fn usb_suspend(dev: *mut device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_resume(dev: *mut device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_resume_complete(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usb_port_suspend(dev: *mut usb_device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_port_resume(dev: *mut usb_device, msg: pm_message_t) -> c_int;
}
extern "C" {
    pub fn usb_autosuspend_device(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_autoresume_device(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_remote_wakeup(dev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_runtime_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usb_runtime_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usb_runtime_idle(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn usb_enable_usb2_hardware_lpm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usb_disable_usb2_hardware_lpm(udev: *mut usb_device) -> c_int;
}
extern "C" {
    pub fn usbfs_notify_suspend(udev: *mut usb_device);
}
extern "C" {
    pub fn usbfs_notify_resume(udev: *mut usb_device);
}

extern "C" {
    pub fn is_usb_device_driver(drv: *const device_driver) -> bool;
}
// for labeling diagnostics
// sysfs stuff
// usbfs stuff
extern "C" {
    pub fn usb_devio_init() -> c_int;
}
extern "C" {
    pub fn usb_devio_cleanup();
}
//
// Firmware specific cookie identifying a port's location. '0' == no location
// data available
//
pub type usb_port_location_t = u32;
// internal notify stuff
extern "C" {
    pub fn usb_notify_add_device(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_notify_remove_device(udev: *mut usb_device);
}
extern "C" {
    pub fn usb_notify_add_bus(ubus: *mut usb_bus);
}
extern "C" {
    pub fn usb_notify_remove_bus(ubus: *mut usb_bus);
}

extern "C" {
    pub fn usb_acpi_register() -> c_int;
}
extern "C" {
    pub fn usb_acpi_unregister();
}

