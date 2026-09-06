//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mlx5/rsc_dump.h
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
// Copyright (c) 2020 Mellanox Technologies inc.

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mlx5_sgmt_type {
    MLX5_SGMT_TYPE_HW_CQPC,
    MLX5_SGMT_TYPE_HW_SQPC,
    MLX5_SGMT_TYPE_HW_RQPC,
    MLX5_SGMT_TYPE_FULL_SRQC,
    MLX5_SGMT_TYPE_FULL_CQC,
    MLX5_SGMT_TYPE_FULL_EQC,
    MLX5_SGMT_TYPE_FULL_QPC,
    MLX5_SGMT_TYPE_SND_BUFF,
    MLX5_SGMT_TYPE_RCV_BUFF,
    MLX5_SGMT_TYPE_SRQ_BUFF,
    MLX5_SGMT_TYPE_CQ_BUFF,
    MLX5_SGMT_TYPE_EQ_BUFF,
    MLX5_SGMT_TYPE_SX_SLICE,
    MLX5_SGMT_TYPE_SX_SLICE_ALL,
    MLX5_SGMT_TYPE_RDB,
    MLX5_SGMT_TYPE_RX_SLICE_ALL,
    MLX5_SGMT_TYPE_PRM_QUERY_QP,
    MLX5_SGMT_TYPE_PRM_QUERY_CQ,
    MLX5_SGMT_TYPE_PRM_QUERY_MKEY,
    MLX5_SGMT_TYPE_MENU,
    MLX5_SGMT_TYPE_TERMINATE,

    MLX5_SGMT_TYPE_NUM, /* Keep last */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_rsc_key {
    pub rsc: mlx5_sgmt_type,
    pub index1: c_int,
    pub index2: c_int,
    pub num_of_obj1: c_int,
    pub num_of_obj2: c_int,
    pub size: c_int,
}

extern "C" {
    pub fn mlx5_rsc_dump_cmd_destroy(cmd: *mut mlx5_rsc_dump_cmd);
}
