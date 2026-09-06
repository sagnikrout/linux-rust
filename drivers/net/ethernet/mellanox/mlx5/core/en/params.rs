//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/params.h
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
// Copyright (c) 2019 Mellanox Technologies.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_xsk_param {
    pub headroom: u16,
    pub chunk_size: u32,
    pub unaligned: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq_opt_param {
    pub xsk: *mut mlx5e_xsk_param,
    pub qcfg: *mut netdev_queue_config,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_cq_param {
    pub cqc: [u32; MLX5_ST_SZ_DW(cqc)],
    pub wq: mlx5_wq_param,
    pub eq_ix: u16,
    pub cq_period_mode: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_rq_param {
    pub cqp: mlx5e_cq_param,
    pub rqc: [u32; MLX5_ST_SZ_DW(rqc)],
    pub wq: mlx5_wq_param,
    pub frags_info: mlx5e_rq_frags_info,
    pub xdp_frag_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_sq_param {
    pub cqp: mlx5e_cq_param,
    pub sqc: [u32; MLX5_ST_SZ_DW(sqc)],
    pub wq: mlx5_wq_param,
    pub is_mpw: bool,
    pub is_tls: bool,
    pub stop_room: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_channel_param {
    pub rq: mlx5e_rq_param,
    pub rq_opt: mlx5e_rq_opt_param,
    pub txq_sq: mlx5e_sq_param,
    pub xdp_sq: mlx5e_sq_param,
    pub icosq: mlx5e_sq_param,
    pub async_icosq: mlx5e_sq_param,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_create_sq_param {
    pub wq_ctrl: *mut mlx5_wq_ctrl,
    pub cqn: u32,
    pub ts_cqe_to_dest_cqn: u32,
    pub tisn: u32,
    pub tis_lst_sz: u8,
    pub min_inline_mode: u8,
    pub uar_page: u32,
}

// Striding RQ dynamic parameters
extern "C" {
    pub fn mlx5e_mpwrq_umr_entry_size(mode: mlx5e_mpwrq_umr_mode) -> u8;
}
// Parameter calculations
extern "C" {
    pub fn slow_pci_heuristic(mdev: *mut mlx5_core_dev) -> bool;
}
extern "C" {
    pub fn mlx5e_mpwrq_validate_regular(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params) -> c_int;
}
extern "C" {
    pub fn mlx5e_build_rq_params(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params);
}
extern "C" {
    pub fn mlx5e_set_rq_type(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params);
}
extern "C" {
    pub fn mlx5e_init_rq_type_params(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params);
}
extern "C" {
    pub fn mlx5e_choose_lro_timeout(mdev: *mut mlx5_core_dev, wanted_timeout: u32) -> u32;
}
extern "C" {
    pub fn mlx5e_mpwqe_get_min_wqe_bulk(wq_sz: c_uint) -> u8;
}
extern "C" {
    pub fn mlx5e_mpwrq_max_page_size(mdev: *mut mlx5_core_dev) -> u32;
}
// Build queue parameters
extern "C" {
    pub fn mlx5e_build_create_cq_param(ccp: *mut mlx5e_create_cq_param, c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_calc_sq_stop_room(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params) -> u16;
}
extern "C" {
    pub fn mlx5e_validate_params(mdev: *mut mlx5_core_dev, params: *mut mlx5e_params) -> c_int;
}
