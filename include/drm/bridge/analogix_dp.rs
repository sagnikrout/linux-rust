//! Automatically rewritten from C Header to Rust Module
//! Source: include/drm/bridge/analogix_dp.h
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
// Analogix DP (Display Port) Core interface driver.
//
// Copyright (C) 2015 Rockchip Electronics Co., Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum analogix_dp_devtype {
    EXYNOS_DP,
    RK3288_DP,
    RK3399_EDP,
    RK3576_EDP,
    RK3588_EDP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct analogix_dp_plat_data {
    pub dev_type: analogix_dp_devtype,
    pub panel: *mut drm_panel,
    pub next_bridge: *mut drm_bridge,
    pub encoder: *mut drm_encoder,
    pub connector: *mut drm_connector,
    pub ops: *const component_ops,
    pub ): *mut *mut int (power_on)(struct analogix_dp_plat_data,
    pub ): *mut *mut int (power_off)(struct analogix_dp_plat_data,
}

extern "C" {
    pub fn analogix_dp_resume(dp: *mut analogix_dp_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_suspend(dp: *mut analogix_dp_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_bind(dp: *mut analogix_dp_device, drm_dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn analogix_dp_unbind(dp: *mut analogix_dp_device);
}
extern "C" {
    pub fn analogix_dp_start_crc(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn analogix_dp_stop_crc(connector: *mut drm_connector) -> c_int;
}
extern "C" {
    pub fn analogix_dp_finish_probe(dp: *mut analogix_dp_device) -> c_int;
}
