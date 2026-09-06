//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/utils/helpers/sysfs.h
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

pub const MAX_LINE_LEN: c_int = 255;
pub const SYSFS_PATH_MAX: c_int = 255;
extern "C" {
    pub fn sysfs_read_file(path: *const c_char, buf: *mut c_char, buflen: usize) -> c_uint;
}
extern "C" {
    pub fn sysfs_is_cpu_online(cpu: c_uint) -> c_int;
}
extern "C" {
    pub fn sysfs_get_idlestate_count(cpu: c_uint) -> c_uint;
}
extern "C" {
    pub fn sysfs_get_sched(smt_mc: *const c_char) -> c_int;
}
extern "C" {
    pub fn sysfs_set_sched(smt_mc: *const c_char, val: c_int) -> c_int;
}
