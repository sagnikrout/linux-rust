//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/fs.h
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
// Copyright (c) 2013-2020, Mellanox Technologies inc. All rights reserved.
//

extern "C" {
    pub fn mlx5_ib_fs_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_fs_cleanup_anchor(dev: *mut mlx5_ib_dev);
}
// When a steering anchor is created, a special flow table is also
// created for the user to reference. Since the user can reference it,
// the kernel cannot trust that when the user destroys the steering
// anchor, they no longer reference the flow table.
//
// To address this issue, when a user destroys a steering anchor, only
// the flow steering rule in the table is destroyed, but the table
// itself is kept to deal with the above scenario. The remaining
// resources are only removed when the RDMA device is destroyed, which
// is a safe assumption that all references are gone.
//
