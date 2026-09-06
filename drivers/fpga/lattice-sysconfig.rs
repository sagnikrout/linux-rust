//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/fpga/lattice-sysconfig.h
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

pub const SYSCONFIG_POLL_INTERVAL_US: c_int = 30;
pub const SYSCONFIG_POLL_BUSY_TIMEOUT_US: c_int = 1000000;
pub const SYSCONFIG_POLL_GPIO_TIMEOUT_US: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sysconfig_priv {
    pub program: *mut gpio_desc,
    pub init: *mut gpio_desc,
    pub done: *mut gpio_desc,
    pub dev: *mut device,
    pub rx_len): *mut *mut size_t tx_len, void rx_buf, size_t,
    pub priv): *mut *mut int (bitstream_burst_write_init)(struct sysconfig_priv,
    pub tx_len): *const *const char tx_buf, size_t,
    pub priv): *mut *mut int (bitstream_burst_write_complete)(struct sysconfig_priv,
}

extern "C" {
    pub fn sysconfig_probe(priv: *mut sysconfig_priv) -> c_int;
}
