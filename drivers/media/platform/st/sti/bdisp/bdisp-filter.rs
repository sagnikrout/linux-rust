//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/bdisp/bdisp-filter.h
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
// Authors: Fabien Dessenne <fabien.dessenne@st.com> for STMicroelectronics.
//
pub const BDISP_HF_NB: c_int = 64;
pub const BDISP_VF_NB: c_int = 40;
//
// struct bdisp_filter_h_spec - Horizontal filter specification
//
// @min:        min scale factor for this filter (6.10 fixed point)
// @max:        max scale factor for this filter (6.10 fixed point)
// @coef:       filter coefficients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_filter_h_spec {
    pub min: u16,
    pub max: u16,
    pub coef: [u8; BDISP_HF_NB],
}

//
// struct bdisp_filter_v_spec - Vertical filter specification
//
// @min:	min scale factor for this filter (6.10 fixed point)
// @max:	max scale factor for this filter (6.10 fixed point)
// @coef:	filter coefficients
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bdisp_filter_v_spec {
    pub min: u16,
    pub max: u16,
    pub coef: [u8; BDISP_VF_NB],
}

// RGB YUV 601 standard conversion
