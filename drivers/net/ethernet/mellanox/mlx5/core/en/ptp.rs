//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/ptp.h
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

pub const MLX5E_PTP_CHANNEL_IX: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptp_metadata_fifo {
    pub cc: u8,
    pub pc: u8,
    pub mask: u8,
    pub data: *mut u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptp_metadata_map {
    pub undelivered_counter: u16,
    pub capacity: u16,
    pub data: *mut sk_buff,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptpsq {
    pub txqsq: mlx5e_txqsq,
    pub ts_cq: mlx5e_cq,
    pub cq_stats: *mut mlx5e_ptp_cq_stats,
    pub ts_cqe_ctr_mask: u16,
    pub report_unhealthy_work: work_struct,
    pub ts_cqe_pending_list: *mut mlx5e_ptp_port_ts_cqe_list,
    pub metadata_freelist: mlx5e_ptp_metadata_fifo,
    pub metadata_map: mlx5e_ptp_metadata_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_ptp {
// data path
    pub ptpsq: [mlx5e_ptpsq; MLX5_MAX_NUM_TC],
    pub rq: mlx5e_rq,
    pub napi: napi_struct,
    pub pdev: *mut device,
    pub netdev: *mut net_device,
    pub mkey_be: __be32,
    pub num_tc: u8,
    pub lag_port: u8,
// data path - accessed per napi poll
    pub stats: *mut mlx5e_ch_stats,
// control
    pub priv: *mut mlx5e_priv,
    pub mdev: *mut mlx5_core_dev,
    pub MLX5E_PTP_STATE_NUM_STATES): DECLARE_BITMAP(state,,
    pub bfreg: *mut mlx5_sq_bfreg,
}

extern "C" {
    pub fn mlx5e_ptp_close(c: *mut mlx5e_ptp);
}
extern "C" {
    pub fn mlx5e_ptp_activate_channel(c: *mut mlx5e_ptp);
}
extern "C" {
    pub fn mlx5e_ptp_deactivate_channel(c: *mut mlx5e_ptp);
}
extern "C" {
    pub fn mlx5e_ptp_get_rqn(c: *mut mlx5e_ptp, rqn: *mut u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_ptp_rx_manage_fs(priv: *mut mlx5e_priv, set: bool) -> c_int;
}
extern "C" {
    pub fn mlx5e_ptpsq_track_metadata(ptpsq: *mut mlx5e_ptpsq, metadata: u8);
}
extern "C" {
    pub fn mlx5e_skb_cb_hwtstamp_init(skb: *mut sk_buff);
}
