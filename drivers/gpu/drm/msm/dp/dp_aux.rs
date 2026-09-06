//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/dp/dp_aux.h
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

extern "C" {
    pub fn msm_dp_aux_register(msm_dp_aux: *mut drm_dp_aux) -> c_int;
}
extern "C" {
    pub fn msm_dp_aux_unregister(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_isr(msm_dp_aux: *mut drm_dp_aux, isr: u32) -> irqreturn_t;
}
extern "C" {
    pub fn msm_dp_aux_enable_xfers(msm_dp_aux: *mut drm_dp_aux, enabled: bool);
}
extern "C" {
    pub fn msm_dp_aux_init(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_deinit(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_reconfig(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_hpd_enable(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_hpd_disable(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_hpd_intr_enable(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_hpd_intr_disable(msm_dp_aux: *mut drm_dp_aux);
}
extern "C" {
    pub fn msm_dp_aux_get_hpd_intr_status(msm_dp_aux: *mut drm_dp_aux) -> u32;
}
extern "C" {
    pub fn msm_dp_aux_is_link_connected(msm_dp_aux: *mut drm_dp_aux) -> u32;
}
extern "C" {
    pub fn msm_dp_aux_put(aux: *mut drm_dp_aux);
}
