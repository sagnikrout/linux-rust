//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/machine.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum gpio_lookup_flags {
    GPIO_ACTIVE_HIGH		= (0 << 0),
    GPIO_ACTIVE_LOW			= (1 << 0),
    GPIO_OPEN_DRAIN			= (1 << 1),
    GPIO_OPEN_SOURCE		= (1 << 2),
    GPIO_PERSISTENT			= (0 << 3),
    GPIO_TRANSITORY			= (1 << 3),
    GPIO_PULL_UP			= (1 << 4),
    GPIO_PULL_DOWN			= (1 << 5),
    GPIO_PULL_DISABLE		= (1 << 6),

    GPIO_LOOKUP_FLAGS_DEFAULT	= GPIO_ACTIVE_HIGH | GPIO_PERSISTENT,
}

//
// struct gpiod_lookup - lookup table
// @key: either the name of the chip the GPIO belongs to, or the GPIO line name
// Note that GPIO line names are not guaranteed to be globally unique,
// so this will use the first match found!
// @chip_hwnum: hardware number (i.e. relative to the chip) of the GPIO, or
// U16_MAX to indicate that @key is a GPIO line name
// @con_id: name of the GPIO from the device's point of view
// @idx: index of the GPIO in case several GPIOs share the same name
// @flags: bitmask of gpio_lookup_flags GPIO_* values
//
// gpiod_lookup is a lookup table for associating GPIOs to specific devices and
// functions using platform data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpiod_lookup {
    pub key: *const c_char,
    pub chip_hwnum: u16,
    pub con_id: *const c_char,
    pub idx: c_uint,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gpiod_lookup_table {
    pub list: list_head,
    pub dev_id: *const c_char,
    pub table: [gpiod_lookup; ],
}

//
// Helper for lookup tables with just one single lookup for a device.
//

//
// Simple definition of a single GPIO under a con_id
//

//
// Use this macro if you need to have several GPIOs under the same con_id.
// Each GPIO needs to use a different index and can be accessed using
// gpiod_get_index()
//

extern "C" {
    pub fn gpiod_add_lookup_table(table: *mut gpiod_lookup_table);
}
extern "C" {
    pub fn gpiod_add_lookup_tables(tables: *mut gpiod_lookup_table, n: usize);
}
extern "C" {
    pub fn gpiod_remove_lookup_table(table: *mut gpiod_lookup_table);
}

