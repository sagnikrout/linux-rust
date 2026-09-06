//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/steering/hws/debug.h
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
// Copyright (c) 2024 NVIDIA Corporation & Affiliates

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5hws_debug_res_type {
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT = 4000,
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT_ATTR = 4001,
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT_CAPS = 4002,
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT_SEND_ENGINE = 4003,
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT_SEND_RING = 4004,
    MLX5HWS_DEBUG_RES_TYPE_CONTEXT_STC = 4005,

    MLX5HWS_DEBUG_RES_TYPE_TABLE = 4100,

    MLX5HWS_DEBUG_RES_TYPE_MATCHER = 4200,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_ATTR = 4201,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_MATCH_TEMPLATE = 4202,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_TEMPLATE_MATCH_DEFINER = 4203,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_ACTION_TEMPLATE = 4204,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_TEMPLATE_HASH_DEFINER = 4205,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_TEMPLATE_RANGE_DEFINER = 4206,
    MLX5HWS_DEBUG_RES_TYPE_MATCHER_TEMPLATE_COMPARE_MATCH_DEFINER = 4207,

    MLX5HWS_DEBUG_RES_TYPE_ACTION_STE_TABLE = 4300,
}

extern "C" {
    pub fn mlx5hws_debug_init_dump(ctx: *mut mlx5hws_context);
}
extern "C" {
    pub fn mlx5hws_debug_uninit_dump(ctx: *mut mlx5hws_context);
}
