//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/qp.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_qp_table {
    pub nb: notifier_block,
    pub dct_xa: xarray,
// protect radix tree
//
    pub lock: spinlock_t,
    pub tree: radix_tree_root,
}

extern "C" {
    pub fn mlx5_init_qp_table(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cleanup_qp_table(dev: *mut mlx5_ib_dev);
}
extern "C" {
    pub fn mlx5_core_destroy_qp(dev: *mut mlx5_ib_dev, qp: *mut mlx5_core_qp) -> c_int;
}
extern "C" {
    pub fn mlx5_core_destroy_dct(dev: *mut mlx5_ib_dev, dct: *mut mlx5_core_dct) -> c_int;
}
extern "C" {
    pub fn mlx5_core_set_delay_drop(dev: *mut mlx5_ib_dev, timeout_usec: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_res_put(res: *mut mlx5_core_rsc_common);
}
extern "C" {
    pub fn mlx5_core_xrcd_alloc(dev: *mut mlx5_ib_dev, xrcdn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5_core_xrcd_dealloc(dev: *mut mlx5_ib_dev, xrcdn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_qp_set_counter(qp: *mut ib_qp, counter: *mut rdma_counter) -> c_int;
}
extern "C" {
    pub fn mlx5_ib_qp_event_init() -> c_int;
}
extern "C" {
    pub fn mlx5_ib_qp_event_cleanup();
}
extern "C" {
    pub fn mlx5r_ib_rate(dev: *mut mlx5_ib_dev, rate: u8) -> c_int;
}
