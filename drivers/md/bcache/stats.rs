//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/stats.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_stat_collector {
    pub cache_hits: core::sync::atomic::AtomicI32,
    pub cache_misses: core::sync::atomic::AtomicI32,
    pub cache_bypass_hits: core::sync::atomic::AtomicI32,
    pub cache_bypass_misses: core::sync::atomic::AtomicI32,
    pub cache_miss_collisions: core::sync::atomic::AtomicI32,
    pub sectors_bypassed: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_stats {
    pub kobj: kobject,
    pub cache_hits: c_ulong,
    pub cache_misses: c_ulong,
    pub cache_bypass_hits: c_ulong,
    pub cache_bypass_misses: c_ulong,
    pub cache_miss_collisions: c_ulong,
    pub sectors_bypassed: c_ulong,
    pub rescale: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_accounting {
    pub cl: closure,
    pub timer: timer_list,
    pub closing: core::sync::atomic::AtomicI32,
    pub collector: cache_stat_collector,
    pub total: cache_stats,
    pub five_minute: cache_stats,
    pub hour: cache_stats,
    pub day: cache_stats,
}

extern "C" {
    pub fn bch_cache_accounting_clear(acc: *mut cache_accounting);
}
extern "C" {
    pub fn bch_cache_accounting_destroy(acc: *mut cache_accounting);
}
