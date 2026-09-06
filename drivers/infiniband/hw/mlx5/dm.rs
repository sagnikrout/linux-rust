//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/dm.h
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
// Copyright (c) 2021, Mellanox Technologies inc. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dm {
    pub ibdm: ib_dm,
    pub type: u32,
    pub dev_addr: phys_addr_t,
    pub size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dm_op_entry {
    pub mentry: mlx5_user_mmap_entry,
    pub op_addr: phys_addr_t,
    pub dm: *mut mlx5_ib_dm_memic,
    pub op: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dm_memic {
    pub base: mlx5_ib_dm,
    pub mentry: mlx5_user_mmap_entry,
    pub ops: xarray,
    pub ops_xa_lock: mutex,
    pub ref: kref,
    pub req_length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ib_dm_icm {
    pub base: mlx5_ib_dm,
    pub obj_id: u32,
}

extern "C" {
    pub fn container_of(_arg: ibdm, mlx5_ib_dm: struct, _arg: ibdm) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdm, mlx5_ib_dm_memic: struct, _arg: base.ibdm) -> return;
}
extern "C" {
    pub fn container_of(_arg: ibdm, mlx5_ib_dm_icm: struct, _arg: base.ibdm) -> return;
}
