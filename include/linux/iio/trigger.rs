//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/iio/trigger.h
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
// The industrial I/O core, trigger handling functions
//
// Copyright (c) 2008 Jonathan Cameron
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_subirq {
    pub enabled: bool,
}

//
// struct iio_trigger_ops - operations structure for an iio_trigger.
// @set_trigger_state:	switch on/off the trigger on demand
// @reenable:		function to reenable the trigger when the
// use count is zero (may be NULL)
// @validate_device:	function to validate the device when the
// current trigger gets changed.
//
// This is typically static const within a driver and shared by
// instances of a given device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_trigger_ops {
    pub state): *mut *mut *mut int (set_trigger_state)(struct iio_trigger trig, bool,
    pub trig): *mut *mut void (reenable)(struct iio_trigger,
    pub indio_dev): *mut iio_dev,
}

//
// struct iio_trigger - industrial I/O trigger device
// @ops:		[DRIVER] operations structure
// @owner:		[INTERN] owner of this driver module
// @id:			[INTERN] unique id number
// @name:		[DRIVER] unique name
// @dev:		[DRIVER] associated device (if relevant)
// @list:		[INTERN] used in maintenance of global trigger list
// @alloc_list:		[DRIVER] used for driver specific trigger list
// @use_count:		[INTERN] use count for the trigger.
// @subirq_chip:	[INTERN] associate 'virtual' irq chip.
// @subirq_base:	[INTERN] base number for irqs provided by trigger.
// @subirqs:		[INTERN] information about the 'child' irqs.
// @pool:		[INTERN] bitmap of irqs currently in use.
// @pool_lock:		[INTERN] protection of the irq pool.
// @attached_own_device:[INTERN] if we are using our own device as trigger,
// i.e. if we registered a poll function to the same
// device as the one providing the trigger.
// @reenable_work:	[INTERN] work item used to ensure reenable can sleep.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iio_trigger {
    pub ops: *const iio_trigger_ops,
    pub owner: *mut module,
    pub id: c_int,
    pub name: *const c_char,
    pub dev: device,
    pub list: list_head,
    pub alloc_list: list_head,
    pub use_count: core::sync::atomic::AtomicI32,
    pub subirq_chip: irq_chip,
    pub subirq_base: c_int,
    pub subirqs: [iio_subirq; CONFIG_IIO_CONSUMERS_PER_TRIGGER],
    pub pool: [c_ulong; BITS_TO_LONGS(CONFIG_IIO_CONSUMERS_PER_TRIGGER)],
    pub pool_lock: mutex,
    pub attached_own_device: bool,
    pub reenable_work: work_struct,
}

extern "C" {
    pub fn container_of(_arg: d, iio_trigger: struct, _arg: dev) -> return;
}
//
// iio_trigger_set_drvdata() - Set trigger driver data
// @trig: IIO trigger structure
// @data: Driver specific data
//
// Allows to attach an arbitrary pointer to an IIO trigger, which can later be
// retrieved by iio_trigger_get_drvdata().
//
// iio_trigger_get_drvdata() - Get trigger driver data
// @trig: IIO trigger structure
//
// Returns the data previously set with iio_trigger_set_drvdata()
//
extern "C" {
    pub fn dev_get_drvdata(_arg: &trig->dev) -> return;
}
//
// iio_trigger_register() - register a trigger with the IIO core
// @trig_info:	trigger to be registered
//
extern "C" {
    pub fn iio_trigger_register(trig_info: *mut iio_trigger) -> c_int;
}
//
// iio_trigger_unregister() - unregister a trigger from the core
// @trig_info:	trigger to be unregistered
//
extern "C" {
    pub fn iio_trigger_unregister(trig_info: *mut iio_trigger);
}
//
// iio_trigger_set_immutable() - set an immutable trigger on destination
//
// @indio_dev: IIO device structure containing the device
// @trig: trigger to assign to device
//
extern "C" {
    pub fn iio_trigger_set_immutable(indio_dev: *mut iio_dev, trig: *mut iio_trigger) -> c_int;
}
extern "C" {
    pub fn iio_trigger_poll(trig: *mut iio_trigger);
}
extern "C" {
    pub fn iio_trigger_poll_nested(trig: *mut iio_trigger);
}
extern "C" {
    pub fn iio_trigger_generic_data_rdy_poll(irq: c_int, private: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn iio_trigger_free(trig: *mut iio_trigger);
}
//
// iio_trigger_using_own() - tells us if we use our own HW trigger ourselves
// @indio_dev:  device to check
//
extern "C" {
    pub fn iio_trigger_using_own(indio_dev: *mut iio_dev) -> bool;
}
extern "C" {
    pub fn iio_validate_own_trigger(idev: *mut iio_dev, trig: *mut iio_trigger) -> c_int;
}

