//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/crush/hash.h
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

pub const CRUSH_HASH_RJENKINS1: c_int = 0;

extern "C" {
    pub fn crush_hash32(type: c_int, a: __u32) -> __u32;
}
extern "C" {
    pub fn crush_hash32_2(type: c_int, a: __u32, b: __u32) -> __u32;
}
extern "C" {
    pub fn crush_hash32_3(type: c_int, a: __u32, b: __u32, c: __u32) -> __u32;
}
extern "C" {
    pub fn crush_hash32_4(type: c_int, a: __u32, b: __u32, c: __u32, d: __u32) -> __u32;
}
