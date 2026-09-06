//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/wmi.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// wmi.h - ACPI WMI interface
//
// Copyright (c) 2015 Andrew Lutomirski
//

//
// struct wmi_device - WMI device structure
// @dev: Device associated with this WMI device
// @setable: True for devices implementing the Set Control Method
//
// This represents WMI devices discovered by the WMI driver core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_device {
    pub dev: device,
    pub setable: bool,
}

//
// to_wmi_device() - Helper macro to cast a device to a wmi_device
// @device: device struct
//
// Cast a struct device to a struct wmi_device.
//

//
// struct wmi_buffer - WMI data buffer
// @length: Buffer length in bytes
// @data: Pointer to the buffer content
//
// This structure is used to exchange data with the WMI driver core.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_buffer {
    pub length: usize,
    pub data: *mut c_void,
}

//
// struct wmi_string - WMI string representation
// @length: Size of @chars in bytes
// @chars: UTF16-LE characters with optional nul termination and padding
//
// This structure is used when exchanging string data over the WMI interface.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_string {
    pub length: __le16,
    pub chars: [__le16; ],
    pub __packed: },
    pub length): *const *const *const ssize_t wmi_string_to_utf8s(struct wmi_string str, u8 dst, size_t,
    pub src_length): usize,
    pub min_size): *const *const *const wmi_buffer in, wmi_buffer out, size_t,
    pub in): *const wmi_buffer,
    pub min_size): usize,
    pub in): *const *const int wmidev_set_block(struct wmi_device wdev, u8 instance, struct wmi_buffer,
    pub out): *const *const acpi_buffer in, acpi_buffer,
    pub instance): *mut *mut *mut acpi_object wmidev_block_query(struct wmi_device wdev, u8,
    pub in): *const *const acpi_status wmidev_block_set(struct wmi_device wdev, u8 instance, struct acpi_buffer,
    pub wdev): *mut u8 wmidev_instance_count(struct wmi_device,
//
// struct wmi_driver - WMI driver structure
// @driver: Driver model structure
// @id_table: List of WMI GUIDs supported by this driver
// @min_event_size: Minimum event payload size supported by this driver
// @no_singleton: Driver can be instantiated multiple times
// @probe: Callback for device binding
// @remove: Callback for device unbinding
// @shutdown: Callback for device shutdown
// @notify: Callback for receiving WMI events (deprecated)
// @notify_new: Callback for receiving WMI events
//
// This represents WMI drivers which handle WMI devices. The data inside the buffer
// passed to the @notify_new callback is guaranteed to be aligned on a 8-byte boundary.
// The minimum supported size for said buffer can be specified using @min_event_size.
// WMI drivers that still use the deprecated @notify callback can still set @min_event_size
// to 0 in order to signal that they support WMI events which provide no event data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wmi_driver {
    pub driver: device_driver,
    pub id_table: *const wmi_device_id,
    pub min_event_size: usize,
    pub no_singleton: bool,
    pub context): *const *const *const int (probe)(struct wmi_device wdev, void,
    pub wdev): *mut *mut void (remove)(struct wmi_device,
    pub wdev): *mut *mut void (shutdown)(struct wmi_device,
    pub data): *mut *mut *mut void (notify)(struct wmi_device device, union acpi_object,
    pub data): *const *const *const void (notify_new)(struct wmi_device device, struct wmi_buffer,
}

//
// to_wmi_driver() - Helper macro to cast a driver to a wmi_driver
// @drv: driver struct
//
// Cast a struct device_driver to a struct wmi_driver.
//

extern "C" {
    pub fn __wmi_driver_register(driver: *mut wmi_driver, owner: *mut module) -> int __must_check;
}
extern "C" {
    pub fn wmi_driver_unregister(driver: *mut wmi_driver);
}
//
// wmi_driver_register() - Helper macro to register a WMI driver
// @driver: wmi_driver struct
//
// Helper macro for registering a WMI driver. It automatically passes
// THIS_MODULE to the underlying function.
//

//
// module_wmi_driver() - Helper macro to register/unregister a WMI driver
// @__wmi_driver: wmi_driver struct
//
// Helper macro for WMI drivers which do not do anything special in module
// init/exit. This eliminates a lot of boilerplate. Each module may only
// use this macro once, and calling it replaces module_init() and module_exit().
//

