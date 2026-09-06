//! Automatically rewritten from C Header to Rust Module
//! Source: fs/xfs/xfs_mru_cache.h
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
// Copyright (c) 2006-2007 Silicon Graphics, Inc.
// All Rights Reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct xfs_mru_cache_elem {
    pub list_node: list_head,
    pub key: c_ulong,
}

// Function pointer type for callback to free a client's data pointer.
extern "C" {
    pub fn void(: *mut *mut xfs_mru_cache_free_func_t)(void, : *mut xfs_mru_cache_elem) -> typedef;
}
extern "C" {
    pub fn xfs_mru_cache_init() -> c_int;
}
extern "C" {
    pub fn xfs_mru_cache_uninit();
}
extern "C" {
    pub fn xfs_mru_cache_destroy(mru: *mut xfs_mru_cache);
}
extern "C" {
    pub fn xfs_mru_cache_delete(mru: *mut xfs_mru_cache, key: c_ulong);
}
extern "C" {
    pub fn xfs_mru_cache_done(mru: *mut xfs_mru_cache);
}
