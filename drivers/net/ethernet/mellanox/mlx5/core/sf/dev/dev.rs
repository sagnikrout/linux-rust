//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/dev/dev.h
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
// Copyright (c) 2020 Mellanox Technologies Ltd

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sf_dev {
    pub adev: auxiliary_device,
    pub parent_mdev: *mut mlx5_core_dev,
    pub mdev: *mut mlx5_core_dev,
    pub bar_base_addr: phys_addr_t,
    pub sfnum: u32,
    pub fn_id: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_sf_peer_devlink_event_ctx {
    pub fn_id: u16,
    pub devlink: *mut devlink,
    pub err: c_int,
}

extern "C" {
    pub fn mlx5_sf_dev_notifier_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_sf_dev_table_create(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_sf_dev_notifier_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_sf_dev_table_destroy(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_sf_driver_register() -> c_int;
}
extern "C" {
    pub fn mlx5_sf_driver_unregister();
}
extern "C" {
    pub fn mlx5_sf_dev_allocated(dev: *const mlx5_core_dev) -> bool;
}

