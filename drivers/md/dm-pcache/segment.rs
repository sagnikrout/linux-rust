//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/dm-pcache/segment.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_segment_info {
    pub header: pcache_meta_header,
    pub flags: __u32,
    pub next_seg: __u32,
}

pub const PCACHE_SEGMENT_TYPE_CACHE_DATA: c_int = 1;
extern "C" {
    pub fn FIELD_GET(_arg: PCACHE_SEG_INFO_FLAGS_TYPE_MASK, _arg: seg_info->flags) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_segment_pos {
    pub /: *mut *mut *mut pcache_segment segment; / Segment associated with the position,
    pub /: *mut *mut u32 off; / Offset within the segment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_segment_init_options {
    pub type: u8,
    pub seg_id: u32,
    pub data_off: u32,
    pub seg_info: *mut pcache_segment_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcache_segment {
    pub cache_dev: *mut pcache_cache_dev,
    pub data: *mut c_void,
    pub data_size: u32,
    pub seg_id: u32,
    pub seg_info: *mut pcache_segment_info,
}
