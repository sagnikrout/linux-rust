//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_ctrl.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (c) 2012-2020, The Linux Foundation. All rights reserved.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct msm_dp_ctrl {
    pub wide_bus_en: bool,
}

extern "C" {
    pub fn msm_dp_ctrl_on_stream(msm_dp_ctrl: *mut msm_dp_ctrl, panel: *mut msm_dp_panel) -> c_int;
}
extern "C" {
    pub fn msm_dp_ctrl_off_pixel_clk(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_push_idle(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_phy_init(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_phy_exit(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_irq_phy_exit(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_core_clk_enable(msm_dp_ctrl: *mut msm_dp_ctrl) -> c_int;
}
extern "C" {
    pub fn msm_dp_ctrl_core_clk_disable(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_enable_irq(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_disable_irq(msm_dp_ctrl: *mut msm_dp_ctrl);
}
extern "C" {
    pub fn msm_dp_ctrl_reinit_phy(msm_dp_ctrl: *mut msm_dp_ctrl);
}
