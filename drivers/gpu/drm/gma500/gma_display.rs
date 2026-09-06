//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/gma500/gma_display.h
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
// Copyright © 2006-2011 Intel Corporation
//
// Authors:
// Eric Anholt <eric@anholt.net>
// Patrik Jakobsson <patrik.r.jakobsson@gmail.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_clock_t {
// given values
    pub n: c_int,
    pub m2: int m1,,
    pub p2: int p1,,
// derived values
    pub dot: c_int,
    pub vco: c_int,
    pub m: c_int,
    pub p: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_range_t {
    pub max: int min,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_p2_t {
    pub dot_limit: c_int,
    pub p2_fast: int p2_slow,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_limit_t {
    pub p1: gma_range_t dot, vco, n, m, m1, m2, p,,
    pub p2: gma_p2_t,
    pub best_clock): *mut gma_clock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gma_clock_funcs {
    pub clock): *mut *mut void (clock)(int refclk, struct gma_clock_t,
    pub refclk): *const *const *const *const gma_limit_t (limit)(drm_crtc crtc, int,
    pub clock): *mut gma_clock_t,
}

// Common pipe related functions
extern "C" {
    pub fn gma_pipe_has_type(crtc: *mut drm_crtc, type: c_int) -> bool;
}
extern "C" {
    pub fn gma_wait_for_vblank(dev: *mut drm_device);
}
extern "C" {
    pub fn gma_crtc_load_lut(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_dpms(crtc: *mut drm_crtc, mode: c_int);
}
extern "C" {
    pub fn gma_crtc_prepare(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_commit(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_disable(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_destroy(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_save(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_crtc_restore(crtc: *mut drm_crtc);
}
extern "C" {
    pub fn gma_encoder_prepare(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn gma_encoder_commit(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn gma_encoder_destroy(encoder: *mut drm_encoder);
}
// Common clock related functions
