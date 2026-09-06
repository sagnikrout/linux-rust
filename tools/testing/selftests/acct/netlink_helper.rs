//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/acct/netlink_helper.h
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
// Shared generic netlink helpers for the acct selftests.
//

pub const NLA_ALIGNTO: c_int = 4;

// Fail an individual test case instead of hanging the whole binary.
pub const ACCT_RCV_TIMEOUT_SEC: c_int = 2;
// remaining -= aligned_len;
extern "C" {
    pub fn netlink_open() -> c_int;
}
extern "C" {
    pub fn send_request(fd: c_int, buf: *mut c_void, len: usize) -> c_int;
}
extern "C" {
    pub fn get_family_id(fd: c_int, name: *const c_char) -> c_int;
}
