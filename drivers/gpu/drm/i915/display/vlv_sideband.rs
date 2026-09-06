//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/i915/display/vlv_sideband.h
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
// Copyright © 2025 Intel Corporation

extern "C" {
    pub fn vlv_bunit_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_bunit_read(display: *mut intel_display, reg: u32) -> u32;
}
extern "C" {
    pub fn vlv_bunit_write(display: *mut intel_display, reg: u32, val: u32);
}
extern "C" {
    pub fn vlv_bunit_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_cck_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_cck_read(display: *mut intel_display, reg: u32) -> u32;
}
extern "C" {
    pub fn vlv_cck_write(display: *mut intel_display, reg: u32, val: u32);
}
extern "C" {
    pub fn vlv_cck_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_ccu_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_ccu_read(display: *mut intel_display, reg: u32) -> u32;
}
extern "C" {
    pub fn vlv_ccu_write(display: *mut intel_display, reg: u32, val: u32);
}
extern "C" {
    pub fn vlv_ccu_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_dpio_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_dpio_read(display: *mut intel_display, phy: dpio_phy, reg: c_int) -> u32;
}
extern "C" {
    pub fn vlv_dpio_write(display: *mut intel_display, phy: dpio_phy, reg: c_int, val: u32);
}
extern "C" {
    pub fn vlv_dpio_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_flisdsi_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_flisdsi_read(display: *mut intel_display, reg: u32) -> u32;
}
extern "C" {
    pub fn vlv_flisdsi_write(display: *mut intel_display, reg: u32, val: u32);
}
extern "C" {
    pub fn vlv_flisdsi_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_nc_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_nc_read(display: *mut intel_display, addr: u8) -> u32;
}
extern "C" {
    pub fn vlv_nc_put(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_punit_get(display: *mut intel_display);
}
extern "C" {
    pub fn vlv_punit_read(display: *mut intel_display, addr: u32) -> u32;
}
extern "C" {
    pub fn vlv_punit_write(display: *mut intel_display, addr: u32, val: u32) -> c_int;
}
extern "C" {
    pub fn vlv_punit_put(display: *mut intel_display);
}
