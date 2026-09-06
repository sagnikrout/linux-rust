//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/dcbnl.h
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
// Copyright (c) 2020 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_cee_config {
// bw pct for priority group
    pub pg_bw_pct: [u8; CEE_DCBX_MAX_PGS],
    pub prio_to_pg_map: [u8; CEE_DCBX_MAX_PRIO],
    pub pfc_setting: [bool; CEE_DCBX_MAX_PRIO],
    pub pfc_enable: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_dcbx {
    pub mode: mlx5_dcbx_oper_mode,
    pub /: *mut *mut mlx5e_cee_config cee_cfg; / pending configuration,
    pub dscp_app_cnt: u8,
// The only setting that cannot be read from FW
    pub tc_tsa: [u8; IEEE_8021QAZ_MAX_TCS],
    pub cap: u8,
// Buffer configuration
    pub cable_len: u32,
    pub xoff: u32,
    pub port_buff_cell_sz: u16,
// Upper limit for 100Mbps and 1Gbps in Kbps units
    pub upper_limit_100mbps: u64,
    pub upper_limit_gbps: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_dcbx_dp {
    pub dscp2prio: [u8; MLX5E_MAX_DSCP],
    pub trust_state: u8,
}

extern "C" {
    pub fn mlx5e_dcbnl_build_netdev(netdev: *mut net_device);
}
extern "C" {
    pub fn mlx5e_dcbnl_initialize(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_dcbnl_init_app(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_dcbnl_delete_app(priv: *mut mlx5e_priv);
}

