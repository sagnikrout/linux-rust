//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/gpio/legacy.h
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
// This is the LEGACY GPIO include file, used only for legacy APIs.
//
// No new code should use this, but instead use the linux/gpio/consumer.h
// interfaces directly.
//

// make these flag values available regardless of GPIO kconfig options

//
// "valid" GPIO numbers are nonnegative and may be passed to
// setup routines like gpio_request().  Only some valid numbers
// can successfully be requested and used.
//
// Invalid GPIO numbers are useful for indicating no-such-GPIO in
// platform data and other tables.
//
// only non-negative numbers are valid
//
// Platforms may implement their GPIO interface with library code,
// at a small performance cost for non-inlined operations and some
// extra memory (for code and for per-GPIO table entries).
//
// Always use the library code for GPIO management calls,
// or when sleeping may be involved.
//
extern "C" {
    pub fn gpio_request(gpio: unsigned, label: *const c_char) -> c_int;
}
extern "C" {
    pub fn gpio_free(gpio: unsigned);
}
extern "C" {
    pub fn gpiod_direction_input(_arg: gpio_to_desc(gpio)) -> return;
}
extern "C" {
    pub fn gpiod_direction_output_raw(_arg: gpio_to_desc(gpio), _arg: value) -> return;
}
extern "C" {
    pub fn gpiod_get_raw_value_cansleep(_arg: gpio_to_desc(gpio)) -> return;
}
extern "C" {
    pub fn gpiod_get_raw_value(_arg: gpio_to_desc(gpio)) -> return;
}
extern "C" {
    pub fn gpiod_to_irq(_arg: gpio_to_desc(gpio)) -> return;
}
extern "C" {
    pub fn gpio_request_one(gpio: unsigned, flags: c_ulong, label: *const c_char) -> c_int;
}

// GPIO can never have been requested
// GPIO can never have been requested or set as {in,out}put
// GPIO can never have been requested or set as output
// GPIO can never have been requested or set as {in,out}put
// GPIO can never have been requested or set as output
// GPIO can never have been requested or set as input

