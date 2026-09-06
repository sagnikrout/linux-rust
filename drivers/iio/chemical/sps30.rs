//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/iio/chemical/sps30.h
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
#[derive(Copy, Clone)]
pub struct sps30_ops {
    pub state): *mut *mut int (start_meas)(struct sps30_state,
    pub state): *mut *mut int (stop_meas)(struct sps30_state,
    pub num): *mut *mut *mut *mut int (read_meas)(struct sps30_state state, __be32 meas, size_t,
    pub state): *mut *mut int (reset)(struct sps30_state,
    pub state): *mut *mut int (clean_fan)(struct sps30_state,
    pub period): *mut *mut *mut int (read_cleaning_period)(struct sps30_state state, __be32,
    pub period): *mut *mut *mut int (write_cleaning_period)(struct sps30_state state, __be32,
    pub state): *mut *mut int (show_info)(struct sps30_state,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sps30_state {
// serialize access to the device
    pub lock: mutex,
    pub dev: *mut device,
    pub state: c_int,
//
// priv pointer is solely for serdev driver private data. We keep it
// here because driver_data inside dev has been already used for iio and
// struct serdev_device doesn't have one.
//
    pub priv: *mut c_void,
    pub ops: *const sps30_ops,
}

extern "C" {
    pub fn sps30_probe(dev: *mut device, name: *const c_char, priv: *mut c_void, ops: *const sps30_ops) -> c_int;
}
