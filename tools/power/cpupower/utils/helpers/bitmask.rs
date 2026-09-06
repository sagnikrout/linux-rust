//! Automatically rewritten from C Header to Rust Module
//! Source: tools/power/cpupower/utils/helpers/bitmask.h
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
// Taken over from libbitmask, a project initiated from sgi:
// Url:            http://oss.sgi.com/projects/cpusets
// Unfortunately it's not very widespread, therefore relevant parts are
// pasted here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bitmask {
    pub size: c_uint,
    pub maskp: *mut c_ulong,
}

extern "C" {
    pub fn bitmask_free(bmp: *mut bitmask);
}
extern "C" {
    pub fn bitmask_first(bmp: *const bitmask) -> c_uint;
}
extern "C" {
    pub fn bitmask_next(bmp: *const bitmask, i: c_uint) -> c_uint;
}
extern "C" {
    pub fn bitmask_last(bmp: *const bitmask) -> c_uint;
}
extern "C" {
    pub fn bitmask_isallclear(bmp: *const bitmask) -> c_int;
}
extern "C" {
    pub fn bitmask_isbitset(bmp: *const bitmask, i: c_uint) -> c_int;
}
extern "C" {
    pub fn bitmask_parselist(buf: *const c_char, bmp: *mut bitmask) -> c_int;
}
extern "C" {
    pub fn bitmask_displaylist(buf: *mut c_char, len: c_int, bmp: *const bitmask) -> c_int;
}
