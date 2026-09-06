//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/msm/disp/mdp4/mdp4_kms.h
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
// Copyright (C) 2013 Red Hat
// Author: Rob Clark <robdclark@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mdp4_kms {
    pub base: mdp_kms,
    pub dev: *mut drm_device,
    pub rev: c_int,
    pub mmio: *mut void __iomem,
    pub vdd: *mut regulator,
    pub clk: *mut clk,
    pub pclk: *mut clk,
    pub lut_clk: *mut clk,
    pub axi_clk: *mut clk,
    pub error_handler: mdp_irq,
    pub rpm_enabled: bool,
// empty/blank cursor bo to use when cursor is "disabled"
    pub blank_cursor_bo: *mut drm_gem_object,
    pub blank_cursor_iova: u64,
}

extern "C" {
    pub fn readl(reg: mdp4_kms->mmio +) -> return;
}
extern "C" {
    pub fn mdp4_disable(mdp4_kms: *mut mdp4_kms) -> c_int;
}
extern "C" {
    pub fn mdp4_enable(mdp4_kms: *mut mdp4_kms) -> c_int;
}
extern "C" {
    pub fn mdp4_irq_preinstall(kms: *mut msm_kms);
}
extern "C" {
    pub fn mdp4_irq_postinstall(kms: *mut msm_kms) -> c_int;
}
extern "C" {
    pub fn mdp4_irq_uninstall(kms: *mut msm_kms);
}
extern "C" {
    pub fn mdp4_irq(kms: *mut msm_kms) -> irqreturn_t;
}
extern "C" {
    pub fn mdp4_enable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn mdp4_disable_vblank(kms: *mut msm_kms, crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mdp4_plane_pipe(plane: *mut drm_plane) -> mdp4_pipe;
}
extern "C" {
    pub fn mdp4_crtc_vblank(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn mdp4_crtc_set_config(crtc: *mut drm_crtc, config: u32);
}
extern "C" {
    pub fn mdp4_crtc_set_intf(crtc: *mut drm_crtc, intf: mdp4_intf, mixer: c_int);
}
extern "C" {
    pub fn mdp4_crtc_wait_for_commit_done(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn mdp4_dtv_round_pixclk(encoder: *mut drm_encoder, rate: c_ulong) -> c_long;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENODEV) -> return;
}

