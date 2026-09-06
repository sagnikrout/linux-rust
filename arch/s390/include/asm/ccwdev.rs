//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/asm/ccwdev.h
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
// Copyright IBM Corp. 2002, 2009
//
// Author(s): Arnd Bergmann <arndb@de.ibm.com>
//
// Interface for CCW device drivers
//

// structs from asm/cio.h
// simplified initializers for struct ccw_device:
// CCW_DEVICE and CCW_DEVICE_DEVTYPE initialize one
// entry in your MODULE_DEVICE_TABLE and set the match_flag correctly

// scan through an array of device ids and return the first
// entry that matches the device.
//
// the array must end with an entry containing zero match_flags
//
// struct ccw_device - channel attached device
// @ccwlock: pointer to device lock
// @id: id of this device
// @drv: ccw driver for this device
// @dev: embedded device structure
// @online: online status of device
// @handler: interrupt handler
//
// @handler is a member of the device rather than the driver since a driver
// can have different interrupt handlers for different ccw devices
// (multi-subchannel drivers).
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_device {
    pub ccwlock: *mut spinlock_t,
// private:
    pub /: *mut *mut *mut ccw_device_private private; / cio private information,
    pub reg_mutex: mutex,
// public:
    pub id: ccw_device_id,
    pub drv: *mut ccw_driver,
    pub dev: device,
    pub online: c_int,
    pub ): *mut *mut *mut void (handler) (struct ccw_device , unsigned long, struct irb,
}

//
// Possible events used by the path_event notifier.
//
pub const PE_NONE: c_uint = 0x0;
pub const PE_PATH_GONE: c_uint = 0x1 /* A path is no longer available. */;
pub const PE_PATH_AVAILABLE: c_uint = 0x2 /* A path has become available and;
pub const PE_PATHGROUP_ESTABLISHED: c_uint = 0x4 /* A pathgroup was reset and had;
pub const PE_PATH_FCES_EVENT: c_uint = 0x8 /* The FCES Status of a path has;
// changed.
//
// Possible CIO actions triggered by the unit check handler.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum uc_todo {
    UC_TODO_RETRY,
    UC_TODO_RETRY_ON_NEW_PATH,
    UC_TODO_STOP
}

//
// struct ccw_driver - device driver for channel attached devices
// @ids: ids supported by this driver
// @probe: function called on probe
// @remove: function called on remove
// @set_online: called when setting device online
// @set_offline: called when setting device offline
// @notify: notify driver of device state changes
// @path_event: notify driver of channel path events
// @shutdown: called at device shutdown
// @uc_handler: callback for unit check handler
// @driver: embedded device driver structure
// @int_class: interruption class to use for accounting interrupts
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ccw_driver {
    pub ids: *mut ccw_device_id,
    pub ): *mut *mut int (probe) (struct ccw_device,
    pub ): *mut *mut void (remove) (struct ccw_device,
    pub ): *mut *mut int (set_online) (struct ccw_device,
    pub ): *mut *mut int (set_offline) (struct ccw_device,
    pub int): *mut *mut *mut int (notify) (struct ccw_device ,,
    pub ): *mut *mut *mut void (path_event) (struct ccw_device , int,
    pub ): *mut *mut void (shutdown) (struct ccw_device,
    pub ): *mut *mut *mut uc_todo (uc_handler) (struct ccw_device , struct irb,
    pub driver: device_driver,
    pub int_class: interruption_class,
}

// devices drivers call these during module load and unload.
// When a driver is registered, its probe method is called
// when new devices for its type pop up
extern "C" {
    pub fn ccw_driver_register(driver: *mut ccw_driver) -> c_int;
}
extern "C" {
    pub fn ccw_driver_unregister(driver: *mut ccw_driver);
}
extern "C" {
    pub fn ccw_device_set_options_mask(: *mut ccw_device, long: unsigned) -> c_int;
}
extern "C" {
    pub fn ccw_device_set_options(: *mut ccw_device, long: unsigned) -> c_int;
}
extern "C" {
    pub fn ccw_device_clear_options(: *mut ccw_device, long: unsigned);
}
extern "C" {
    pub fn ccw_device_is_pathgroup(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_is_multipath(cdev: *mut ccw_device) -> c_int;
}
// Allow for i/o completion notification after primary interrupt status.
pub const CCWDEV_EARLY_NOTIFICATION: c_uint = 0x0001;
// Report all interrupt conditions.
pub const CCWDEV_REPORT_ALL: c_uint = 0x0002;
// Try to perform path grouping.
pub const CCWDEV_DO_PATHGROUP: c_uint = 0x0004;
// Allow forced onlining of boxed devices.
pub const CCWDEV_ALLOW_FORCE: c_uint = 0x0008;
// Try to use multipath mode.
pub const CCWDEV_DO_MULTIPATH: c_uint = 0x0010;
extern "C" {
    pub fn ccw_device_resume(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_halt(: *mut ccw_device, long: unsigned) -> c_int;
}
extern "C" {
    pub fn ccw_device_clear(: *mut ccw_device, long: unsigned) -> c_int;
}
extern "C" {
    pub fn ccw_device_tm_intrg(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_mdc(cdev: *mut ccw_device, mask: u8) -> c_int;
}
extern "C" {
    pub fn ccw_device_set_online(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_set_offline(cdev: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_path_mask(: *mut ccw_device) -> __u8;
}
extern "C" {
    pub fn ccw_device_get_id(: *mut ccw_device, : *mut ccw_dev_id);
}

extern "C" {
    pub fn ccw_device_destroy_console(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_enable_console(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_wait_idle(: *mut ccw_device);
}
extern "C" {
    pub fn ccw_device_siosl(: *mut ccw_device) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_schid(: *mut ccw_device, : *mut subchannel_id);
}
extern "C" {
    pub fn ccw_device_get_cssid(cdev: *mut ccw_device, cssid: *mut u8) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_iid(cdev: *mut ccw_device, iid: *mut u8) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_chpid(cdev: *mut ccw_device, chp_idx: c_int, chpid: *mut u8) -> c_int;
}
extern "C" {
    pub fn ccw_device_get_chid(cdev: *mut ccw_device, chp_idx: c_int, chid: *mut u16) -> c_int;
}
