//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/tc_ct.h
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
// Copyright (c) 2018 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5_ct_attr {
    pub zone: u16,
    pub ct_action: u16,
    pub nf_ft: *mut nf_flowtable,
    pub ct_labels_id: u32,
    pub act_miss_mapping: u32,
    pub act_miss_cookie: u64,
    pub offloaded: bool,
    pub ft: *mut mlx5_ct_ft,
}

// 8 LSB of metadata C5 are reserved for packet color

extern "C" {
    pub fn mlx5_tc_ct_add_no_trk_match(spec: *mut mlx5_flow_spec) -> c_int;
}

extern "C" {
    pub fn mlx5e_tc_ct_is_valid_flow_rule(dev: *const net_device, flow_rule: *mut flow_rule) -> bool;
}

