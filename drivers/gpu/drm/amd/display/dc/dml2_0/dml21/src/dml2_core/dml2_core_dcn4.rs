//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/src/dml2_core/dml2_core_dcn4.h
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
    pub fn core_dcn4_initialize(in_out: *mut dml2_core_initialize_in_out) -> bool;
}
extern "C" {
    pub fn core_dcn42_initialize(in_out: *mut dml2_core_initialize_in_out) -> bool;
}
extern "C" {
    pub fn core_dcn4_mode_support(in_out: *mut dml2_core_mode_support_in_out) -> bool;
}
extern "C" {
    pub fn core_dcn4_mode_programming(in_out: *mut dml2_core_mode_programming_in_out) -> bool;
}
extern "C" {
    pub fn core_dcn4_populate_informative(in_out: *mut dml2_core_populate_informative_in_out) -> bool;
}
extern "C" {
    pub fn core_dcn4_calculate_mcache_allocation(in_out: *mut dml2_calculate_mcache_allocation_in_out) -> bool;
}
