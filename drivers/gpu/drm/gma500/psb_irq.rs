//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/psb_irq.h
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
// Copyright (c) 2009-2011, Intel Corporation.
// All Rights Reserved.
//
// Authors:
// Benjamin Defnet <benjamin.r.defnet@intel.com>
// Rajesh Poornachandran <rajesh.poornachandran@intel.com>
//
extern "C" {
    pub fn gma_irq_preinstall(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_irq_postinstall(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_irq_install(dev: *mut drm_device) -> c_int;
}
extern "C" {
    pub fn gma_irq_uninstall(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_crtc_enable_vblank(crtc: *mut drm_crtc) -> c_int;
}
extern "C" {
    pub fn gma_crtc_disable_vblank(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_get_vblank_counter(crtc: *mut drm_crtc) -> u32;
}
extern "C" {
    pub fn gma_enable_pipestat(dev_priv: *mut drm_psb_private, pipe: c_int, mask: u32);
}
extern "C" {
    pub fn gma_disable_pipestat(dev_priv: *mut drm_psb_private, pipe: c_int, mask: u32);
}
