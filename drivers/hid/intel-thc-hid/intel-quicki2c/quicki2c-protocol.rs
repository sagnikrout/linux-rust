//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/hid/intel-thc-hid/intel-quicki2c/quicki2c-protocol.h
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
// Copyright (c) 2024 Intel Corporation

extern "C" {
    pub fn quicki2c_set_power(qcdev: *mut quicki2c_device, power_state: hidi2c_power_state) -> c_int;
}
extern "C" {
    pub fn quicki2c_output_report(qcdev: *mut quicki2c_device, buf: *mut c_void, buf_len: usize) -> c_int;
}
extern "C" {
    pub fn quicki2c_get_device_descriptor(qcdev: *mut quicki2c_device) -> c_int;
}
extern "C" {
    pub fn quicki2c_get_report_descriptor(qcdev: *mut quicki2c_device) -> c_int;
}
extern "C" {
    pub fn quicki2c_reset(qcdev: *mut quicki2c_device) -> c_int;
}
