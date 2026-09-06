//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ceph/msgpool.h
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
// we use memory pools for preallocating messages we may receive, to
// avoid unexpected OOM conditions.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ceph_msgpool {
    pub name: *const c_char,
    pub pool: *mut mempool_t,
    pub /: *mut *mut int type; / preallocated message type,
    pub /: *mut *mut int front_len; / preallocated payload size,
    pub max_data_items: c_int,
}

extern "C" {
    pub fn ceph_msgpool_destroy(pool: *mut ceph_msgpool);
}
extern "C" {
    pub fn ceph_msgpool_put(: *mut ceph_msgpool, : *mut ceph_msg);
}
