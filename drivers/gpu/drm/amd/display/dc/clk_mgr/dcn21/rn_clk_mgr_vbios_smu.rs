//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/clk_mgr/dcn21/rn_clk_mgr_vbios_smu.h
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


//
// Copyright 2018 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//
extern "C" {
    pub fn rn_vbios_smu_get_smu_version(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
extern "C" {
    pub fn rn_vbios_smu_set_dispclk(clk_mgr: *mut clk_mgr_internal, requested_dispclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn rn_vbios_smu_set_hard_min_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn rn_vbios_smu_set_min_deep_sleep_dcfclk(clk_mgr: *mut clk_mgr_internal, requested_min_ds_dcfclk_khz: c_int) -> c_int;
}
extern "C" {
    pub fn rn_vbios_smu_set_phyclk(clk_mgr: *mut clk_mgr_internal, requested_phyclk_khz: c_int);
}
extern "C" {
    pub fn rn_vbios_smu_set_dppclk(clk_mgr: *mut clk_mgr_internal, requested_dpp_khz: c_int) -> c_int;
}
extern "C" {
    pub fn rn_vbios_smu_set_dcn_low_power_state(clk_mgr: *mut clk_mgr_internal, dcn_pwr_state: enum);
}
extern "C" {
    pub fn rn_vbios_smu_enable_48mhz_tmdp_refclk_pwrdwn(clk_mgr: *mut clk_mgr_internal, enable: bool);
}
extern "C" {
    pub fn rn_vbios_smu_enable_pme_wa(clk_mgr: *mut clk_mgr_internal);
}
extern "C" {
    pub fn rn_vbios_smu_is_periodic_retraining_disabled(clk_mgr: *mut clk_mgr_internal) -> c_int;
}
