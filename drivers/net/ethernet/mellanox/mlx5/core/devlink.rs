//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/devlink.h
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
// Copyright (c) 2019, Mellanox Technologies

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_devlink_resource_id {
    MLX5_DL_RES_MAX_LOCAL_SFS = 1,
    MLX5_DL_RES_MAX_EXTERNAL_SFS,

    __MLX5_ID_RES_MAX,
    MLX5_ID_RES_MAX = __MLX5_ID_RES_MAX - 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_devlink_port_resource_id {
    MLX5_DL_PORT_RES_MAX_SFS = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_devlink_param_id {
    MLX5_DEVLINK_PARAM_ID_BASE = DEVLINK_PARAM_GENERIC_ID_MAX,
    MLX5_DEVLINK_PARAM_ID_FLOW_STEERING_MODE,
    MLX5_DEVLINK_PARAM_ID_ESW_LARGE_GROUP_NUM,
    MLX5_DEVLINK_PARAM_ID_ESW_PORT_METADATA,
    MLX5_DEVLINK_PARAM_ID_ESW_MULTIPORT,
    MLX5_DEVLINK_PARAM_ID_HAIRPIN_NUM_QUEUES,
    MLX5_DEVLINK_PARAM_ID_HAIRPIN_QUEUE_SIZE,
    MLX5_DEVLINK_PARAM_ID_PCIE_CONG_IN_LOW,
    MLX5_DEVLINK_PARAM_ID_PCIE_CONG_IN_HIGH,
    MLX5_DEVLINK_PARAM_ID_PCIE_CONG_OUT_LOW,
    MLX5_DEVLINK_PARAM_ID_PCIE_CONG_OUT_HIGH,
    MLX5_DEVLINK_PARAM_ID_CQE_COMPRESSION_TYPE,
    MLX5_DEVLINK_PARAM_ID_SWP_L4_CSUM_MODE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_trap_ctx {
    pub id: c_int,
    pub action: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_devlink_trap {
    pub trap: mlx5_trap_ctx,
    pub item: *mut c_void,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_devlink_trap_event_ctx {
    pub trap: *mut mlx5_trap_ctx,
    pub err: c_int,
}

extern "C" {
    pub fn mlx5_devlink_trap_get_num_active(dev: *mut mlx5_core_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_devlink_traps_register(devlink: *mut devlink) -> c_int;
}
extern "C" {
    pub fn mlx5_devlink_traps_unregister(devlink: *mut devlink);
}
extern "C" {
    pub fn mlx5_devlink_free(devlink: *mut devlink);
}
extern "C" {
    pub fn mlx5_devlink_params_register(devlink: *mut devlink) -> c_int;
}
extern "C" {
    pub fn mlx5_devlink_params_unregister(devlink: *mut devlink);
}
