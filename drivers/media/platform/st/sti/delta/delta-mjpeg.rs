//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/st/sti/delta/delta-mjpeg.h
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
// Copyright (C) STMicroelectronics SA 2013
// Author: Hugues Fruchet <hugues.fruchet@st.com> for STMicroelectronics.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mjpeg_component {
    pub /: *mut *mut unsigned int id;/ 1=Y, 2=Cb, 3=Cr, 4=L, 5=Q,
    pub h_sampling_factor: c_uint,
    pub v_sampling_factor: c_uint,
    pub quant_table_index: c_uint,
}

pub const MJPEG_MAX_COMPONENTS: c_int = 5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mjpeg_header {
    pub length: c_uint,
    pub sample_precision: c_uint,
    pub frame_width: c_uint,
    pub frame_height: c_uint,
    pub nb_of_components: c_uint,
    pub components: [mjpeg_component; MJPEG_MAX_COMPONENTS],
}
