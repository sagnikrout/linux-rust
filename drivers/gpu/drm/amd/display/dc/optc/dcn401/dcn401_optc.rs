//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/optc/dcn401/dcn401_optc.h
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
// Copyright 2024 Advanced Micro Devices, Inc.

// Macro flag: #define OPTC_COMMON_MASK_SH_LIST_DCN401(mask_sh)\
extern "C" {
    pub fn dcn401_timing_generator_init(optc1: *mut optc);
}
extern "C" {
    pub fn optc401_set_vtotal_min_max(optc: *mut timing_generator, vtotal_min: c_int, vtotal_max: c_int);
}
extern "C" {
    pub fn optc401_setup_manual_trigger(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc401_enable_crtc(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc401_disable_crtc(optc: *mut timing_generator) -> bool;
}
extern "C" {
    pub fn optc401_phantom_crtc_post_enable(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc401_disable_phantom_otg(optc: *mut timing_generator);
}
extern "C" {
    pub fn optc401_set_h_timing_div_manual_mode(optc: *mut timing_generator, manual_mode: bool);
}
extern "C" {
    pub fn optc401_set_out_mux(optc: *mut timing_generator, dest: otg_out_mux_dest);
}
extern "C" {
    pub fn optc401_wait_update_lock_status(tg: *mut timing_generator, locked: bool) -> bool;
}
extern "C" {
    pub fn optc401_set_vupdate_keepout(tg: *mut timing_generator, enable: bool);
}
