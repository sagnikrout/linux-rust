//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/base/base.h
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
// Copyright (c) 2001-2003 Patrick Mochel <mochel@osdl.org>
// Copyright (c) 2004-2009 Greg Kroah-Hartman <gregkh@suse.de>
// Copyright (c) 2008-2012 Novell Inc.
// Copyright (c) 2012-2019 Greg Kroah-Hartman <gregkh@linuxfoundation.org>
// Copyright (c) 2012-2019 Linux Foundation
//
// Core driver model functions and structures that should not be
// shared outside of the drivers/base/ directory.
//

//
// struct subsys_private - structure to hold the private to the driver core
// portions of the bus_type/class structure.
// @subsys: the struct kset that defines this subsystem
// @devices_kset: the subsystem's 'devices' directory
// @interfaces: list of subsystem interfaces associated
// @mutex: protect the devices, and interfaces lists.
// @drivers_kset: the list of drivers associated
// @klist_devices: the klist to iterate over the @devices_kset
// @klist_drivers: the klist to iterate over the @drivers_kset
// @bus_notifier: the bus notifier list for anything that cares about things
// on this bus.
// @drivers_autoprobe: gate whether new devices are automatically attached to
// registered drivers, or new drivers automatically attach
// to existing devices.
// @bus: pointer back to the struct bus_type that this structure is associated
// with.
// @dev_root: Default device to use as the parent.
// @glue_dirs: "glue" directory to put in-between the parent device to
// avoid namespace conflicts
// @class: pointer back to the struct class that this structure is associated
// with.
// @lock_key: Lock class key for use by the lock validator
//
// This structure is the one that is the actual kobject allowing struct
// bus_type/class to be statically allocated safely.  Nothing outside of the
// driver core should ever touch these fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct subsys_private {
    pub subsys: kset,
    pub devices_kset: *mut kset,
    pub interfaces: list_head,
    pub mutex: mutex,
    pub drivers_kset: *mut kset,
    pub klist_devices: klist,
    pub klist_drivers: klist,
    pub bus_notifier: blocking_notifier_head,
    pub drivers_autoprobe:1: c_uint,
    pub bus: *const bus_type,
    pub dev_root: *mut device,
    pub glue_dirs: kset,
    pub class: *const class,
    pub lock_key: lock_class_key,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct driver_private {
    pub kobj: kobject,
    pub klist_devices: klist,
    pub knode_bus: klist_node,
    pub mkobj: *mut module_kobject,
    pub driver: *mut device_driver,
}

//
// struct device_private - structure to hold the private to the driver core
// portions of the device structure.
// @klist_children: klist containing all children of this device
// @knode_parent: node in sibling list
// @knode_driver: node in driver list
// @knode_bus: node in bus list
// @knode_class: node in class list
// @deferred_probe: entry in deferred_probe_list which is used to retry the
// binding of drivers which were unable to get all the
// resources needed by the device; typically because it depends
// on another driver getting probed first.
// @async_driver: pointer to device driver awaiting probe via async_probe
// @deferred_probe_reason: capture the -EPROBE_DEFER message emitted with
// dev_err_probe() for later retrieval via debugfs
// @device: pointer back to the struct device that this structure is
// associated with.
// @dead: This device is currently either in the process of or has been
// removed from the system. Any asynchronous events scheduled for this
// device should exit without taking any action.
//
// Nothing outside of the driver core should ever touch these fields.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct device_private {
    pub klist_children: klist,
    pub knode_parent: klist_node,
    pub knode_driver: klist_node,
    pub knode_bus: klist_node,
    pub knode_class: klist_node,
    pub deferred_probe: list_head,
    pub async_driver: *const device_driver,
    pub deferred_probe_reason: *mut c_char,
    pub device: *mut device,
    pub dead:1: u8,
}

// initialisation functions
extern "C" {
    pub fn devices_init() -> c_int;
}
extern "C" {
    pub fn buses_init() -> c_int;
}
extern "C" {
    pub fn classes_init() -> c_int;
}
extern "C" {
    pub fn firmware_init() -> c_int;
}

extern "C" {
    pub fn hypervisor_init() -> c_int;
}

extern "C" {
    pub fn platform_bus_init() -> c_int;
}
extern "C" {
    pub fn faux_bus_init() -> c_int;
}
extern "C" {
    pub fn cpu_dev_init();
}
extern "C" {
    pub fn container_dev_init();
}

