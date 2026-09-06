//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/netfilter_bridge/ebt_limit.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// timings are in milliseconds.
pub const EBT_LIMIT_SCALE: c_int = 10000;
// 1/10,000 sec period => max of 10,000/sec.  Min rate is then 429490
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ebt_limit_info {
    pub /: *mut *mut *mut __u32 avg; / Average secs between packets  scale,
    pub /: *mut *mut __u32 burst; / Period multiplier for upper limit.,
// Used internally by the kernel
    pub prev: c_ulong,
    pub credit: __u32,
    pub cost: __u32 credit_cap,,
}
