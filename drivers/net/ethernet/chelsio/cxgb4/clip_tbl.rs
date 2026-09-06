//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/chelsio/cxgb4/clip_tbl.h
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


//
// This file is part of the Chelsio T4 Ethernet driver for Linux.
// Copyright (C) 2003-2014 Chelsio Communications.  All rights reserved.
//
// Written by Deepak (deepak.s@chelsio.com)
//
// This program is distributed in the hope that it will be useful, but WITHOUT
// ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
// FITNESS FOR A PARTICULAR PURPOSE.  See the LICENSE file included in this
// release for licensing terms and conditions.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clip_entry {
    pub /: *mut *mut spinlock_t lock; / Hold while modifying clip reference,
    pub refcnt: refcount_t,
    pub list: list_head,
    pub addr: sockaddr_in,
    pub addr6: sockaddr_in6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct clip_tbl {
    pub clipt_start: c_uint,
    pub clipt_size: c_uint,
    pub lock: rwlock_t,
    pub nfree: core::sync::atomic::AtomicI32,
    pub ce_free_head: list_head,
    pub cl_list: *mut c_void,
    pub __counted_by(clipt_size): list_head hash_list[],
}

extern "C" {
    pub fn cxgb4_clip_get(dev: *const net_device, lip: *const u32, v6: u8) -> c_int;
}
extern "C" {
    pub fn cxgb4_clip_release(dev: *const net_device, lip: *const u32, v6: u8);
}
extern "C" {
    pub fn clip_tbl_show(seq: *mut seq_file, v: *mut c_void) -> c_int;
}
extern "C" {
    pub fn cxgb4_update_root_dev_clip(dev: *mut net_device) -> c_int;
}
extern "C" {
    pub fn t4_cleanup_clip_tbl(adap: *mut adapter);
}