extern "C" {
    pub fn auxiliary_bus_init();
}

extern "C" {
    pub fn bus_add_device(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn bus_probe_device(dev: *mut device);
}
extern "C" {
    pub fn bus_remove_device(dev: *mut device);
}
extern "C" {
    pub fn bus_notify(dev: *mut device, value: bus_notifier_event);
}
extern "C" {
    pub fn bus_is_registered(bus: *const bus_type) -> bool;
}
extern "C" {
    pub fn bus_add_driver(drv: *mut device_driver) -> c_int;
}
extern "C" {
    pub fn bus_remove_driver(drv: *mut device_driver);
}
extern "C" {
    pub fn driver_detach(drv: *const device_driver);
}
extern "C" {
    pub fn driver_deferred_probe_del(dev: *mut device);
}
extern "C" {
    pub fn device_set_deferred_probe_reason(dev: *const device, vaf: *mut va_format);
}
extern "C" {
    pub fn device_driver_detach(dev: *mut device);
}
//
// Majority (all?) read accesses to dev->driver happens either
// while holding device lock or in bus/driver code that is only
// invoked when the device is bound to a driver and there is no
// concern of the pointer being changed while it is being read.
// However when reading device's uevent file we read driver pointer
// without taking device lock (so we do not block there for
// arbitrary amount of time). We use WRITE_ONCE() here to prevent
// tearing so that READ_ONCE() can safely be used in uevent code.
//
// FIXME - this cast should not be needed "soon"
extern "C" {
    pub fn void(dev: *mut *mut dr_node_release_t)(struct device, node: *mut devres_node) -> typedef;
}
extern "C" {
    pub fn void(node: *mut *mut dr_node_free_t)(struct devres_node) -> typedef;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct devres_node {
    pub entry: list_head,
    pub release: dr_node_release_t,
    pub free_node: dr_node_free_t,
    pub name: *const c_char,
    pub size: usize,
}

extern "C" {
    pub fn devres_node_add(dev: *mut device, node: *mut devres_node);
}
extern "C" {
    pub fn devres_node_remove(dev: *mut device, node: *mut devres_node) -> bool;
}
extern "C" {
    pub fn devres_release_all(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn device_block_probing();
}
extern "C" {
    pub fn device_unblock_probing();
}
extern "C" {
    pub fn deferred_probe_extend_timeout();
}
extern "C" {
    pub fn driver_deferred_probe_trigger();
}
// /sys/devices directory
extern "C" {
    pub fn devices_kset_move_last(dev: *mut device);
}

extern "C" {
    pub fn module_add_driver(mod: *mut module, drv: *const device_driver) -> c_int;
}
extern "C" {
    pub fn module_remove_driver(drv: *const device_driver);
}

extern "C" {
    pub fn devtmpfs_init() -> c_int;
}

// Device links support
extern "C" {
    pub fn device_links_read_lock() -> c_int;
}
extern "C" {
    pub fn device_links_read_unlock(idx: c_int);
}
extern "C" {
    pub fn device_links_read_lock_held() -> c_int;
}
extern "C" {
    pub fn device_links_check_suppliers(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn device_links_force_bind(dev: *mut device);
}
extern "C" {
    pub fn device_links_driver_bound(dev: *mut device);
}
extern "C" {
    pub fn device_links_driver_cleanup(dev: *mut device);
}
extern "C" {
    pub fn device_links_no_driver(dev: *mut device);
}
extern "C" {
    pub fn device_links_busy(dev: *mut device) -> bool;
}
extern "C" {
    pub fn device_links_unbind_consumers(dev: *mut device);
}
extern "C" {
    pub fn device_link_flag_is_sync_state_only(flags: u32) -> bool;
}
extern "C" {
    pub fn fw_devlink_drivers_done();
}
extern "C" {
    pub fn fw_devlink_probing_done();
}

// device pm support
extern "C" {
    pub fn device_pm_move_to_tail(dev: *mut device);
}

extern "C" {
    pub fn devtmpfs_create_node(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn devtmpfs_delete_node(dev: *mut device) -> c_int;
}

extern "C" {
    pub fn software_node_init();
}
extern "C" {
    pub fn software_node_notify(dev: *mut device);
}
extern "C" {
    pub fn software_node_notify_remove(dev: *mut device);
}

extern "C" {
    pub fn pinctrl_bind_pins(dev: *mut device) -> c_int;
}

