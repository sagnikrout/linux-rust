//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/platform_device.h
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
// platform_device.h - generic, centralized driver model
//
// Copyright (c) 2001-2003 Patrick Mochel <mochel@osdl.org>
//
// See Documentation/driver-api/driver-model/ for more information.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_device {
    pub name: *const c_char,
    pub id: c_int,
    pub id_auto: bool,
    pub dev: device,
    pub platform_dma_mask: u64,
    pub dma_parms: device_dma_parameters,
    pub num_resources: u32,
    pub resource: *mut resource,
    pub id_entry: *const platform_device_id,
// MFD cell pointer
    pub mfd_cell: *mut mfd_cell,
// arch specific additions
    pub archdata: pdev_archdata,
}

extern "C" {
    pub fn platform_device_register(: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn platform_device_unregister(: *mut platform_device);
}

extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}
extern "C" {
    pub fn IOMEM_ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn platform_get_irq(: *mut platform_device, int: unsigned) -> c_int;
}
extern "C" {
    pub fn platform_get_irq_optional(: *mut platform_device, int: unsigned) -> c_int;
}
extern "C" {
    pub fn platform_irq_count(: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn platform_get_irq_byname(: *mut platform_device, : *const c_char) -> c_int;
}
extern "C" {
    pub fn platform_add_devices(: *mut platform_device, _arg: c_int) -> c_int;
}
//
// struct platform_device_info - set of parameters for creating a platform device
// @parent: parent device for the new platform device.
// @fwnode: firmware node associated with the device.
// @of_node_reused: indicates that device tree node associated with the device
// is shared with another device, typically its ancestor. Setting this to
// %true prevents the device from being matched via the OF match table,
// and stops the device core from automatically binding pinctrl
// configuration to avoid disrupting the other device.
// @name: name of the device.
// @id: instance ID of the device. Use %PLATFORM_DEVID_NONE if there is only
// one instance of the device, or %PLATFORM_DEVID_AUTO to let the
// kernel automatically assign a unique instance ID.
// @res: set of resources to attach to the device.
// @num_res: number of entries in @res.
// @data: device-specific data for this platform device.
// @size_data: size of device-specific data.
// @dma_mask: DMA mask for the device.
// @swnode: a secondary software node to be attached to the device. The node
// will be automatically registered and its lifetime tied to the platform
// device if it is not registered yet.
// @properties: a set of software properties for the device. If provided,
// a managed software node will be automatically created and
// assigned to the device. The properties array must be terminated
// with a sentinel entry. Specifying both @properties and @swnode is not
// allowed.
//
// This structure is used to hold information needed to create and register
// a platform device using platform_device_register_full().
//
// platform_device_register_full() makes deep copies of @name, @res, @data and
// @properties, so the caller does not need to keep them after registration.
// If the registration is performed during initialization, these can be marked
// as __initconst.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_device_info {
    pub parent: *mut device,
    pub fwnode: *mut fwnode_handle,
    pub of_node_reused: bool,
    pub name: *const c_char,
    pub id: c_int,
    pub res: *const resource,
    pub num_res: c_uint,
    pub data: *const c_void,
    pub size_data: usize,
    pub dma_mask: u64,
    pub swnode: *const software_node,
    pub properties: *const property_entry,
}

