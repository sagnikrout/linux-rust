//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_quirks.h
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
pub enum intel_quirk_id {
    QUIRK_BACKLIGHT_PRESENT,
    QUIRK_INCREASE_DDI_DISABLED_TIME,
    QUIRK_INCREASE_T12_DELAY,
    QUIRK_INVERT_BRIGHTNESS,
    QUIRK_LVDS_SSC_DISABLE,
    QUIRK_NO_PPS_BACKLIGHT_POWER_HOOK,
    QUIRK_FW_SYNC_LEN,
    QUIRK_EDP_LIMIT_RATE_HBR2,
    QUIRK_DISABLE_EDP_PANEL_REPLAY,
    QUIRK_DISABLE_PSR2,
}

extern "C" {
    pub fn intel_init_quirks(display: *mut intel_display);
}
extern "C" {
    pub fn intel_has_quirk(display: *mut intel_display, quirk: intel_quirk_id) -> bool;
}
extern "C" {
    pub fn intel_has_dpcd_quirk(intel_dp: *mut intel_dp, quirk: intel_quirk_id) -> bool;
}
