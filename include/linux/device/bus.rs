//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/device/bus.h
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
// bus.h - the bus-specific portions of the driver model
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
// struct bus_type - The bus type of the device
//
// @name:	The name of the bus.
// @dev_name:	Used for subsystems to enumerate devices like ("foo%u", dev->id).
// @bus_groups:	Default attributes of the bus.
// @dev_groups:	Default attributes of the devices on the bus.
// @drv_groups: Default attributes of the device drivers on the bus.
// @match:	Called, perhaps multiple times, whenever a new device or driver
// is added for this bus. It should return a positive value if the
// given device can be handled by the given driver and zero
// otherwise. It may also return error code if determining that
// the driver supports the device is not possible. In case of
// -EPROBE_DEFER it will queue the device for deferred probing.
// Note: This callback may be invoked with or without the device
// lock held.
// @uevent:	Called when a device is added, removed, or a few other things
// that generate uevents to add the environment variables.
// @probe:	Called when a new device or driver add to this bus, and callback
// the specific driver's probe to initial the matched device.
// @sync_state:	Called to sync device state to software state after all the
// state tracking consumers linked to this device (present at
// the time of late_initcall) have successfully bound to a
// driver. If the device has no consumers, this function will
// be called at late_initcall_sync level. If the device has
// consumers that are never bound to a driver, this function
// will never get called until they do.
// @remove:	Called when a device removed from this bus.
// @shutdown:	Called at shut-down time to quiesce the device.
// @irq_get_affinity:	Get IRQ affinity mask for the device on this bus.
//
// @online:	Called to put the device back online (after offlining it).
// @offline:	Called to put the device offline for hot-removal. May fail.
//
// @suspend:	Called when a device on this bus wants to go to sleep mode.
// @resume:	Called to bring a device on this bus out of sleep mode.
// @num_vf:	Called to find out how many virtual functions a device on this
// bus supports.
// @dma_configure:	Called to setup DMA configuration on a device on
// this bus.
// @dma_cleanup:	Called to cleanup DMA configuration on a device on
// this bus.
// @pm:		Power management operations of this bus, callback the specific
// device driver's pm-ops.
// @driver_override:	Set to true if this bus supports the driver_override
// mechanism, which allows userspace to force a specific
// driver to bind to a device via a sysfs attribute.
// @need_parent_lock:	When probing or removing a device on this bus, the
// device core should lock the device's parent.
//
// A bus is a channel between the processor and one or more devices. For the
// purposes of the device model, all devices are connected via a bus, even if
// it is an internal, virtual, "platform" bus. Buses can plug into each other.
// A USB controller is usually a PCI device, for example. The device model
// represents the actual connections between buses and the devices they control.
// A bus is represented by the bus_type structure. It contains the name, the
// default attributes, the bus' methods, PM operations, and the driver core's
// private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_type {
    pub name: *const c_char,
    pub dev_name: *const c_char,
    pub bus_groups: *const *const attribute_group,
    pub dev_groups: *const *const attribute_group,
    pub drv_groups: *const *const attribute_group,
    pub drv): *const *const *const int (match)(struct device dev, struct device_driver,
    pub env): *const *const *const int (uevent)(struct device dev, struct kobj_uevent_env,
    pub dev): *mut *mut int (probe)(struct device,
    pub dev): *mut *mut void (sync_state)(struct device,
    pub dev): *mut *mut void (remove)(struct device,
    pub dev): *mut *mut void (shutdown)(struct device,
    pub irq_vec): c_uint,
    pub dev): *mut *mut int (online)(struct device,
    pub dev): *mut *mut int (offline)(struct device,
    pub state): *mut *mut *mut int (suspend)(struct device dev, pm_message_t,
    pub dev): *mut *mut int (resume)(struct device,
    pub dev): *mut *mut int (num_vf)(struct device,
    pub dev): *mut *mut int (dma_configure)(struct device,
    pub dev): *mut *mut void (dma_cleanup)(struct device,
    pub pm: *const dev_pm_ops,
    pub driver_override: bool,
    pub need_parent_lock: bool,
}

