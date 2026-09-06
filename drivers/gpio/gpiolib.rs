//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpio/gpiolib.h
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
// Internal GPIO functions.
//
// Copyright (C) 2013, Intel Corporation
// Author: Mika Westerberg <mika.westerberg@linux.intel.com>
//

//
// struct gpio_device - internal state container for GPIO devices
// @dev: the GPIO device struct
// @chrdev: character device for the GPIO device
// @id: numerical ID number for the GPIO chip
// @owner: helps prevent removal of modules exporting active GPIOs
// @chip: pointer to the corresponding gpiochip, holding static
// data for this device
// @descs: array of ngpio descriptors.
// @valid_mask: If not %NULL, holds bitmask of GPIOs which are valid to be
// used from the chip.
// @desc_srcu: ensures consistent state of GPIO descriptors exposed to users
// @ngpio: the number of GPIO lines on this GPIO device, equal to the size
// of the @descs array.
// @can_sleep: indicate whether the GPIO chip driver's callbacks can sleep
// implying that they cannot be used from atomic context
// @base: GPIO base in the DEPRECATED global Linux GPIO numberspace, assigned
// at device creation time.
// @label: a descriptive name for the GPIO device, such as the part number
// or name of the IP component in a System on Chip.
// @data: per-instance data assigned by the driver
// @list: links gpio_device:s together for traversal
// @line_state_notifier: used to notify subscribers about lines being
// requested, released or reconfigured
// @line_state_lock: RW-spinlock protecting the line state notifier
// @line_state_wq: used to emit line state events from a separate thread in
// process context
// @device_notifier: used to notify character device wait queues about the GPIO
// device being unregistered
// @srcu: protects the pointer to the underlying GPIO chip
// @pin_ranges: range of pins served by the GPIO driver
//
// This state container holds most of the runtime variable data
// for a GPIO device and can hold references and live on after the
// GPIO chip has been removed, if it is still being used from
// userspace.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_device {
    pub dev: device,
    pub chrdev: cdev,
    pub id: c_int,
    pub owner: *mut module,
    pub chip: *mut gpio_chip __rcu,
    pub descs: *mut gpio_desc,
    pub valid_mask: *mut c_ulong,
    pub desc_srcu: srcu_struct,
    pub base: c_uint,
    pub ngpio: u16,
    pub can_sleep: bool,
    pub label: *const c_char,
    pub data: *mut c_void,
    pub list: list_head,
    pub line_state_notifier: raw_notifier_head,
    pub line_state_lock: rwlock_t,
    pub line_state_wq: *mut workqueue_struct,
    pub device_notifier: blocking_notifier_head,
    pub srcu: srcu_struct,

//
// If CONFIG_PINCTRL is enabled, then gpio controllers can optionally
// describe the actual pin range which they serve in an SoC. This
// information would be used by pinctrl subsystem to configure
// corresponding pins for gpio usage.
//
    pub pin_ranges: list_head,

}

extern "C" {
    pub fn container_of(_arg: dev, gpio_device: struct, _arg: dev) -> return;
}
// GPIO suffixes used for ACPI and device tree lookup

// __suffixes && ({								\
//
// struct gpio_array - Opaque descriptor for a structure of GPIO array attributes
//
// @desc:		Array of pointers to the GPIO descriptors
// @size:		Number of elements in desc
// @gdev:		Parent GPIO device
// @get_mask:		Get mask used in fastpath
// @set_mask:		Set mask used in fastpath
// @invert_mask:	Invert mask used in fastpath
//
// This structure is attached to struct gpiod_descs obtained from
// gpiod_get_array() and can be passed back to get/set array functions in order
// to activate fast processing path if applicable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_array {
    pub desc: *mut gpio_desc,
    pub size: c_uint,
    pub gdev: *mut gpio_device,
    pub get_mask: *mut c_ulong,
    pub set_mask: *mut c_ulong,
    pub invert_mask: [c_ulong; ],
}

extern "C" {
    pub fn gpiod_set_transitory(desc: *mut gpio_desc, transitory: bool) -> c_int;
}
extern "C" {
    pub fn gpiod_line_state_notify(desc: *mut gpio_desc, action: c_ulong);
}
extern "C" {
    pub fn gpiod_direction_output_nonotify(desc: *mut gpio_desc, value: c_int) -> c_int;
}
extern "C" {
    pub fn gpiod_direction_input_nonotify(desc: *mut gpio_desc) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_desc_label {
    pub rh: rcu_head,
    pub str: [c_char; ],
}

//
// struct gpio_desc - Opaque descriptor for a GPIO
//
// @gdev:		Pointer to the parent GPIO device
// @flags:		Binary descriptor flags
// @label:		Name of the consumer
// @name:		Line name
// @hog:		Pointer to the device node that hogs this line (if any)
// @debounce_period_us:	Debounce period in microseconds
//
// These are obtained using gpiod_get() and are preferable to the old
// integer-based handles.
//
// Contrary to integers, a pointer to a &struct gpio_desc is guaranteed to be
// valid until the GPIO is released.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_desc {
    pub gdev: *mut gpio_device,
    pub flags: c_ulong,
// flag symbols are bit numbers

// Connection label
    pub label: *mut gpio_desc_label __rcu,
// Name of the GPIO
    pub name: *const c_char,

    pub hog: *mut device_node,

// debounce period in microseconds
    pub debounce_period_us: c_uint,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_chip_guard {
    pub gdev: *mut gpio_device,
    pub gc: *mut gpio_chip,
    pub idx: c_int,
}

extern "C" {
    pub fn gpiod_request(desc: *mut gpio_desc, label: *const c_char) -> c_int;
}
extern "C" {
    pub fn gpiod_request_commit(desc: *mut gpio_desc, label: *const c_char) -> c_int;
}
extern "C" {
    pub fn gpiod_free(desc: *mut gpio_desc);
}
extern "C" {
    pub fn gpiod_free_commit(desc: *mut gpio_desc);
}
extern "C" {
    pub fn gpio_do_set_config(desc: *mut gpio_desc, config: c_ulong) -> c_int;
}
extern "C" {
    pub fn gpio_set_debounce_timeout(desc: *mut gpio_desc, debounce: c_uint) -> c_int;
}
extern "C" {
    pub fn gpiochip_add_hog(gc: *mut gpio_chip, fwnode: *mut fwnode_handle) -> c_int;
}
extern "C" {
    pub fn gpiochip_get_ngpios(gc: *mut gpio_chip, dev: *mut device) -> c_int;
}
// With descriptor prefix

// With chip prefix

