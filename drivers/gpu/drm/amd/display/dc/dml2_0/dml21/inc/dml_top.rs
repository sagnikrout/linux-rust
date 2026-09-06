//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/inc/dml_top.h
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

//
// Top Level Interface for DML2
//
// Returns the size of the DML instance for the caller to allocate
//
extern "C" {
    pub fn dml2_get_instance_size_bytes() -> c_uint;
}
//
// Initializes the DML instance (i.e. with configuration, soc BB, IP params, etc...)
//
extern "C" {
    pub fn dml2_initialize_instance(in_out: *mut dml2_initialize_instance_in_out) -> bool;
}
//
// Determines if the input mode is supported (boolean) on the SoC at all.  Does not return
// information on how mode should be programmed.
//
extern "C" {
    pub fn dml2_check_mode_supported(in_out: *mut dml2_check_mode_supported_in_out) -> bool;
}
//
// Determines the full (optimized) programming for the input mode.  Returns minimum
// clocks as well as dchub register programming values for all pipes, additional meta
// such as ODM or MPCC combine factors.
//
extern "C" {
    pub fn dml2_build_mode_programming(in_out: *mut dml2_build_mode_programming_in_out) -> bool;
}
//
// Determines the correct per pipe mcache register programming for a valid mode.
// The mcache allocation must have been calculated (successfully) in a previous
// call to dml2_build_mode_programming.
// The actual hubp viewport dimensions be what the actual registers will be
// programmed to (i.e. based on scaler setup).
//
extern "C" {
    pub fn dml2_build_mcache_programming(in_out: *mut dml2_build_mcache_programming_in_out) -> bool;
}
