//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc-v12.h
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
// Register definition file for Samsung MFC V12.x Interface (FIMV) driver
//
// Copyright (c) 2020 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// MFCv12 Context buffer sizes

// MFCv12 variant defines

pub const MFC_VERSION_V12: c_uint = 0xC0;
pub const MFC_NUM_PORTS_V12: c_int = 1;
pub const S5P_FIMV_CODEC_VP9_ENC: c_int = 27;
pub const MFC_CHROMA_PAD_BYTES_V12: c_int = 256;
pub const S5P_FIMV_D_ALIGN_PLANE_SIZE_V12: c_int = 256;
// Encoder buffer size for MFCv12

