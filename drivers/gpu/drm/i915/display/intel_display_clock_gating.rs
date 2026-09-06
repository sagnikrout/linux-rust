//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/intel_display_clock_gating.h
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
// Copyright 2026 Intel Corporation
//
extern "C" {
    pub fn intel_display_skl_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_kbl_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_cfl_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_bxt_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_glk_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_bdw_clock_gating_disable_fbcq(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_bdw_clock_gating_vblank_in_srd(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_bdw_clock_gating_kvm_notif(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_hsw_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_disable_trickle_feed(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_ilk_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_gen6_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_ivb_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_g4x_init_clock_gating(display: *mut intel_display);
}
extern "C" {
    pub fn intel_display_i965gm_init_clock_gating(display: *mut intel_display);
}
