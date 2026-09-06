//! Automatically rewritten from C Header to Rust Module
//! Source: tools/lib/api/fs/fs.h
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
// On most systems <limits.h> would have given us this, but  not on some systems
// (e.g. GNU/Hurd).
//

pub const PATH_MAX: c_int = 4096;

//
// The xxxx__mountpoint() entry points find the first match mount point for each
// filesystems listed below, where xxxx is the filesystem type.
//
// The interface is as follows:
//
// - If a mount point is found on first call, it is cached and used for all
// subsequent calls.
//
// - If a mount point is not found, NULL is returned on first call and all
// subsequent calls.
//

extern "C" {
    pub fn cgroupfs_find_mountpoint(buf: *mut c_char, maxlen: usize, subsys: *const c_char) -> c_int;
}
extern "C" {
    pub fn filename__read_int(filename: *const c_char, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn filename__read_ull(filename: *const c_char, value: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn filename__read_xll(filename: *const c_char, value: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn filename__read_str(filename: *const c_char, buf: *mut c_char, sizep: *mut usize) -> c_int;
}
extern "C" {
    pub fn filename__write_int(filename: *const c_char, value: c_int) -> c_int;
}
extern "C" {
    pub fn procfs__read_str(entry: *const c_char, buf: *mut c_char, sizep: *mut usize) -> c_int;
}
extern "C" {
    pub fn sysctl__read_int(sysctl: *const c_char, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn sysfs__read_int(entry: *const c_char, value: *mut c_int) -> c_int;
}
extern "C" {
    pub fn sysfs__read_ull(entry: *const c_char, value: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn sysfs__read_xll(entry: *const c_char, value: *mut c_ulonglong) -> c_int;
}
extern "C" {
    pub fn sysfs__read_str(entry: *const c_char, buf: *mut c_char, sizep: *mut usize) -> c_int;
}
extern "C" {
    pub fn sysfs__read_bool(entry: *const c_char, value: *mut bool) -> c_int;
}
extern "C" {
    pub fn sysfs__write_int(entry: *const c_char, value: c_int) -> c_int;
}
