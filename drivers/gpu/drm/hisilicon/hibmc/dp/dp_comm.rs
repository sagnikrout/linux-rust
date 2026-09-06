//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_comm.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Hisilicon Limited.

pub const HIBMC_DP_LANE_NUM_MAX: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_link_status {
    pub clock_recovered: bool,
    pub channel_equalized: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_link_cap {
    pub link_rate: u8,
    pub lanes: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dp_link {
    pub status: hibmc_link_status,
    pub train_set: [u8; HIBMC_DP_LANE_NUM_MAX],
    pub cap: hibmc_link_cap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dp_dev {
    pub aux: *mut drm_dp_aux,
    pub dev: *mut drm_device,
    pub base: *mut void __iomem,
    pub /: *mut *mut mutex lock; / protects concurrent RW in hibmc_dp_reg_write_field(),
    pub link: hibmc_dp_link,
    pub dpcd: [u8; DP_RECEIVER_CAP_SIZE],
    pub downstream_ports: [u8; DP_MAX_DOWNSTREAM_PORTS],
    pub desc: drm_dp_desc,
    pub is_branch: bool,
    pub hpd_status: c_int,
    pub serdes_base: *mut void __iomem,
}

extern "C" {
    pub fn hibmc_dp_aux_init(dp: *mut hibmc_dp);
}
extern "C" {
    pub fn hibmc_dp_link_training(dp: *mut hibmc_dp_dev) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_serdes_init(dp: *mut hibmc_dp_dev) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_serdes_rate_switch(rate: u8, dp: *mut hibmc_dp_dev) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_serdes_set_tx_cfg(dp: *mut hibmc_dp_dev, train_set[HIBMC_DP_LANE_NUM_MAX]: u8) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_update_caps(dp: *mut hibmc_dp_dev);
}
