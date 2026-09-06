//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/dp.h
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


// SPDX-License-Identifier: MIT
//
// Copyright (C) 2013-2019 NVIDIA Corporation.
// Copyright (C) 2015 Rob Clark
//
pub const DRM_TEGRA_DP_H: c_int = 1;

//
// struct drm_dp_link_caps - DP link capabilities
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_caps {
//
// @enhanced_framing:
//
// enhanced framing capability (mandatory as of DP 1.2)
//
    pub enhanced_framing: bool,
//
// @tps3_supported:
//
// training pattern sequence 3 supported for equalization
//
    pub tps3_supported: bool,
//
// @fast_training:
//
// AUX CH handshake not required for link training
//
    pub fast_training: bool,
//
// @channel_coding:
//
// ANSI 8B/10B channel coding capability
//
    pub channel_coding: bool,
//
// @alternate_scrambler_reset:
//
// eDP alternate scrambler reset capability
//
    pub alternate_scrambler_reset: bool,
}

//
// struct drm_dp_link_ops - DP link operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_ops {
//
// @apply_training: apply the link training
//
    pub link): *mut *mut int (apply_training)(struct drm_dp_link,
//
// @configure: configure the DP link
//
    pub link): *mut *mut int (configure)(struct drm_dp_link,
}

//
// struct drm_dp_link_train_set - link training settings
// @voltage_swing: per-lane voltage swing
// @pre_emphasis: per-lane pre-emphasis
// @post_cursor: per-lane post-cursor
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_train_set {
    pub voltage_swing: [c_uint; 4],
    pub pre_emphasis: [c_uint; 4],
    pub post_cursor: [c_uint; 4],
}

//
// struct drm_dp_link_train - link training state information
// @request: currently requested settings
// @adjust: adjustments requested by sink
// @pattern: currently requested training pattern
// @clock_recovered: flag to track if clock recovery has completed
// @channel_equalized: flag to track if channel equalization has completed
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link_train {
    pub request: drm_dp_link_train_set,
    pub adjust: drm_dp_link_train_set,
    pub pattern: c_uint,
    pub clock_recovered: bool,
    pub channel_equalized: bool,
}

//
// struct drm_dp_link - DP link capabilities and configuration
// @revision: DP specification revision supported on the link
// @max_rate: maximum clock rate supported on the link
// @max_lanes: maximum number of lanes supported on the link
// @caps: capabilities supported on the link (see &drm_dp_link_caps)
// @aux_rd_interval: AUX read interval to use for training (in microseconds)
// @aux_rd_interval.cr: clock recovery read interval
// @aux_rd_interval.ce: channel equalization read interval
// @edp: eDP revision (0x11: eDP 1.1, 0x12: eDP 1.2, ...)
// @rate: currently configured link rate
// @lanes: currently configured number of lanes
// @rates: additional supported link rates in kHz (eDP 1.4)
// @num_rates: number of additional supported link rates (eDP 1.4)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drm_dp_link {
    pub revision: c_uchar,
    pub max_rate: c_uint,
    pub max_lanes: c_uint,
    pub caps: drm_dp_link_caps,
    pub cr: c_uint,
    pub ce: c_uint,
    pub aux_rd_interval: },
    pub edp: c_uchar,
    pub rate: c_uint,
    pub lanes: c_uint,
    pub rates: [c_ulong; DP_MAX_SUPPORTED_RATES],
    pub num_rates: c_uint,
//
// @ops: DP link operations
//
    pub ops: *const drm_dp_link_ops,
//
// @aux: DP AUX channel
//
    pub aux: *mut drm_dp_aux,
//
// @train: DP link training state
//
    pub train: drm_dp_link_train,
}

extern "C" {
    pub fn drm_dp_link_add_rate(link: *mut drm_dp_link, rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn drm_dp_link_remove_rate(link: *mut drm_dp_link, rate: c_ulong) -> c_int;
}
extern "C" {
    pub fn drm_dp_link_update_rates(link: *mut drm_dp_link);
}
extern "C" {
    pub fn drm_dp_link_probe(aux: *mut drm_dp_aux, link: *mut drm_dp_link) -> c_int;
}
extern "C" {
    pub fn drm_dp_link_configure(aux: *mut drm_dp_aux, link: *mut drm_dp_link) -> c_int;
}
extern "C" {
    pub fn drm_dp_link_train_init(train: *mut drm_dp_link_train);
}
extern "C" {
    pub fn drm_dp_link_train(link: *mut drm_dp_link) -> c_int;
}
