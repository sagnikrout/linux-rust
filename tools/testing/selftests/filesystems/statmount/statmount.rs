//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/filesystems/statmount/statmount.h
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

pub const __NR_statmount: c_int = 567;

pub const __NR_statmount: c_int = 4457;

pub const __NR_statmount: c_int = 6457;

pub const __NR_statmount: c_int = 5457;

pub const __NR_statmount: c_int = 457;

pub const __NR_listmount: c_int = 568;

pub const __NR_listmount: c_int = 4458;

pub const __NR_listmount: c_int = 6458;

pub const __NR_listmount: c_int = 5458;

pub const __NR_listmount: c_int = 458;

extern "C" {
    pub fn syscall(_arg: __NR_statmount, _arg: &req, _arg: buf, _arg: bufsize, _arg: flags) -> return;
}
extern "C" {
    pub fn syscall(_arg: __NR_listmount, _arg: &req, _arg: list, _arg: num, _arg: flags) -> return;
}
