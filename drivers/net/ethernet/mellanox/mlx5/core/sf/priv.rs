//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/sf/priv.h
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

extern "C" {
    pub fn mlx5_cmd_alloc_sf(dev: *mut mlx5_core_dev, function_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_dealloc_sf(dev: *mut mlx5_core_dev, function_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_sf_enable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_cmd_sf_disable_hca(dev: *mut mlx5_core_dev, func_id: u16) -> c_int;
}
extern "C" {
    pub fn mlx5_sf_sw_to_hw_id(dev: *mut mlx5_core_dev, controller: u32, sw_id: u16) -> u16;
}
extern "C" {
    pub fn mlx5_sf_hw_table_sf_alloc(dev: *mut mlx5_core_dev, controller: u32, usr_sfnum: u32) -> c_int;
}
extern "C" {
    pub fn mlx5_sf_hw_table_sf_free(dev: *mut mlx5_core_dev, controller: u32, id: u16);
}
extern "C" {
    pub fn mlx5_sf_hw_table_sf_deferred_free(dev: *mut mlx5_core_dev, controller: u32, id: u16);
}
extern "C" {
    pub fn mlx5_sf_hw_table_supported(dev: *const mlx5_core_dev) -> bool;
}
