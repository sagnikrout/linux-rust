//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/gc.h
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
// fs/f2fs/gc.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// a threshold to determine
// whether IO subsystem is idle
// or not
//

pub const DEF_GC_THREAD_MAX_SLEEP_TIME: c_int = 60000;

// GC sleep parameters for zoned deivces
pub const DEF_GC_THREAD_MIN_SLEEP_TIME_ZONED: c_int = 10;
pub const DEF_GC_THREAD_MAX_SLEEP_TIME_ZONED: c_int = 20;
pub const DEF_GC_THREAD_NOGC_SLEEP_TIME_ZONED: c_int = 60000;
// choose candidates from sections which has age of more than 7 days

pub const DEF_MIGRATION_WINDOW_GRANULARITY_ZONED: c_int = 3;
pub const BOOST_GC_MULTIPLE: c_int = 5;
pub const ZONED_PIN_SEC_REQUIRED_COUNT: c_int = 1;
pub const DEF_GC_FAILED_PINNED_FILES: c_int = 2048;

// Search max. number of dirty segments to select a victim segment

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gc_inode_list {
    pub ilist: list_head,
    pub iroot: radix_tree_root,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct victim_entry {
    pub /: *mut *mut rb_node rb_node; / rb node located in rb-tree,
    pub /: *mut *mut unsigned long long mtime; / mtime of section,
    pub /: *mut *mut unsigned int segno; / segment No.,
    pub list: list_head,
}

//
// inline functions
//
// On a Zoned device zone-capacity can be less than zone-size and if
// zone-capacity is not aligned to f2fs segment size(2MB), then the segment
// starting just before zone-capacity has some blocks spanning across the
// zone-capacity, these blocks are not usable.
// Such spanning segments can be in free list so calculate the sum of usable
// blocks in currently free segments including normal and spanning segments.
//
extern "C" {
    pub fn free_segs_blk_count_zoned(_arg: sbi) -> return;
}
extern "C" {
    pub fn SEGS_TO_BLKS(_arg: sbi, _arg: free_segments(sbi)) -> return;
}
// wait = max_time;
// wait += min_time;
// wait = gc_th->max_sleep_time;
// wait = min_time;
// wait -= min_time;
extern "C" {
    pub fn free_sections(100: *mut *mut sbi) > ((sbi->total_sections  limit_perc) /) -> return;
}
//
// Background GC is triggered with the following conditions.
// 1. There are a number of invalid blocks.
// 2. There is not enough free space.
//
extern "C" {
    pub fn has_enough_invalid_blocks(_arg: sbi) -> return;
}
