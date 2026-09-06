//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/tout.h
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
// Copyright (c) 2021, NVIDIA CORPORATION & AFFILIATES. All rights reserved.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_timeouts_types {
// pre init timeouts (not read from FW)
    MLX5_TO_FW_PRE_INIT_TIMEOUT_MS,
    MLX5_TO_FW_PRE_INIT_ON_RECOVERY_TIMEOUT_MS,
    MLX5_TO_FW_PRE_INIT_WARN_MESSAGE_INTERVAL_MS,
    MLX5_TO_FW_PRE_INIT_WAIT_MS,

// init segment timeouts
    MLX5_TO_FW_INIT_MS,
    MLX5_TO_CMD_MS,

// DTOR timeouts
    MLX5_TO_PCI_TOGGLE_MS,
    MLX5_TO_HEALTH_POLL_INTERVAL_MS,
    MLX5_TO_FULL_CRDUMP_MS,
    MLX5_TO_FW_RESET_MS,
    MLX5_TO_FLUSH_ON_ERROR_MS,
    MLX5_TO_PCI_SYNC_UPDATE_MS,
    MLX5_TO_TEARDOWN_MS,
    MLX5_TO_FSM_REACTIVATE_MS,
    MLX5_TO_RECLAIM_PAGES_MS,
    MLX5_TO_RECLAIM_VFS_PAGES_MS,
    MLX5_TO_RESET_UNLOAD_MS,

    MAX_TIMEOUT_TYPES
}

extern "C" {
    pub fn mlx5_tout_init(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_tout_cleanup(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_tout_query_iseg(dev: *mut mlx5_core_dev);
}
extern "C" {
    pub fn mlx5_tout_query_dtor(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn _mlx5_tout_ms(dev: *mut mlx5_core_dev, type: mlx5_timeouts_types) -> u64;
}

