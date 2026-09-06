//! Automatically rewritten from C Header to Rust Module
//! Source: mm/damon/sysfs-common.h
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
// Common Code for DAMON Sysfs Interface
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_ul_range {
    pub kobj: kobject,
    pub min: c_ulong,
    pub max: c_ulong,
}

extern "C" {
    pub fn damon_sysfs_ul_range_release(kobj: *mut kobject);
}
//
// schemes directory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct damon_sysfs_schemes {
    pub kobj: kobject,
    pub schemes_arr: *mut damon_sysfs_scheme,
    pub nr: c_int,
}

extern "C" {
    pub fn damon_sysfs_schemes_rm_dirs(schemes: *mut damon_sysfs_schemes);
}
extern "C" {
    pub fn damon_sysfs_memcg_path_to_id(memcg_path: *mut c_char, id: *mut u64) -> c_int;
}
