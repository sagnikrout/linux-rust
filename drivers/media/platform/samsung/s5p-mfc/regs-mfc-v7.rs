//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc-v7.h
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
// Register definition file for Samsung MFC V7.x Interface (FIMV) driver
//
// Copyright (c) 2013 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// Additional features of v7
pub const S5P_FIMV_CODEC_VP8_ENC_V7: c_int = 25;
// Additional registers for v7
pub const S5P_FIMV_E_SOURCE_FIRST_ADDR_V7: c_uint = 0xf9e0;
pub const S5P_FIMV_E_SOURCE_SECOND_ADDR_V7: c_uint = 0xf9e4;
pub const S5P_FIMV_E_SOURCE_THIRD_ADDR_V7: c_uint = 0xf9e8;
pub const S5P_FIMV_E_SOURCE_FIRST_STRIDE_V7: c_uint = 0xf9ec;
pub const S5P_FIMV_E_SOURCE_SECOND_STRIDE_V7: c_uint = 0xf9f0;
pub const S5P_FIMV_E_SOURCE_THIRD_STRIDE_V7: c_uint = 0xf9f4;
pub const S5P_FIMV_E_ENCODED_SOURCE_FIRST_ADDR_V7: c_uint = 0xfa70;
pub const S5P_FIMV_E_ENCODED_SOURCE_SECOND_ADDR_V7: c_uint = 0xfa74;
pub const S5P_FIMV_E_ENCODED_SOURCE_THIRD_ADDR_V7: c_uint = 0xfa78;
pub const S5P_FIMV_E_VP8_OPTIONS_V7: c_uint = 0xfdb0;
pub const S5P_FIMV_E_VP8_FILTER_OPTIONS_V7: c_uint = 0xfdb4;
pub const S5P_FIMV_E_VP8_GOLDEN_FRAME_OPTION_V7: c_uint = 0xfdb8;
pub const S5P_FIMV_E_VP8_NUM_T_LAYER_V7: c_uint = 0xfdc4;
// MFCv7 variant defines

pub const MFC_VERSION_V7: c_uint = 0x72;
pub const MFC_NUM_PORTS_V7: c_int = 1;
pub const MFC_LUMA_PAD_BYTES_V7: c_int = 256;
pub const MFC_CHROMA_PAD_BYTES_V7: c_int = 128;
// MFCv7 Context buffer sizes

// Buffer size defines

