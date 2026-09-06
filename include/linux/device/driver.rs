//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device/driver.h
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
// The driver-specific portions of the driver model
//
// Copyright (c) 2001-2003 Patrick Mochel <mochel@osdl.org>
// Copyright (c) 2004-2009 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2008-2009 Novell Inc.
// Copyright (c) 2012-2019 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
// Copyright (c) 2012-2019 Linux Foundation
//
// See Documentation/driver-api/driver-model/ for more information.
//

//
// enum probe_type - device driver probe type to try
// Device drivers may opt in for special handling of their
// respective probe routines. This tells the core what to
// expect and prefer.
//
// @PROBE_DEFAULT_STRATEGY: Used by drivers that work equally well
// whether probed synchronously or asynchronously.
// @PROBE_PREFER_ASYNCHRONOUS: Drivers for "slow" devices which
// probing order is not essential for booting the system may
// opt into executing their probes asynchronously.
// @PROBE_FORCE_SYNCHRONOUS: Use this to annotate drivers that need
// their probe routines to run synchronously with driver and
// device registration (with the exception of -EPROBE_DEFER
// handling - re-probing always ends up being done asynchronously).
//
// Note that the end goal is to switch the kernel to use asynchronous
// probing by default, so annotating drivers with
// %PROBE_PREFER_ASYNCHRONOUS is a temporary measure that allows us
// to speed up boot process while we are validating the rest of the
// drivers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum probe_type {
    PROBE_DEFAULT_STRATEGY,
    PROBE_PREFER_ASYNCHRONOUS,
    PROBE_FORCE_SYNCHRONOUS,
}

//
// struct device_driver - The basic device driver structure
// @name:	Name of the device driver.
// @bus:	The bus which the device of this driver belongs to.
// @owner:	The module owner.
// @mod_name:	Used for built-in modules.
// @suppress_bind_attrs: Disables bind/unbind via sysfs.
// @probe_type:	Type of the probe (synchronous or asynchronous) to use.
// @of_match_table: The open firmware table.
// @acpi_match_table: The ACPI match table.
// @probe:	Called to query the existence of a specific device,
// whether this driver can work with it, and bind the driver
// to a specific device.
// @sync_state:	Called to sync device state to software state after all the
// state tracking consumers linked to this device (present at
// the time of late_initcall) have successfully bound to a
// driver. If the device has no consumers, this function will
// be called at late_initcall_sync level. If the device has
// consumers that are never bound to a driver, this function
// will never get called until they do.
// @remove:	Called when the device is removed from the system to
// unbind a device from this driver.
// @shutdown:	Called at shut-down time to quiesce the device.
// @suspend:	Called to put the device to sleep mode. Usually to a
// low power state.
// @resume:	Called to bring a device from sleep mode.
// @groups:	Default attributes that get created by the driver core
// automatically.
// @dev_groups:	Additional attributes attached to device instance once
// it is bound to the driver.
// @pm:		Power management operations of the device which matched
// this driver.
// @coredump:	Called when sysfs entry is written to. The device driver
// is expected to call the dev_coredump API resulting in a
// uevent.
// @p:		Driver core's private data, no one other than the driver
// core can touch this.
// @p_cb:	Callbacks private to the driver core; no one other than the
// driver core is allowed to touch this.
//
// The device driver-model tracks all of the drivers known to the system.
// The main reason for this tracking is to enable the driver core to match
// up drivers with new devices. Once drivers are known objects within the
// system, however, a number of other things become possible. Device drivers
// can export information and configuration variables that are independent
// of any specific device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_driver {
    pub name: *const c_char,
    pub bus: *const bus_type,
    pub owner: *mut module,
    pub /: *const *const *const char mod_name; / used for built-in modules,
    pub /: *mut *mut bool suppress_bind_attrs; / disables bind/unbind via sysfs,
    pub probe_type: probe_type,
    pub of_match_table: *const of_device_id,
    pub acpi_match_table: *const acpi_device_id,
    pub dev): *mut *mut int (probe) (struct device,
    pub dev): *mut *mut void (sync_state)(struct device,
    pub dev): *mut *mut int (remove) (struct device,
    pub dev): *mut *mut void (shutdown) (struct device,
    pub state): *mut *mut *mut int (suspend) (struct device dev, pm_message_t,
    pub dev): *mut *mut int (resume) (struct device,
    pub groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub pm: *const dev_pm_ops,
    pub dev): *mut *mut void (coredump) (struct device,
    pub p: *mut driver_private,
//
// Called after remove() but before devres entries are released.
// This is a Rust only callback.
//
    pub dev): *mut *mut void (post_unbind_rust)(struct device,
    pub p_cb: },
}

