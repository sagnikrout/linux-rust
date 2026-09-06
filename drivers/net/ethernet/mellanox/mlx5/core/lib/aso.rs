//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/lib/aso.h
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES. All rights reserved.

pub const MLX5_WQE_CTRL_WQE_OPC_MOD_SHIFT: c_int = 24;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_aso_ctrl_seg {
    pub va_h: __be32,
    pub /: *mut *mut __be32 va_l; / include read_enable,
    pub l_key: __be32,
    pub data_mask_mode: u8,
    pub condition_1_0_operand: u8,
    pub condition_1_0_offset: u8,
    pub data_offset_condition_operand: u8,
    pub condition_0_data: __be32,
    pub condition_0_mask: __be32,
    pub condition_1_data: __be32,
    pub condition_1_mask: __be32,
    pub bitwise_data: __be64,
    pub data_mask: __be64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_wqe_aso_data_seg {
    pub bytewise_data: [__be32; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_aso_wqe {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub aso_ctrl: mlx5_wqe_aso_ctrl_seg,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_aso_wqe_data {
    pub ctrl: mlx5_wqe_ctrl_seg,
    pub aso_ctrl: mlx5_wqe_aso_ctrl_seg,
    pub aso_data: mlx5_wqe_aso_data_seg,
}

extern "C" {
    pub fn mlx5_aso_poll_cq(aso: *mut mlx5_aso, with_data: bool) -> c_int;
}
extern "C" {
    pub fn mlx5_aso_destroy(aso: *mut mlx5_aso);
}
