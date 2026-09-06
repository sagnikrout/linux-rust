//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dml2_0/dml21/dml21_wrapper_fpu.h
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
// Copyright 2026 Advanced Micro Devices, Inc.

//
// dml21_init - Initialize DML21 context
// @in_dc: dc.
// @dml_ctx: DML21 context to initialize.
// @config: dml21 configuration options.
//
// Performs FPU-requiring initialization. Must be called with FPU protection.
//
extern "C" {
    pub fn dml21_init(in_dc: *const dc, dml_ctx: *mut dml2_context, config: *const dml2_configuration_options);
}
//
// dml21_validate - Determines if a display configuration is supported or not.
// @in_dc: dc.
// @context: dc_state to be validated.
// @dml_ctx: dml21 context.
// @validate_mode: DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX
// will not populate context.res_ctx.
//
// Based on fast_validate option internally would call:
//
// -dml21_mode_check_and_programming - for DC_VALIDATE_MODE_AND_PROGRAMMING option
// Calculates if dc_state can be supported on the input display
// configuration. If supported, generates the necessary HW
// programming for the new dc_state.
//
// -dml21_check_mode_support - for DC_VALIDATE_MODE_ONLY and DC_VALIDATE_MODE_AND_STATE_INDEX option
// Calculates if dc_state can be supported for the input display
// config.
//
// Context: Two threads may not invoke this function concurrently unless they reference
// separate dc_states for validation.
// Return: True if mode is supported, false otherwise.
//
// Prepare hubp mcache_regs for hubp mcache ID and split coordinate programming
extern "C" {
    pub fn dml21_prepare_mcache_programming(in_dc: *mut dc, context: *mut dc_state, dml_ctx: *mut dml2_context);
}
