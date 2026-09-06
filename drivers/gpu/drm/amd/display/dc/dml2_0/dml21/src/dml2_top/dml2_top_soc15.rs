//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_top/dml2_top_soc15.h
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

extern "C" {
    pub fn dml2_top_soc15_initialize_instance(in_out: *mut dml2_initialize_instance_in_out) -> bool;
}
extern "C" {
    pub fn dml2_top_mcache_calc_mcache_count_and_offsets(params: *mut top_mcache_calc_mcache_count_and_offsets_in_out) -> bool;
}
extern "C" {
    pub fn dml2_top_mcache_assign_global_mcache_ids(params: *mut top_mcache_assign_global_mcache_ids_in_out);
}
extern "C" {
    pub fn dml2_top_mcache_validate_admissability(params: *mut top_mcache_validate_admissability_in_out) -> bool;
}
extern "C" {
    pub fn dml2_top_soc15_build_mcache_programming(params: *mut dml2_build_mcache_programming_in_out) -> bool;
}
