//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/sti/sti_awg_utils.h
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


// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) STMicroelectronics SA 2014
// Author: Vincent Abriou <vincent.abriou@st.com> for STMicroelectronics.
//

pub const AWG_MAX_INST: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct awg_code_generation_params {
    pub ram_code: *mut u32,
    pub instruction_offset: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct awg_timing {
    pub total_lines: u32,
    pub active_lines: u32,
    pub blanking_lines: u32,
    pub trailing_lines: u32,
    pub total_pixels: u32,
    pub active_pixels: u32,
    pub blanking_pixels: u32,
    pub trailing_pixels: u32,
    pub blanking_level: u32,
}
