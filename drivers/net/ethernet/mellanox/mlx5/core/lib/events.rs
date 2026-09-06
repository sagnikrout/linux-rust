//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/events.h
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
// Copyright (c) 2023, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const PORT_MODULE_EVENT_MODULE_STATUS_MASK: c_uint = 0xF;
pub const PORT_MODULE_EVENT_ERROR_TYPE_MASK: c_uint = 0xF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_module_event_status_type {
    MLX5_MODULE_STATUS_PLUGGED   = 0x1,
    MLX5_MODULE_STATUS_UNPLUGGED = 0x2,
    MLX5_MODULE_STATUS_ERROR     = 0x3,
    MLX5_MODULE_STATUS_DISABLED  = 0x4,
    MLX5_MODULE_STATUS_NUM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum port_module_event_error_type {
    MLX5_MODULE_EVENT_ERROR_POWER_BUDGET_EXCEEDED    = 0x0,
    MLX5_MODULE_EVENT_ERROR_LONG_RANGE_FOR_NON_MLNX  = 0x1,
    MLX5_MODULE_EVENT_ERROR_BUS_STUCK                = 0x2,
    MLX5_MODULE_EVENT_ERROR_NO_EEPROM_RETRY_TIMEOUT  = 0x3,
    MLX5_MODULE_EVENT_ERROR_ENFORCE_PART_NUMBER_LIST = 0x4,
    MLX5_MODULE_EVENT_ERROR_UNKNOWN_IDENTIFIER       = 0x5,
    MLX5_MODULE_EVENT_ERROR_HIGH_TEMPERATURE         = 0x6,
    MLX5_MODULE_EVENT_ERROR_BAD_CABLE                = 0x7,
    MLX5_MODULE_EVENT_ERROR_PCIE_POWER_SLOT_EXCEEDED = 0xc,
    MLX5_MODULE_EVENT_ERROR_NUM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_pme_stats {
    pub status_counters: [u64; MLX5_MODULE_STATUS_NUM],
    pub error_counters: [u64; MLX5_MODULE_EVENT_ERROR_NUM],
}

extern "C" {
    pub fn mlx5_get_pme_stats(dev: *mut mlx5_core_dev, stats: *mut mlx5_pme_stats);
}
extern "C" {
    pub fn mlx5_notifier_call_chain(events: *mut mlx5_events, event: c_uint, data: *mut c_void) -> c_int;
}
