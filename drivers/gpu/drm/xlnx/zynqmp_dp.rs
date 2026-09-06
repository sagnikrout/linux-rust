//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/xlnx/zynqmp_dp.h
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
//
// ZynqMP DisplayPort Driver
//
// Copyright (C) 2017 - 2020 Xilinx, Inc.
//
// Authors:
// - Hyun Woo Kwon <hyun.kwon@xilinx.com>
// - Laurent Pinchart <laurent.pinchart@ideasonboard.com>
//
extern "C" {
    pub fn zynqmp_dp_enable_vblank(dp: *mut zynqmp_dp);
}
extern "C" {
    pub fn zynqmp_dp_disable_vblank(dp: *mut zynqmp_dp);
}
extern "C" {
    pub fn zynqmp_dp_probe(dpsub: *mut zynqmp_dpsub) -> c_int;
}
extern "C" {
    pub fn zynqmp_dp_remove(dpsub: *mut zynqmp_dpsub);
}
extern "C" {
    pub fn zynqmp_dp_audio_enable(dp: *mut zynqmp_dp);
}
extern "C" {
    pub fn zynqmp_dp_audio_disable(dp: *mut zynqmp_dp);
}
extern "C" {
    pub fn zynqmp_dp_audio_write_n_m(dp: *mut zynqmp_dp);
}
