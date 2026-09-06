//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/umr.h
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
// Copyright (c) 2022, NVIDIA CORPORATION & AFFILIATES.

pub const MLX5_MAX_UMR_SHIFT: c_int = 16;

pub const MLX5_MAX_UMR_EXTENDED_SHIFT: c_int = 43;
pub const MLX5_IB_UMR_OCTOWORD: c_int = 16;
pub const MLX5_IB_UMR_XLT_ALIGNMENT: c_int = 64;
extern "C" {
    pub fn mlx5r_umr_resource_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5r_umr_resource_cleanup(dev: *mut mlx5_ib_dev);
}
extern "C" {
    pub fn mlx5r_umr_init(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5r_umr_cleanup(dev: *mut mlx5_ib_dev);
}
//
// umr_check_mkey_mask() rejects MLX5_MKEY_MASK_PAGE_SIZE which is
// always set if MLX5_IB_SEND_UMR_UPDATE_TRANSLATION (aka
// MLX5_IB_UPD_XLT_ADDR and MLX5_IB_UPD_XLT_ENABLE) is set. Thus, a mkey
// can never be enabled without this capability. Simplify this weird
// quirky hardware by just saying it can't use PAS lists with UMR at
// all.
//
// length is the size of the MR in bytes when mlx5_ib_update_xlt() is
// used.
//
// true if an existing MR can be reconfigured to new access_flags using UMR.
// Older HW cannot use UMR to update certain elements of the MKC. See
// get_umr_update_access_mask() and umr_check_mkey_mask()
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5r_umr_context {
    pub cqe: ib_cqe,
    pub status: ib_wc_status,
    pub done: completion,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5r_umr_wqe {
    pub ctrl_seg: mlx5_wqe_umr_ctrl_seg,
    pub mkey_seg: mlx5_mkey_seg,
    pub data_seg: mlx5_wqe_data_seg,
}

extern "C" {
    pub fn mlx5r_umr_revoke_mr(mr: *mut mlx5_ib_mr) -> c_int;
}
extern "C" {
    pub fn mlx5r_umr_update_data_direct_ksm_pas(mr: *mut mlx5_ib_mr, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn mlx5r_umr_update_mr_pas(mr: *mut mlx5_ib_mr, flags: c_uint, pdn: u32) -> c_int;
}
