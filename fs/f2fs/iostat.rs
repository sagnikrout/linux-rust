//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/iostat.h
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
// Copyright 2021 Google LLC
// Author: Daeho Jeong <daehojeong@google.com>
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iostat_lat_type {
    READ_IO = 0,
    WRITE_SYNC_IO,
    WRITE_ASYNC_IO,
    MAX_IO_TYPE,
}

pub const NUM_PREALLOC_IOSTAT_CTXS: c_int = 128;
pub const DEFAULT_IOSTAT_PERIOD_MS: c_int = 3000;
pub const MIN_IOSTAT_PERIOD_MS: c_int = 100;
// maximum period of iostat tracing is 1 day
pub const MAX_IOSTAT_PERIOD_MS: c_int = 8640000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct iostat_lat_info {
    pub /: *mut *mut unsigned long sum_lat[MAX_IO_TYPE][NR_PAGE_TYPE]; / sum of io latencies,
    pub /: *mut *mut unsigned long peak_lat[MAX_IO_TYPE][NR_PAGE_TYPE]; / peak io latency,
    pub /: *mut *mut unsigned int bio_cnt[MAX_IO_TYPE][NR_PAGE_TYPE]; / bio count,
}

extern "C" {
    pub fn f2fs_reset_iostat(sbi: *mut f2fs_sb_info);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_iostat_ctx {
    pub sbi: *mut f2fs_sb_info,
    pub submit_ts: c_ulong,
    pub type: page_type,
    pub post_read_ctx: *mut bio_post_read_ctx,
}

extern "C" {
    pub fn iostat_update_and_unbind_ctx(bio: *mut bio);
}
extern "C" {
    pub fn f2fs_init_iostat_processing() -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_iostat_processing();
}
extern "C" {
    pub fn f2fs_init_iostat(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_iostat(sbi: *mut f2fs_sb_info);
}