//
// platform_device_register_resndata - add a platform-level device with
// resources and platform-specific data
//
// @parent: parent device for the device we're adding
// @name: base name of the device we're adding
// @id: instance id
// @res: set of resources that needs to be allocated for the device
// @num: number of resources
// @data: platform specific data for this platform device
// @size: size of platform specific data
//
// Returns &struct platform_device pointer on success, or ERR_PTR() on error.
//
extern "C" {
    pub fn platform_device_register_full(_arg: &pdevinfo) -> return;
}
//
// platform_device_register_simple - add a platform-level device and its resources
// @name: base name of the device we're adding
// @id: instance id
// @res: set of resources that needs to be allocated for the device
// @num: number of resources
//
// This function creates a simple platform device that requires minimal
// resource and memory management. Canned release function freeing memory
// allocated for the device allows drivers using such devices to be
// unloaded without waiting for the last reference to the device to be
// dropped.
//
// This interface is primarily intended for use with legacy drivers which
// probe hardware directly.  Because such drivers create sysfs device nodes
// themselves, rather than letting system infrastructure handle such device
// enumeration tasks, they don't fully conform to the Linux driver model.
// In particular, when such drivers are built as modules, they can't be
// "hotplugged".
//
// Returns &struct platform_device pointer on success, or ERR_PTR() on error.
//
// platform_device_register_data - add a platform-level device with platform-specific data
// @parent: parent device for the device we're adding
// @name: base name of the device we're adding
// @id: instance id
// @data: platform specific data for this platform device
// @size: size of platform specific data
//
// This function creates a simple platform device that requires minimal
// resource and memory management. Canned release function freeing memory
// allocated for the device allows drivers using such devices to be
// unloaded without waiting for the last reference to the device to be
// dropped.
//
// Returns &struct platform_device pointer on success, or ERR_PTR() on error.
//
extern "C" {
    pub fn platform_device_add(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn platform_device_del(pdev: *mut platform_device);
}
extern "C" {
    pub fn platform_device_put(pdev: *mut platform_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct platform_driver {
    pub ): *mut *mut int (probe)(struct platform_device,
    pub ): *mut *mut void (remove)(struct platform_device,
    pub ): *mut *mut void (shutdown)(struct platform_device,
    pub state): *mut *mut *mut int (suspend)(struct platform_device , pm_message_t,
    pub ): *mut *mut int (resume)(struct platform_device,
    pub driver: device_driver,
    pub id_table: *const platform_device_id,
    pub prevent_deferred_probe: bool,
//
// For most device drivers, no need to care about this flag as long as
// all DMAs are handled through the kernel DMA API. For some special
// ones, for example VFIO drivers, they know how to manage the DMA
// themselves and set this flag so that the IOMMU layer will allow them
// to setup and manage their own I/O address space.
//
    pub driver_managed_dma: bool,
}

//
// use a macro to avoid include chaining to get THIS_MODULE
//

extern "C" {
    pub fn platform_driver_unregister(: *mut platform_driver);
}
// non-hotpluggable platform devices may use this so that probe() and
// its support may live in __init sections, conserving runtime memory.
//

extern "C" {
    pub fn dev_get_drvdata(_arg: &pdev->dev) -> return;
}
// module_platform_driver() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

// builtin_platform_driver() - Helper macro for builtin drivers that
// don't do anything special in driver init.  This eliminates some
// boilerplate.  Each driver may only use this macro once, and
// calling it replaces device_initcall().  Note this is meant to be
// a parallel of module_platform_driver() above, but w/o _exit stuff.
//

// module_platform_driver_probe() - Helper macro for drivers that don't do
// anything special in module init/exit.  This eliminates a lot of
// boilerplate.  Each module may only use this macro once, and
// calling it replaces module_init() and module_exit()
//

// builtin_platform_driver_probe() - Helper macro for drivers that don't do
// anything special in device init.  This eliminates some boilerplate.  Each
// driver may only use this macro once, and using it replaces device_initcall.
// This is meant to be a parallel of module_platform_driver_probe above, but
// without the __exit parts.
//

extern "C" {
    pub fn platform_pm_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn platform_pm_resume(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn platform_pm_freeze(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn platform_pm_thaw(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn platform_pm_poweroff(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn platform_pm_restore(dev: *mut device) -> c_int;
}

// Macro flag: #define USE_PLATFORM_PM_SLEEP_OPS

//
// REVISIT: This stub is needed for all non-SuperH users of early platform
// drivers. It should go away once we introduce the new platform_device-based
// early driver framework.
//

// For now only SuperH uses it
extern "C" {
    pub fn early_platform_cleanup();
}