extern "C" {
    pub fn driver_register(drv: *mut device_driver) -> int __must_check;
}
extern "C" {
    pub fn driver_unregister(drv: *mut device_driver);
}
extern "C" {
    pub fn driver_probe_done() -> bool __init;
}
extern "C" {
    pub fn wait_for_device_probe();
}
extern "C" {
    pub fn wait_for_init_devices_probe() -> void __init;
}
// sysfs interface for exporting driver attributes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct driver_attribute {
    pub attr: attribute,
    pub buf): *mut *mut *mut ssize_t (show)(struct device_driver driver, char,
    pub count): usize,
}

//
// driver_find_device_by_name - device iterator for locating a particular device
// of a specific name.
// @drv: the driver we're iterating
// @name: name of the device to match
//
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: NULL, _arg: name, _arg: device_match_name) -> return;
}
//
// driver_find_device_by_of_node- device iterator for locating a particular device
// by of_node pointer.
// @drv: the driver we're iterating
// @np: of_node pointer to match.
//
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: NULL, _arg: np, _arg: device_match_of_node) -> return;
}
//
// driver_find_device_by_fwnode- device iterator for locating a particular device
// by fwnode pointer.
// @drv: the driver we're iterating
// @fwnode: fwnode pointer to match.
//
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: NULL, _arg: fwnode, _arg: device_match_fwnode) -> return;
}
//
// driver_find_device_by_devt- device iterator for locating a particular device
// by devt.
// @drv: the driver we're iterating
// @devt: devt pointer to match.
//
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: NULL, _arg: &devt, _arg: device_match_devt) -> return;
}
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: start, _arg: NULL, _arg: device_match_any) -> return;
}

//
// driver_find_device_by_acpi_dev : device iterator for locating a particular
// device matching the ACPI_COMPANION device.
// @drv: the driver we're iterating
// @adev: ACPI_COMPANION device to match.
//
extern "C" {
    pub fn driver_find_device(_arg: drv, _arg: NULL, _arg: adev, _arg: device_match_acpi_dev) -> return;
}

extern "C" {
    pub fn driver_deferred_probe_add(dev: *mut device);
}
extern "C" {
    pub fn driver_deferred_probe_check_state(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn driver_init();
}
//
// module_driver() - Helper macro for drivers that don't do anything
// special in module init/exit. This eliminates a lot of boilerplate.
// Each module may only use this macro once, and calling it replaces
// module_init() and module_exit().
//
// @__driver: driver name
// @__register: register function for this driver type
// @__unregister: unregister function for this driver type
// @...: Additional arguments to be passed to __register and __unregister.
//
// Use this macro to construct bus specific macros for registering
// drivers, and do not use it on its own.
//

//
// builtin_driver() - Helper macro for drivers that don't do anything
// special in init and have no exit. This eliminates some boilerplate.
// Each driver may only use this macro once, and calling it replaces
// device_initcall (or in some cases, the legacy __initcall).  This is
// meant to be a direct parallel of module_driver() above but without
// the __exit stuff that is not used for builtin cases.
//
// @__driver: driver name
// @__register: register function for this driver type
// @...: Additional arguments to be passed to __register
//
// Use this macro to construct bus specific macros for registering
// drivers, and do not use it on its own.
//

