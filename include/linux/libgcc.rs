//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/libgcc.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// include/lib/libgcc.h
//

extern "C" {
    pub fn __attribute__((__word__)): (mode) -> typedef int word_type;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DWstruct {
    pub low: int high,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct DWstruct {
    pub high: int low,,
}

extern "C" {
    pub fn __ashldi3(u: c_longlong, b: word_type) -> long long notrace;
}
extern "C" {
    pub fn __ashrdi3(u: c_longlong, b: word_type) -> long long notrace;
}
extern "C" {
    pub fn __cmpdi2(a: c_longlong, b: c_longlong) -> word_type notrace;
}
extern "C" {
    pub fn __lshrdi3(u: c_longlong, b: word_type) -> long long notrace;
}
extern "C" {
    pub fn __muldi3(u: c_longlong, v: c_longlong) -> long long notrace;
}
extern "C" {
    pub fn __ucmpdi2(a: c_ulonglong, b: c_ulonglong) -> word_type notrace;
}

