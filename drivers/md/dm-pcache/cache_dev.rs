//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/cache_dev.h
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

pub const PCACHE_MAGIC: c_uint = 0x65B05EFA96C596EFULL;

//
// PCACHE SB flags configured during formatting
//
// The PCACHE_SB_F_xxx flags define registration requirements based on cache_dev
// formatting. For a machine to register a cache_dev:
// - PCACHE_SB_F_BIGENDIAN: Requires a big-endian machine.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_sb {
    pub crc: __le32,
    pub flags: __le32,
    pub magic: __le64,
    pub seg_num: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_cache_dev {
    pub sb_flags: u32,
    pub seg_num: u32,
    pub mapping: *mut c_void,
    pub use_vmap: bool,
    pub dm_dev: *mut dm_dev,
    pub seg_lock: mutex,
    pub seg_bitmap: *mut c_ulong,
}

extern "C" {
    pub fn cache_dev_start(pcache: *mut dm_pcache) -> c_int;
}
extern "C" {
    pub fn cache_dev_stop(pcache: *mut dm_pcache);
}
extern "C" {
    pub fn cache_dev_zero_range(cache_dev: *mut pcache_cache_dev, pos: *mut c_void, size: u32);
}
extern "C" {
    pub fn cache_dev_get_empty_segment_id(cache_dev: *mut pcache_cache_dev, seg_id: *mut u32) -> c_int;
}
