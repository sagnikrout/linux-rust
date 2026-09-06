//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/s5p_mfc_opr_v6.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// drivers/media/platform/samsung/s5p-mfc/s5p_mfc_opr_v6.h
//
// Header file for Samsung MFC (Multi Function Codec - FIMV) driver
// Contains declarations of hw related functions.
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// Definition

pub const ENC_MULTI_SLICE_BIT_MIN: c_int = 2800;

pub const ENC_H264_LOOP_FILTER_AB_MAX: c_int = 12;

pub const ENC_H264_PROFILE_MAX: c_int = 3;
pub const ENC_H264_LEVEL_MAX: c_int = 42;

pub const FRAME_DELTA_H264_H263: c_int = 1;
pub const LOOSE_CBR_MAX: c_int = 5;
pub const TIGHT_CBR_MAX: c_int = 10;

pub const ENC_HEVC_QP_INDEX_MAX: c_int = 12;

pub const ENC_HEVC_LOOP_FILTER_MAX: c_int = 12;
pub const ENC_HEVC_LEVEL_MAX: c_int = 62;
pub const FRAME_DELTA_DEFAULT: c_int = 1;
