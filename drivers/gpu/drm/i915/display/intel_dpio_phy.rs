//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_dpio_phy.h
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
// Copyright © 2019 Intel Corporation
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpio_channel {
    DPIO_CH0,
    DPIO_CH1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dpio_phy {
    DPIO_PHY0,
    DPIO_PHY1,
    DPIO_PHY2,
}

extern "C" {
    pub fn bxt_dpio_phy_init(display: *mut intel_display, phy: dpio_phy);
}
extern "C" {
    pub fn bxt_dpio_phy_uninit(display: *mut intel_display, phy: dpio_phy);
}
extern "C" {
    pub fn bxt_dpio_phy_calc_lane_lat_optim_mask(lane_count: u8) -> u8;
}
extern "C" {
    pub fn bxt_dpio_phy_get_lane_lat_optim_mask(encoder: *mut intel_encoder) -> u8;
}
extern "C" {
    pub fn vlv_dig_port_to_channel(dig_port: *mut intel_digital_port) -> dpio_channel;
}
extern "C" {
    pub fn vlv_dig_port_to_phy(dig_port: *mut intel_digital_port) -> dpio_phy;
}
extern "C" {
    pub fn vlv_pipe_to_phy(pipe: pipe) -> dpio_phy;
}
extern "C" {
    pub fn vlv_pipe_to_channel(pipe: pipe) -> dpio_channel;
}
extern "C" {
    pub fn chv_phy_release_cl2_override(encoder: *mut intel_encoder);
}

