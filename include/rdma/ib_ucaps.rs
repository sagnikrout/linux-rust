//! Automatically rewritten from C Header to Rust Module
//! Source: include/rdma/ib_ucaps.h
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
// Copyright (c) 2025, NVIDIA CORPORATION & AFFILIATES. All rights reserved
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rdma_user_cap {
    RDMA_UCAP_MLX5_CTRL_LOCAL,
    RDMA_UCAP_MLX5_CTRL_OTHER_VHCA,
    RDMA_UCAP_MAX
}

extern "C" {
    pub fn ib_get_ucaps(fds: *mut c_int, fd_count: c_int, idx_mask: *mut u64) -> c_int;
}

extern "C" {
    pub fn ib_create_ucap(type: rdma_user_cap) -> c_int;
}
extern "C" {
    pub fn ib_remove_ucap(type: rdma_user_cap);
}

