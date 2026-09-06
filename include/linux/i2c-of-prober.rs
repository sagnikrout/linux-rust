//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/i2c-of-prober.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Definitions for the Linux I2C OF component prober
//
// Copyright (C) 2024 Google LLC
//

//
// struct i2c_of_probe_ops - I2C OF component prober callbacks
//
// A set of callbacks to be used by i2c_of_probe_component().
//
// All callbacks are optional. Callbacks are called only once per run, and are
// used in the order they are defined in this structure.
//
// All callbacks that have return values shall return %0 on success,
// or a negative error number on failure.
//
// The @dev parameter passed to the callbacks is the same as @dev passed to
// i2c_of_probe_component(). It should only be used for dev_printk() calls
// and nothing else, especially not managed device resource (devres) APIs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_of_probe_ops {
//
// @enable: Retrieve and enable resources so that the components respond to probes.
//
// It is OK for this callback to return -EPROBE_DEFER since the intended use includes
// retrieving resources and enables them. Resources should be reverted to their initial
// state and released before returning if this fails.
//
    pub data): *mut *mut *mut *mut int (enable)(struct device dev, struct device_node bus_node, void,
//
// @cleanup_early: Release exclusive resources prior to calling probe() on a
// detected component.
//
// Only called if a matching component is actually found. If none are found,
// resources that would have been released in this callback should be released in
// @free_resourcs_late instead.
//
    pub data): *mut *mut *mut void (cleanup_early)(struct device dev, void,
//
// @cleanup: Opposite of @enable to balance refcounts and free resources after probing.
//
// Should check if resources were already freed by @cleanup_early.
//
    pub data): *mut *mut *mut void (cleanup)(struct device dev, void,
}

//
// struct i2c_of_probe_cfg - I2C OF component prober configuration
// @ops: Callbacks for the prober to use.
// @type: A string to match the device node name prefix to probe for.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_of_probe_cfg {
    pub ops: *const i2c_of_probe_ops,
    pub type: *const c_char,
}

extern "C" {
    pub fn i2c_of_probe_component(dev: *mut device, cfg: *const i2c_of_probe_cfg, ctx: *mut c_void) -> c_int;
}
//
// DOC: I2C OF component prober simple helpers
//
// Components such as trackpads are commonly connected to a devices baseboard
// with a 6-pin ribbon cable. That gives at most one voltage supply and one
// GPIO (commonly a "enable" or "reset" line) besides the I2C bus, interrupt
// pin, and common ground. Touchscreens, while integrated into the display
// panel's connection, typically have the same set of connections.
//
// A simple set of helpers are provided here for use with the I2C OF component
// prober. This implementation targets such components, allowing for at most
// one regulator supply.
//
// The following helpers are provided:
// * i2c_of_probe_simple_enable()
// * i2c_of_probe_simple_cleanup_early()
// * i2c_of_probe_simple_cleanup()
//
// struct i2c_of_probe_simple_opts - Options for simple I2C component prober callbacks
// @res_node_compatible: Compatible string of device node to retrieve resources from.
// @supply_name: Name of regulator supply.
// @gpio_name: Name of GPIO. NULL if no GPIO line is used. Empty string ("") if GPIO
// line is unnamed.
// @post_power_on_delay_ms: Delay after regulators are powered on. Passed to msleep().
// @post_gpio_config_delay_ms: Delay after GPIO is configured. Passed to msleep().
// @gpio_assert_to_enable: %true if GPIO should be asserted, i.e. set to logical high,
// to enable the component.
//
// This describes power sequences common for the class of components supported by the
// simple component prober:
// * @gpio_name is configured to the non-active setting according to @gpio_assert_to_enable.
// * @supply_name regulator supply is enabled.
// * Wait for @post_power_on_delay_ms to pass.
// * @gpio_name is configured to the active setting according to @gpio_assert_to_enable.
// * Wait for @post_gpio_config_delay_ms to pass.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_of_probe_simple_opts {
    pub res_node_compatible: *const c_char,
    pub supply_name: *const c_char,
    pub gpio_name: *const c_char,
    pub post_power_on_delay_ms: c_uint,
    pub post_gpio_config_delay_ms: c_uint,
    pub gpio_assert_to_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct i2c_of_probe_simple_ctx {
// public: provided by user before helpers are used.
    pub opts: *const i2c_of_probe_simple_opts,
// private: internal fields for helpers.
    pub supply: *mut regulator,
    pub gpiod: *mut gpio_desc,
}

extern "C" {
    pub fn i2c_of_probe_simple_enable(dev: *mut device, bus_node: *mut device_node, data: *mut c_void) -> c_int;
}
extern "C" {
    pub fn i2c_of_probe_simple_cleanup_early(dev: *mut device, data: *mut c_void);
}
extern "C" {
    pub fn i2c_of_probe_simple_cleanup(dev: *mut device, data: *mut c_void);
}

