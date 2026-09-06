//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc-v10.h
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
// Copyright (c) 2017 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//
// Register definition file for Samsung MFC V10.x Interface (FIMV) driver
//

// MFCv10 register definitions
pub const S5P_FIMV_MFC_CLOCK_OFF_V10: c_uint = 0x7120;
pub const S5P_FIMV_MFC_STATE_V10: c_uint = 0x7124;
pub const S5P_FIMV_D_STATIC_BUFFER_ADDR_V10: c_uint = 0xF570;
pub const S5P_FIMV_D_STATIC_BUFFER_SIZE_V10: c_uint = 0xF574;
pub const S5P_FIMV_E_NUM_T_LAYER_V10: c_uint = 0xFBAC;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER0_V10: c_uint = 0xFBB0;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER1_V10: c_uint = 0xFBB4;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER2_V10: c_uint = 0xFBB8;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER3_V10: c_uint = 0xFBBC;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER4_V10: c_uint = 0xFBC0;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER5_V10: c_uint = 0xFBC4;
pub const S5P_FIMV_E_HIERARCHICAL_QP_LAYER6_V10: c_uint = 0xFBC8;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER0_V10: c_uint = 0xFD18;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER1_V10: c_uint = 0xFD1C;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER2_V10: c_uint = 0xFD20;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER3_V10: c_uint = 0xFD24;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER4_V10: c_uint = 0xFD28;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER5_V10: c_uint = 0xFD2C;
pub const S5P_FIMV_E_HIERARCHICAL_BIT_RATE_LAYER6_V10: c_uint = 0xFD30;
pub const S5P_FIMV_E_HEVC_OPTIONS_V10: c_uint = 0xFDD4;
pub const S5P_FIMV_E_HEVC_REFRESH_PERIOD_V10: c_uint = 0xFDD8;
pub const S5P_FIMV_E_HEVC_CHROMA_QP_OFFSET_V10: c_uint = 0xFDDC;
pub const S5P_FIMV_E_HEVC_LF_BETA_OFFSET_DIV2_V10: c_uint = 0xFDE0;
pub const S5P_FIMV_E_HEVC_LF_TC_OFFSET_DIV2_V10: c_uint = 0xFDE4;
pub const S5P_FIMV_E_HEVC_NAL_CONTROL_V10: c_uint = 0xFDE8;
// MFCv10 Context buffer sizes

// MFCv10 variant defines

pub const MFC_VERSION_V10: c_uint = 0xA0;
pub const MFC_NUM_PORTS_V10: c_int = 1;
// MFCv10 codec defines
pub const S5P_FIMV_CODEC_HEVC_DEC: c_int = 17;
pub const S5P_FIMV_CODEC_VP9_DEC: c_int = 18;
pub const S5P_FIMV_CODEC_HEVC_ENC: c_int = 26;
// Decoder buffer size for MFC v10
pub const DEC_VP9_STATIC_BUFFER_SIZE: c_int = 20480;
// Encoder buffer size for MFC v10.0

