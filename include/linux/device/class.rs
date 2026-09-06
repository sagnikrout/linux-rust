//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device/class.h
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
// The class-specific portions of the driver model
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
// struct class - device classes
// @name:	Name of the class.
// @class_groups: Default attributes of this class.
// @dev_groups:	Default attributes of the devices that belong to the class.
// @dev_uevent:	Called when a device is added, removed from this class, or a
// few other things that generate uevents to add the environment
// variables.
// @devnode:	Callback to provide the devtmpfs.
// @class_release: Called to release this class.
// @dev_release: Called to release the device.
// @shutdown_pre: Called at shut-down time before driver shutdown.
// @ns_type:	Callbacks so sysfs can determine namespaces.
// @namespace:	Namespace of the device belongs to this class.
// @get_ownership: Allows class to specify uid/gid of the sysfs directories
// for the devices belonging to the class. Usually tied to
// device's namespace.
// @pm:		The default device power management operations of this class.
//
// A class is a higher-level view of a device that abstracts out low-level
// implementation details. Drivers may see a SCSI disk or an ATA disk, but,
// at the class level, they are all simply disks. Classes allow user space
// to work with devices based on what they do, rather than how they are
// connected or how they work.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct class {
    pub name: *const c_char,
    pub class_groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub env): *const *const *const int (dev_uevent)(struct device dev, struct kobj_uevent_env,
    pub mode): *const *const *const *const char (devnode)(struct device dev, umode_t,
    pub class): *const *const void (class_release)(struct class,
    pub dev): *mut *mut void (dev_release)(struct device,
    pub dev): *mut *mut int (shutdown_pre)(struct device,
    pub ns_type: *const kobj_ns_type_operations,
    pub dev): *const *const *const ns_common (namespace)(device,
    pub gid): *const *const *const *const void (get_ownership)(struct device dev, kuid_t uid, kgid_t,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_dev_iter {
    pub ki: klist_iter,
    pub type: *const device_type,
    pub sp: *mut subsys_private,
}

extern "C" {
    pub fn class_register(class: *const class) -> int __must_check;
}
extern "C" {
    pub fn class_unregister(class: *const class);
}
extern "C" {
    pub fn class_is_registered(class: *const class) -> bool;
}
extern "C" {
    pub fn class_compat_unregister(cls: *mut class_compat);
}
extern "C" {
    pub fn class_compat_create_link(cls: *mut class_compat, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn class_compat_remove_link(cls: *mut class_compat, dev: *mut device);
}
extern "C" {
    pub fn class_dev_iter_exit(iter: *mut class_dev_iter);
}
//
// class_find_device_by_name - device iterator for locating a particular device
// of a specific name.
// @class: class type
// @name: name of the device to match
//
extern "C" {
    pub fn class_find_device(_arg: class, _arg: NULL, _arg: name, _arg: device_match_name) -> return;
}
//
// class_find_device_by_of_node : device iterator for locating a particular device
// matching the of_node.
// @class: class type
// @np: of_node of the device to match.
//
extern "C" {
    pub fn class_find_device(_arg: class, _arg: NULL, _arg: np, _arg: device_match_of_node) -> return;
}
//
// class_find_device_by_fwnode : device iterator for locating a particular device
// matching the fwnode.
// @class: class type
// @fwnode: fwnode of the device to match.
//
extern "C" {
    pub fn class_find_device(_arg: class, _arg: NULL, _arg: fwnode, _arg: device_match_fwnode) -> return;
}
//
// class_find_device_by_devt : device iterator for locating a particular device
// matching the device type.
// @class: class type
// @devt: device type of the device to match.
//
extern "C" {
    pub fn class_find_device(_arg: class, _arg: NULL, _arg: &devt, _arg: device_match_devt) -> return;
}

//
// class_find_device_by_acpi_dev : device iterator for locating a particular
// device matching the ACPI_COMPANION device.
// @class: class type
// @adev: ACPI_COMPANION device to match.
//
extern "C" {
    pub fn class_find_device(_arg: class, _arg: NULL, _arg: adev, _arg: device_match_acpi_dev) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_attribute {
    pub attr: attribute,
    pub buf): *mut c_char,
    pub count): *const *const char buf, size_t,
}

extern "C" {
    pub fn class_create_file_ns(_arg: class, _arg: attr, _arg: NULL) -> return;
}
// Simple class attribute that is just a static string
#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_attribute_string {
    pub attr: class_attribute,
    pub str: *mut c_char,
}

// Currently read-only only

#[repr(C)]
#[derive(Copy, Clone)]
pub struct class_interface {
    pub node: list_head,
    pub class: *const class,
    pub dev): *mut *mut int (add_dev) (struct device,
    pub dev): *mut *mut void (remove_dev) (struct device,
}

extern "C" {
    pub fn class_interface_register(: *mut class_interface) -> int __must_check;
}
extern "C" {
    pub fn class_interface_unregister(: *mut class_interface);
}
extern "C" {
    pub fn class_create(name: *const c_char) -> *mut class  __must_check;
}
extern "C" {
    pub fn class_destroy(cls: *const class);
}
