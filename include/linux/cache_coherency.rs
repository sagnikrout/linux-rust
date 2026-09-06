//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/cache_coherency.h
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
// Cache coherency maintenance operation device drivers
//
// Copyright Huawei 2025
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cc_inval_params {
    pub addr: phys_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_coherency_ops {
    pub invp): *mut cc_inval_params,
    pub cci): *mut *mut int (done)(struct cache_coherency_ops_inst,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cache_coherency_ops_inst {
    pub kref: kref,
    pub node: list_head,
    pub ops: *const cache_coherency_ops,
}

extern "C" {
    pub fn cache_coherency_ops_instance_register(cci: *mut cache_coherency_ops_inst) -> c_int;
}
extern "C" {
    pub fn cache_coherency_ops_instance_unregister(cci: *mut cache_coherency_ops_inst);
}
//
// cache_coherency_ops_instance_alloc - Allocate cache coherency ops instance
// @ops: Cache maintenance operations
// @drv_struct: structure that contains the struct cache_coherency_ops_inst
// @member: Name of the struct cache_coherency_ops_inst member in @drv_struct.
//
// This allocates a driver specific structure and initializes the
// cache_coherency_ops_inst embedded in the drv_struct. Upon success the
// pointer must be freed via cache_coherency_ops_instance_put().
//
// Returns a &drv_struct * on success, %NULL on error.
//

extern "C" {
    pub fn cache_coherency_ops_instance_put(cci: *mut cache_coherency_ops_inst);
}
