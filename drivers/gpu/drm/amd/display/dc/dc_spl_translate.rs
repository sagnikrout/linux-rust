//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dc_spl_translate.h
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

// Map SPL input parameters to pipe context
// @pipe_ctx: pipe context
// @spl_in: spl input structure
//
extern "C" {
    pub fn translate_SPL_in_params_from_pipe_ctx(pipe_ctx: *mut pipe_ctx, spl_in: *mut spl_in);
}
// Map SPL output parameters to pipe context
// @pipe_ctx: pipe context
// @spl_out: spl output structure
//
extern "C" {
    pub fn translate_SPL_out_params_to_pipe_ctx(pipe_ctx: *mut pipe_ctx, spl_out: *mut spl_out);
}
