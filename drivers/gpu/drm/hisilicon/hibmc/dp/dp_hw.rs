//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/hisilicon/hibmc/dp/dp_hw.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hibmc_hpd_status {
    HIBMC_HPD_OUT,
    HIBMC_HPD_IN,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hibmc_dp_cbar_pattern {
    CBAR_COLOR_BAR,
    CBAR_WHITE,
    CBAR_RED,
    CBAR_ORANGE,
    CBAR_YELLOW,
    CBAR_GREEN,
    CBAR_CYAN,
    CBAR_BLUE,
    CBAR_PURPLE,
    CBAR_BLACK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dp_color_raw {
    pub pattern: hibmc_dp_cbar_pattern,
    pub r_value: u32,
    pub g_value: u32,
    pub b_value: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dp_cbar_cfg {
    pub enable: u8,
    pub self_timing: u8,
    pub /: *mut *mut u8 dynamic_rate; / 0:static, 1-255(frame):dynamic,
    pub pattern: hibmc_dp_cbar_pattern,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hibmc_dp {
    pub dp_dev: *mut hibmc_dp_dev,
    pub drm_dev: *mut drm_device,
    pub encoder: drm_encoder,
    pub connector: drm_connector,
    pub mmio: *mut void __iomem,
    pub aux: drm_dp_aux,
    pub cfg: hibmc_dp_cbar_cfg,
    pub irq_status: u32,
    pub phys_status: c_int,
}

extern "C" {
    pub fn hibmc_dp_hw_init(dp: *mut hibmc_dp) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_mode_set(dp: *mut hibmc_dp, mode: *mut drm_display_mode) -> c_int;
}
extern "C" {
    pub fn hibmc_dp_display_en(dp: *mut hibmc_dp, enable: bool);
}
extern "C" {
    pub fn hibmc_dp_set_cbar(dp: *mut hibmc_dp, cfg: *const hibmc_dp_cbar_cfg);
}
extern "C" {
    pub fn hibmc_dp_reset_link(dp: *mut hibmc_dp);
}
extern "C" {
    pub fn hibmc_dp_hpd_cfg(dp: *mut hibmc_dp);
}
extern "C" {
    pub fn hibmc_dp_enable_int(dp: *mut hibmc_dp);
}
extern "C" {
    pub fn hibmc_dp_disable_int(dp: *mut hibmc_dp);
}
extern "C" {
    pub fn hibmc_dp_check_hpd_status(dp: *mut hibmc_dp, exp_status: c_int) -> bool;
}
extern "C" {
    pub fn hibmc_dp_get_link_rate(dp: *mut hibmc_dp) -> u8;
}
extern "C" {
    pub fn hibmc_dp_get_lanes(dp: *mut hibmc_dp) -> u8;
}
