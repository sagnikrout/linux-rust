//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/watchdog.h
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
// Generic watchdog defines. Derived from..
//
// Berkshire PC Watchdog Defines
// by Ken Hollis <khollis@bitgate.com>
//

//
// struct watchdog_ops - The watchdog-devices operations
//
// @owner:	The module owner.
// @start:	The routine for starting the watchdog device.
// @stop:	The routine for stopping the watchdog device.
// @ping:	The routine that sends a keepalive ping to the watchdog device.
// @status:	The routine that shows the status of the watchdog device.
// @set_timeout:The routine for setting the watchdog devices timeout value (in seconds).
// @set_pretimeout:The routine for setting the watchdog devices pretimeout.
// @get_timeleft:The routine that gets the time left before a reset (in seconds).
// @restart:	The routine for restarting the machine.
// @ioctl:	The routines that handles extra ioctl calls.
//
// The watchdog_ops structure contains a list of low-level operations
// that control a watchdog device. It also contains the module that owns
// these operations. The start function is mandatory, all other
// functions are optional.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_ops {
    pub owner: *mut module,
// mandatory operations
    pub ): *mut *mut int (start)(struct watchdog_device,
// optional operations
    pub ): *mut *mut int (stop)(struct watchdog_device,
    pub ): *mut *mut int (ping)(struct watchdog_device,
    pub ): *mut *mut unsigned int (status)(struct watchdog_device,
    pub int): *mut *mut *mut int (set_timeout)(struct watchdog_device , unsigned,
    pub int): *mut *mut *mut int (set_pretimeout)(struct watchdog_device , unsigned,
    pub ): *mut *mut unsigned int (get_timeleft)(struct watchdog_device,
    pub ): *mut *mut *mut int (restart)(struct watchdog_device , unsigned long, void,
    pub long): *mut *mut *mut long (ioctl)(struct watchdog_device , unsigned int, unsigned,
}

//
// struct watchdog_device - The structure that defines a watchdog device
//
// @id:		The watchdog's ID. (Allocated by watchdog_register_device)
// @parent:	The parent bus device
// @groups:	List of sysfs attribute groups to create when creating the
// watchdog device.
// @info:	Pointer to a watchdog_info structure.
// @ops:	Pointer to the list of watchdog operations.
// @gov:	Pointer to watchdog pretimeout governor.
// @bootstatus:	Status of the watchdog device at boot.
// @timeout:	The watchdog devices timeout value (in seconds).
// @pretimeout: The watchdog devices pre_timeout value.
// @min_timeout:The watchdog devices minimum timeout value (in seconds).
// @max_timeout:The watchdog devices maximum timeout value (in seconds)
// as configurable from user space. Only relevant if
// max_hw_heartbeat_ms is not provided.
// @min_hw_heartbeat_ms:
// Hardware limit for minimum time between heartbeats,
// in milli-seconds.
// @max_hw_heartbeat_ms:
// Hardware limit for maximum timeout, in milli-seconds.
// Replaces max_timeout if specified.
// @reboot_nb:	The notifier block to stop watchdog on reboot.
// @restart_nb:	The notifier block to register a restart function.
// @pm_nb:	The notifier block to stop watchdog on suspend and restart it
// on resume.
// @driver_data:Pointer to the drivers private data.
// @wd_data:	Pointer to watchdog core internal data.
// @status:	Field that contains the devices internal status bits.
// @deferred:	Entry in wtd_deferred_reg_list which is used to
// register early initialized watchdogs.
//
// The watchdog_device structure contains all information about a
// watchdog timer device.
//
// The driver-data field may not be accessed directly. It must be accessed
// via the watchdog_set_drvdata and watchdog_get_drvdata helpers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_device {
    pub id: c_int,
    pub parent: *mut device,
    pub groups: *const attribute_group,
    pub info: *const watchdog_info,
    pub ops: *const watchdog_ops,
    pub gov: *const watchdog_governor,
    pub bootstatus: c_uint,
    pub timeout: c_uint,
    pub pretimeout: c_uint,
    pub min_timeout: c_uint,
    pub max_timeout: c_uint,
    pub min_hw_heartbeat_ms: c_uint,
    pub max_hw_heartbeat_ms: c_uint,
    pub reboot_nb: notifier_block,
    pub restart_nb: notifier_block,
    pub pm_nb: notifier_block,
    pub driver_data: *mut c_void,
    pub wd_data: *mut watchdog_core_data,
    pub status: c_ulong,
// Bit numbers for status flags

    pub deferred: list_head,
}

// Use the following function to check whether or not the watchdog is active
extern "C" {
    pub fn test_bit(_arg: WDOG_ACTIVE, _arg: &wdd->status) -> return;
}
//
// Use the following function to check whether or not the hardware watchdog
// is running
//
extern "C" {
    pub fn test_bit(_arg: WDOG_HW_RUNNING, _arg: &wdd->status) -> return;
}
// Use the following function to set the nowayout feature
// Use the following function to stop the watchdog on reboot
// Use the following function to stop the watchdog when unregistering it
// Use the following function to stop the wdog ping worker when suspending
// Use the following function to check if a timeout value is invalid
//
// The timeout is invalid if
// - the requested value is larger than UINT_MAX / 1000
// (since internal calculations are done in milli-seconds),
// or
// - the requested value is smaller than the configured minimum timeout,
// or
// - a maximum hardware timeout is not configured, a maximum timeout
// is configured, and the requested value is larger than the
// configured maximum timeout.
//
// Use the following function to check if a pretimeout value is invalid
// Use the following functions to manipulate watchdog driver specific data
// Use the following functions to report watchdog pretimeout event

extern "C" {
    pub fn watchdog_notify_pretimeout(wdd: *mut watchdog_device);
}

// drivers/watchdog/watchdog_core.c
extern "C" {
    pub fn watchdog_set_restart_priority(wdd: *mut watchdog_device, priority: c_int);
}
extern "C" {
    pub fn watchdog_register_device(: *mut watchdog_device) -> c_int;
}
extern "C" {
    pub fn watchdog_unregister_device(: *mut watchdog_device);
}
extern "C" {
    pub fn watchdog_dev_suspend(wdd: *mut watchdog_device) -> c_int;
}
extern "C" {
    pub fn watchdog_dev_resume(wdd: *mut watchdog_device) -> c_int;
}
extern "C" {
    pub fn watchdog_set_last_hw_keepalive(: *mut watchdog_device, int: unsigned) -> c_int;
}
// devres register variant
extern "C" {
    pub fn devm_watchdog_register_device(dev: *mut device, : *mut watchdog_device) -> c_int;
}
