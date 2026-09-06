//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/infiniband/hw/mlx5/srq.h
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
// Copyright (c) 2013-2018, Mellanox Technologies. All rights reserved.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_srq_attr {
    pub type: u32,
    pub flags: u32,
    pub log_size: u32,
    pub wqe_shift: u32,
    pub log_page_size: u32,
    pub wqe_cnt: u32,
    pub srqn: u32,
    pub xrcd: u32,
    pub page_offset: u32,
    pub cqn: u32,
    pub pd: u32,
    pub lwm: u32,
    pub user_index: u32,
    pub db_record: u64,
    pub pas: *mut __be64,
    pub umem: *mut ib_umem,
    pub tm_log_list_size: u32,
    pub tm_next_tag: u32,
    pub tm_hw_phase_cnt: u32,
    pub tm_sw_phase_cnt: u32,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_core_srq {
    pub /: *mut *mut mlx5_core_rsc_common common; / must be first,
    pub srqn: u32,
    pub max: c_int,
    pub max_gs: usize,
    pub max_avail_gather: usize,
    pub wqe_shift: c_int,
    pub e): *mut *mut *mut void (event)(struct mlx5_core_srq srq, enum mlx5_event,
    pub uid: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_srq_table {
    pub nb: notifier_block,
    pub array: xarray,
}

extern "C" {
    pub fn mlx5_cmd_destroy_srq(dev: *mut mlx5_ib_dev, srq: *mut mlx5_core_srq) -> c_int;
}
extern "C" {
    pub fn mlx5_init_srq_table(dev: *mut mlx5_ib_dev) -> c_int;
}
extern "C" {
    pub fn mlx5_cleanup_srq_table(dev: *mut mlx5_ib_dev);
}
