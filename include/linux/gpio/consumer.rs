//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/consumer.h
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
// struct gpio_descs - Struct containing an array of descriptors that can be
// obtained using gpiod_get_array()
//
// @info:	Pointer to the opaque gpio_array structure
// @ndescs:	Number of held descriptors
// @desc:	Array of pointers to GPIO descriptors
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpio_descs {
    pub info: *mut gpio_array,
    pub ndescs: c_uint,
    pub desc: [*mut gpio_desc; ],
}

// GPIOD_FLAGS_BIT_NONEXCLUSIVE is DEPRECATED, don't use in new code.

//
// enum gpiod_flags - Optional flags that can be passed to one of gpiod_* to
// configure direction and output value. These values
// cannot be OR'd.
//
// @GPIOD_ASIS:			Don't change anything
// @GPIOD_IN:			Set lines to input mode
// @GPIOD_OUT_LOW:		Set lines to output and drive them low
// @GPIOD_OUT_HIGH:		Set lines to output and drive them high
// @GPIOD_OUT_LOW_OPEN_DRAIN:	Set lines to open-drain output and drive them low
// @GPIOD_OUT_HIGH_OPEN_DRAIN:	Set lines to open-drain output and drive them high
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpiod_flags {
    GPIOD_ASIS	= 0,
    GPIOD_IN	= GPIOD_FLAGS_BIT_DIR_SET,
    GPIOD_OUT_LOW	= GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT,
    GPIOD_OUT_HIGH	= GPIOD_FLAGS_BIT_DIR_SET | GPIOD_FLAGS_BIT_DIR_OUT |
    GPIOD_FLAGS_BIT_DIR_VAL,
    GPIOD_OUT_LOW_OPEN_DRAIN = GPIOD_OUT_LOW | GPIOD_FLAGS_BIT_OPEN_DRAIN,
    GPIOD_OUT_HIGH_OPEN_DRAIN = GPIOD_OUT_HIGH | GPIOD_FLAGS_BIT_OPEN_DRAIN,
}

// Return the number of GPIOs associated with a device / function
extern "C" {
    pub fn gpiod_count(dev: *mut device, con_id: *const c_char) -> c_int;
}
// Acquire and dispose GPIOs
extern "C" {
    pub fn gpiod_put(desc: *mut gpio_desc);
}
extern "C" {
    pub fn gpiod_put_array(descs: *mut gpio_descs);
}
extern "C" {
    pub fn devm_gpiod_put(dev: *mut device, desc: *mut gpio_desc);
}
extern "C" {
    pub fn devm_gpiod_unhinge(dev: *mut device, desc: *mut gpio_desc);
}
extern "C" {
    pub fn devm_gpiod_put_array(dev: *mut device, descs: *mut gpio_descs);
}
extern "C" {
    pub fn gpiod_get_direction(desc: *mut gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_is_single_ended(desc: *mut gpio_desc) -> bool;
}
extern "C" {
    pub fn gpiod_direction_input(desc: *mut gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_direction_output(desc: *mut gpio_desc, value: c_int) -> c_int;
}
extern "C" {
    pub fn gpiod_direction_output_raw(desc: *mut gpio_desc, value: c_int) -> c_int;
}
// Value get/set from non-sleeping context
extern "C" {
    pub fn gpiod_get_value(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_set_value(desc: *mut gpio_desc, value: c_int) -> c_int;
}
extern "C" {
    pub fn gpiod_get_raw_value(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_set_raw_value(desc: *mut gpio_desc, value: c_int) -> c_int;
}
// Value get/set from sleeping context
extern "C" {
    pub fn gpiod_get_value_cansleep(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: c_int) -> c_int;
}
extern "C" {
    pub fn gpiod_get_raw_value_cansleep(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_set_raw_value_cansleep(desc: *mut gpio_desc, value: c_int) -> c_int;
}
extern "C" {
    pub fn gpiod_set_config(desc: *mut gpio_desc, config: c_ulong) -> c_int;
}
extern "C" {
    pub fn gpiod_set_debounce(desc: *mut gpio_desc, debounce: c_uint) -> c_int;
}
extern "C" {
    pub fn gpiod_toggle_active_low(desc: *mut gpio_desc);
}
extern "C" {
    pub fn gpiod_is_active_low(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_cansleep(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_to_irq(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_set_consumer_name(desc: *mut gpio_desc, name: *const c_char) -> c_int;
}
extern "C" {
    pub fn gpiod_is_shared(desc: *const gpio_desc) -> bool;
}
// Convert between the old gpio_ and new gpiod_ interfaces
extern "C" {
    pub fn desc_to_gpio(desc: *const gpio_desc) -> c_int;
}
extern "C" {
    pub fn gpiod_hwgpio(desc: *const gpio_desc) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
// GPIO can never have been requested
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
// GPIO can never have been requested
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOSYS) -> return;
}

extern "C" {
    pub fn gpiod_enable_hw_timestamp_ns(desc: *mut gpio_desc, flags: c_ulong) -> c_int;
}
extern "C" {
    pub fn gpiod_disable_hw_timestamp_ns(desc: *mut gpio_desc, flags: c_ulong) -> c_int;
}

extern "C" {
    pub fn fwnode_gpiod_get_index(_arg: fwnode, _arg: con_id, _arg: 0, _arg: flags, _arg: label) -> return;
}
//
// devm_fwnode_gpiod_get_optional - obtain an optional GPIO from firmware node
// @dev:	GPIO consumer
// @fwnode:	handle of the firmware node
// @con_id:	function within the GPIO consumer
// @flags:	GPIO initialization flags
// @label:	label to attach to the requested GPIO
//
// This function can be used for drivers that get their configuration
// from opaque firmware.
//
// GPIO descriptors returned from this function are automatically disposed on
// driver detach.
//
// Returns:
// The GPIO descriptor corresponding to the optional function @con_id of device
// dev, NULL if no GPIO has been assigned to the requested function, or
// another IS_ERR() code if an error occurred while trying to acquire the GPIO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_params {
    pub crs_entry_index: c_uint,
    pub line_index: c_ushort,
    pub active_low: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acpi_gpio_mapping {
    pub name: *const c_char,
    pub data: *const acpi_gpio_params,
    pub size: c_uint,
// Ignore IoRestriction field

//
// When ACPI GPIO mapping table is in use the index parameter inside it
// refers to the GPIO resource in _CRS method. That index has no
// distinction of actual type of the resource. When consumer wants to
// get GpioIo type explicitly, this quirk may be used.
//

// Use given pin as an absolute GPIO number in the system

    pub quirks: c_uint,
}

extern "C" {
    pub fn acpi_dev_remove_driver_gpios(adev: *mut acpi_device);
}

extern "C" {
    pub fn gpiod_export(desc: *mut gpio_desc, direction_may_change: bool) -> c_int;
}
extern "C" {
    pub fn gpiod_unexport(desc: *mut gpio_desc);
}

extern "C" {
    pub fn PTR_ERR_OR_ZERO(_arg: descs) -> return;
}
