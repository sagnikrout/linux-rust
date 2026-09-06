//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/inno_hdmi.h
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
//
// Copyright (c) 2025 Rockchip Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi_plat_ops {
    pub mode): *mut *mut *mut void (enable)(struct device pdev, struct drm_display_mode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi_phy_config {
    pub pixelclock: c_ulong,
    pub pre_emphasis: u8,
    pub voltage_level_control: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct inno_hdmi_plat_data {
    pub ops: *const inno_hdmi_plat_ops,
    pub phy_configs: *mut inno_hdmi_phy_config,
    pub default_phy_config: *mut inno_hdmi_phy_config,
}
