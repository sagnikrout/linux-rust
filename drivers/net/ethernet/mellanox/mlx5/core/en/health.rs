//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/mellanox/mlx5/core/en/health.h
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


// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Mellanox Technologies.

extern "C" {
    pub fn mlx5e_reporter_tx_create(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_reporter_tx_destroy(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_reporter_tx_err_cqe(sq: *mut mlx5e_txqsq);
}
extern "C" {
    pub fn mlx5e_reporter_tx_timeout(sq: *mut mlx5e_txqsq) -> c_int;
}
extern "C" {
    pub fn mlx5e_reporter_tx_ptpsq_unhealthy(ptpsq: *mut mlx5e_ptpsq);
}
extern "C" {
    pub fn mlx5e_health_cq_diag_fmsg(cq: *mut mlx5e_cq, fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn mlx5e_health_cq_common_diag_fmsg(cq: *mut mlx5e_cq, fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn mlx5e_health_eq_diag_fmsg(eq: *mut mlx5_eq_comp, fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn mlx5e_health_fmsg_named_obj_nest_start(fmsg: *mut devlink_fmsg, name: *mut c_char);
}
extern "C" {
    pub fn mlx5e_health_fmsg_named_obj_nest_end(fmsg: *mut devlink_fmsg);
}
extern "C" {
    pub fn mlx5e_reporter_rx_create(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_reporter_rx_destroy(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_reporter_icosq_cqe_err(icosq: *mut mlx5e_icosq);
}
extern "C" {
    pub fn mlx5e_reporter_rq_cqe_err(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_reporter_rx_timeout(rq: *mut mlx5e_rq);
}
extern "C" {
    pub fn mlx5e_reporter_icosq_suspend_recovery(c: *mut mlx5e_channel);
}
extern "C" {
    pub fn mlx5e_reporter_icosq_resume_recovery(c: *mut mlx5e_channel);
}
pub const MLX5E_REPORTER_PER_Q_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx5e_err_ctx {
    pub ctx): *mut *mut int (recover)(void,
    pub ctx): *mut *mut *mut *mut int (dump)(struct mlx5e_priv priv, struct devlink_fmsg fmsg, void,
    pub ctx: *mut c_void,
}

extern "C" {
    pub fn mlx5e_health_sq_to_ready(mdev: *mut mlx5_core_dev, dev: *mut net_device, sqn: u32) -> c_int;
}
extern "C" {
    pub fn mlx5e_health_recover_channels(priv: *mut mlx5e_priv) -> c_int;
}
extern "C" {
    pub fn mlx5e_health_create_reporters(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_health_destroy_reporters(priv: *mut mlx5e_priv);
}
extern "C" {
    pub fn mlx5e_health_channels_update(priv: *mut mlx5e_priv);
}