extern "C" {
    pub fn bus_register(bus: *const bus_type) -> int __must_check;
}
extern "C" {
    pub fn bus_unregister(bus: *const bus_type);
}
extern "C" {
    pub fn bus_rescan_devices(bus: *const bus_type) -> int __must_check;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bus_attribute {
    pub attr: attribute,
    pub buf): *const *const *const ssize_t (show)(struct bus_type bus, char,
    pub count): *const *const *const *const ssize_t (store)(struct bus_type bus, char buf, size_t,
}

extern "C" {
    pub fn bus_create_file(bus: *const bus_type, attr: *mut bus_attribute) -> int __must_check;
}
extern "C" {
    pub fn bus_remove_file(bus: *const bus_type, attr: *mut bus_attribute);
}
// Matching function type for drivers/base APIs to find a specific device
extern "C" {
    pub fn int(dev: *mut *mut device_match_t)(struct device, data: *const c_void) -> typedef;
}
// Generic device matching functions that all busses can use to match with
extern "C" {
    pub fn device_match_name(dev: *mut device, name: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_type(dev: *mut device, type: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_of_node(dev: *mut device, np: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_fwnode(dev: *mut device, fwnode: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_devt(dev: *mut device, pdevt: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_acpi_dev(dev: *mut device, adev: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_acpi_handle(dev: *mut device, handle: *const c_void) -> c_int;
}
extern "C" {
    pub fn device_match_any(dev: *mut device, unused: *const c_void) -> c_int;
}
// Device iterating function type for various driver core for_each APIs
extern "C" {
    pub fn int(dev: *mut *mut device_iter_t)(struct device, data: *mut c_void) -> typedef;
}
// iterator helpers for buses
//
// bus_find_device_by_name - device iterator for locating a particular device
// of a specific name.
// @bus: bus type
// @start: Device to begin with
// @name: name of the device to match
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: start, _arg: name, _arg: device_match_name) -> return;
}
//
// bus_find_device_by_of_node : device iterator for locating a particular device
// matching the of_node.
// @bus: bus type
// @np: of_node of the device to match.
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: NULL, _arg: np, _arg: device_match_of_node) -> return;
}
//
// bus_find_device_by_fwnode : device iterator for locating a particular device
// matching the fwnode.
// @bus: bus type
// @fwnode: fwnode of the device to match.
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: NULL, _arg: fwnode, _arg: device_match_fwnode) -> return;
}
//
// bus_find_device_by_devt : device iterator for locating a particular device
// matching the device type.
// @bus: bus type
// @devt: device type of the device to match.
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: NULL, _arg: &devt, _arg: device_match_devt) -> return;
}
//
// bus_find_next_device - Find the next device after a given device in a
// given bus.
// @bus: bus type
// @cur: device to begin the search with.
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: cur, _arg: NULL, _arg: device_match_any) -> return;
}

//
// bus_find_device_by_acpi_dev : device iterator for locating a particular device
// matching the ACPI COMPANION device.
// @bus: bus type
// @adev: ACPI COMPANION device to match.
//
extern "C" {
    pub fn bus_find_device(_arg: bus, _arg: NULL, _arg: adev, _arg: device_match_acpi_dev) -> return;
}

//
// Bus notifiers: Get notified of addition/removal of devices
// and binding/unbinding of drivers to devices.
// In the long run, it should be a replacement for the platform
// notify hooks.
//
extern "C" {
    pub fn bus_register_notifier(bus: *const bus_type, nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn bus_unregister_notifier(bus: *const bus_type, nb: *mut notifier_block) -> c_int;
}
//
// enum bus_notifier_event - Bus Notifier events that have happened
// @BUS_NOTIFY_ADD_DEVICE: device is added to this bus
// @BUS_NOTIFY_DEL_DEVICE: device is about to be removed from this bus
// @BUS_NOTIFY_REMOVED_DEVICE: device is successfully removed from this bus
// @BUS_NOTIFY_BIND_DRIVER: a driver is about to be bound to this device on this bus
// @BUS_NOTIFY_BOUND_DRIVER: a driver is successfully bound to this device on this bus
// @BUS_NOTIFY_UNBIND_DRIVER: a driver is about to be unbound from this device on this bus
// @BUS_NOTIFY_UNBOUND_DRIVER: a driver is successfully unbound from this device on this bus
// @BUS_NOTIFY_DRIVER_NOT_BOUND: a driver failed to be bound to this device on this bus
//
// These are the value passed to a bus notifier when a specific event happens.
//
// Note that bus notifiers are likely to be called with the device lock already
// held by the driver core, so be careful in any notifier callback as to what
// you do with the device structure.
//
// All bus notifiers are called with the target struct device * as an argument.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bus_notifier_event {
    BUS_NOTIFY_ADD_DEVICE,
    BUS_NOTIFY_DEL_DEVICE,
    BUS_NOTIFY_REMOVED_DEVICE,
    BUS_NOTIFY_BIND_DRIVER,
    BUS_NOTIFY_BOUND_DRIVER,
    BUS_NOTIFY_UNBIND_DRIVER,
    BUS_NOTIFY_UNBOUND_DRIVER,
    BUS_NOTIFY_DRIVER_NOT_BOUND,
}
