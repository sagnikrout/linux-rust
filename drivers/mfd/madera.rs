//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mfd/madera.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// MFD internals for Cirrus Logic Madera codecs
//
// Copyright 2015-2018 Cirrus Logic
//

extern "C" {
    pub fn madera_dev_init(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn madera_dev_exit(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn cs47l15_patch(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn cs47l35_patch(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn cs47l85_patch(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn cs47l90_patch(madera: *mut madera) -> c_int;
}
extern "C" {
    pub fn cs47l92_patch(madera: *mut madera) -> c_int;
}
