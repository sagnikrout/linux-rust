//! Automatically rewritten from C Header to Rust Module
//! Source: tools/perf/util/addr_location.h
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
pub const __PERF_ADDR_LOCATION: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_location {
    pub thread: *mut thread,
    pub map: *mut map,
    pub sym: *mut symbol,
    pub srcline: *const c_char,
    pub addr: u64,
    pub level: c_char,
    pub cpumode: u8,
    pub filtered: u16,
    pub cpu: i32,
    pub socket: i32,
// Same as machine.parallelism but within [1, nr_cpus].
    pub parallelism: c_int,
// See he_stat.latency.
    pub latency: u64,
}

extern "C" {
    pub fn addr_location__init(al: *mut addr_location);
}
extern "C" {
    pub fn addr_location__exit(al: *mut addr_location);
}
extern "C" {
    pub fn addr_location__copy(dst: *mut addr_location, src: *mut addr_location);
}
