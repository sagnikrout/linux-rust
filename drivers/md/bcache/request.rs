//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/bcache/request.h
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
pub struct data_insert_op {
    pub cl: closure,
    pub c: *mut cache_set,
    pub bio: *mut bio,
    pub wq: *mut workqueue_struct,
    pub inode: c_uint,
    pub write_point: u16,
    pub write_prio: u16,
    pub status: blk_status_t,
    pub flags: u16,
    pub bypass:1: c_uint,
    pub writeback:1: c_uint,
    pub flush_journal:1: c_uint,
    pub csum:1: c_uint,
    pub replace:1: c_uint,
    pub replace_collision:1: c_uint,
    pub insert_data_done:1: c_uint,
}

extern "C" {
    pub fn bch_get_congested(c: *const cache_set) -> c_uint;
}
extern "C" {
    pub fn bch_cached_dev_request_init(dc: *mut cached_dev);
}
extern "C" {
    pub fn cached_dev_submit_bio(bio: *mut bio);
}
extern "C" {
    pub fn bch_flash_dev_request_init(d: *mut bcache_device);
}
extern "C" {
    pub fn flash_dev_submit_bio(bio: *mut bio);
}
