//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/imx/ipuv3/ipuv3-plane.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipu_plane {
    pub base: drm_plane,
    pub ipu: *mut ipu_soc,
    pub ipu_ch: *mut ipuv3_channel,
    pub alpha_ch: *mut ipuv3_channel,
    pub dmfc: *mut dmfc_channel,
    pub dp: *mut ipu_dp,
    pub dma: c_int,
    pub dp_flow: c_int,
    pub disabling: bool,
}

// Init IDMAC, DMFC, DP
extern "C" {
    pub fn ipu_plane_irq(plane: *mut ipu_plane) -> c_int;
}
extern "C" {
    pub fn ipu_plane_disable(ipu_plane: *mut ipu_plane, disable_dp_channel: bool);
}
extern "C" {
    pub fn ipu_plane_disable_deferred(plane: *mut drm_plane);
}
extern "C" {
    pub fn ipu_plane_atomic_update_pending(plane: *mut drm_plane) -> bool;
}
