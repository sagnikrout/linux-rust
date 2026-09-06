//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/platform/chrome/cros_ec_proto_test_util.h
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
// CrOS Kunit tests utilities.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ec_xfer_mock {
    pub list: list_head,
    pub test: *mut kunit,
// input
    pub i_data: *mut c_void,
// output
    pub ret: c_int,
    pub result: c_int,
    pub o_data: *mut c_void,
    pub o_data_len: u32,
// input
// Must be last -ends in a flexible-array member.
    pub msg: cros_ec_command,
}

extern "C" {
    pub fn cros_kunit_ec_xfer_mock(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> c_int;
}
extern "C" {
    pub fn cros_kunit_ec_cmd_xfer_mock(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> c_int;
}
extern "C" {
    pub fn cros_kunit_ec_pkt_xfer_mock(ec_dev: *mut cros_ec_device, msg: *mut cros_ec_command) -> c_int;
}
extern "C" {
    pub fn cros_kunit_mock_reset();
}
