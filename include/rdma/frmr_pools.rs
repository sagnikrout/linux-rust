//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/frmr_pools.h
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_frmr_key {
    pub vendor_key: u64,
// A pool with non-zero kernel_vendor_key is a kernel-only pool.
    pub kernel_vendor_key: u64,
    pub num_dma_blocks: usize,
    pub access_flags: c_int,
    pub ats:1: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_frmr_pool_ops {
    pub count): *mut *mut u32 handles, u32,
    pub count): u32,
    pub out): *mut ib_frmr_key,
}

extern "C" {
    pub fn ib_frmr_pools_cleanup(device: *mut ib_device);
}
extern "C" {
    pub fn ib_frmr_pool_pop(device: *mut ib_device, mr: *mut ib_mr) -> c_int;
}
extern "C" {
    pub fn ib_frmr_pool_push(device: *mut ib_device, mr: *mut ib_mr);
}
extern "C" {
    pub fn ib_frmr_pool_drop(mr: *mut ib_mr);
}
