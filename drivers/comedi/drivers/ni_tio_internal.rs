//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_tio_internal.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Header file for NI general purpose counter support code (ni_tio.c and
// ni_tiocmd.c)
//
// COMEDI - Linux Control and Measurement Device Interface
//

pub const GI_AUTO_INC_MASK: c_uint = 0xff;

extern "C" {
    pub fn ni_tio_read(counter: *mut ni_gpct, ni_gpct_register: enum) -> c_uint;
}
// m series and 660x variants have counting mode registers
extern "C" {
    pub fn ni_tio_arm(counter: *mut ni_gpct, arm: bool, start_trigger: c_uint) -> c_int;
}
