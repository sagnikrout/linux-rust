//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/soc_and_ip_translator/dcn401/dcn401_soc_and_ip_translator.h
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
// Copyright 2025 Advanced Micro Devices, Inc.

extern "C" {
    pub fn dcn401_construct_soc_and_ip_translator(soc_and_ip_translator: *mut soc_and_ip_translator);
}
// Functions that can be re-used by higher DCN revisions of this component
extern "C" {
    pub fn dcn401_get_soc_bb(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options);
}
extern "C" {
    pub fn dcn401_update_soc_bb_with_values_from_clk_mgr(soc_bb: *mut dml2_soc_bb, dc: *const dc, config: *const dml2_configuration_options);
}
extern "C" {
    pub fn dcn401_update_soc_bb_with_values_from_vbios(soc_bb: *mut dml2_soc_bb, dc: *const dc);
}
extern "C" {
    pub fn dcn401_update_soc_bb_with_values_from_software_policy(soc_bb: *mut dml2_soc_bb, dc: *const dc);
}
