//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_cx0_phy.h
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
// Copyright © 2023 Intel Corporation
//

extern "C" {
    pub fn intel_encoder_is_c10phy(encoder: *mut intel_encoder) -> bool;
}
extern "C" {
    pub fn intel_readout_lane_count(encoder: *mut intel_encoder, lane0: c_int, lane1: c_int) -> c_int;
}
extern "C" {
    pub fn intel_mtl_pll_disable(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_mtl_pll_disable_clock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_mtl_pll_disable_clock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_mtl_tbt_pll_disable_clock(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_cx0_setup_powerdown(encoder: *mut intel_encoder);
}
extern "C" {
    pub fn intel_cx0_is_hdmi_frl(clock: u32) -> bool;
}
extern "C" {
    pub fn intel_cx0_read(encoder: *mut intel_encoder, lane_mask: u8, addr: u16) -> u8;
}
extern "C" {
    pub fn intel_cx0_bus_reset(encoder: *mut intel_encoder, lane: c_int);
}
extern "C" {
    pub fn intel_mtl_tbt_pll_calc_state(hw_state: *mut intel_dpll_hw_state);
}
extern "C" {
    pub fn intel_mtl_tbt_calc_port_clock(encoder: *mut intel_encoder) -> c_int;
}
extern "C" {
    pub fn intel_cx0pll_verify_plls(display: *mut intel_display);
}
extern "C" {
    pub fn intel_cx0_pll_power_save_wa(display: *mut intel_display);
}
extern "C" {
    pub fn intel_mtl_tbt_pll_disable(encoder: *mut intel_encoder);
}
