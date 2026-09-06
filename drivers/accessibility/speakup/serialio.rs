//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/accessibility/speakup/serialio.h
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
// this is cut&paste from 8250.h. Get rid of the structure, the definitions
// and this whole broken driver.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct old_serial_port {
    pub /: *mut *mut unsigned int uart; / unused,
    pub baud_base: c_uint,
    pub port: c_uint,
    pub irq: c_uint,
    pub /: *mut *mut upf_t flags; / unused,
}

// countdown values for serial timeouts in us

// countdown values transmitter/dsr timeouts in us
pub const SPK_XMITR_TIMEOUT: c_int = 100000;
// countdown values cts timeouts in us
pub const SPK_CTS_TIMEOUT: c_int = 100000;
// check ttyS0 ... ttyS3
pub const SPK_LO_TTY: c_int = 0;
pub const SPK_HI_TTY: c_int = 3;
// # of timeouts permitted before disable
pub const NUM_DISABLE_TIMEOUTS: c_int = 3;
// buffer timeout in ms
pub const SPK_TIMEOUT: c_int = 100;

