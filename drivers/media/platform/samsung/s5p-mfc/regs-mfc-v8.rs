//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc-v8.h
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
// Register definition file for Samsung MFC V8.x Interface (FIMV) driver
//
// Copyright (c) 2014 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// Additional registers for v8
pub const S5P_FIMV_D_MVC_NUM_VIEWS_V8: c_uint = 0xf104;
pub const S5P_FIMV_D_MIN_SCRATCH_BUFFER_SIZE_V8: c_uint = 0xf108;
pub const S5P_FIMV_D_FIRST_PLANE_DPB_SIZE_V8: c_uint = 0xf144;
pub const S5P_FIMV_D_SECOND_PLANE_DPB_SIZE_V8: c_uint = 0xf148;
pub const S5P_FIMV_D_THIRD_PLANE_DPB_SIZE_V8: c_uint = 0xf14C;
pub const S5P_FIMV_D_MV_BUFFER_SIZE_V8: c_uint = 0xf150;
pub const S5P_FIMV_D_FIRST_PLANE_DPB_STRIDE_SIZE_V8: c_uint = 0xf138;
pub const S5P_FIMV_D_SECOND_PLANE_DPB_STRIDE_SIZE_V8: c_uint = 0xf13c;
pub const S5P_FIMV_D_THIRD_PLANE_DPB_STRIDE_SIZE_V8: c_uint = 0xf140;
pub const S5P_FIMV_D_FIRST_PLANE_DPB_V8: c_uint = 0xf160;
pub const S5P_FIMV_D_SECOND_PLANE_DPB_V8: c_uint = 0xf260;
pub const S5P_FIMV_D_THIRD_PLANE_DPB_V8: c_uint = 0xf360;
pub const S5P_FIMV_D_MV_BUFFER_V8: c_uint = 0xf460;
pub const S5P_FIMV_D_NUM_MV_V8: c_uint = 0xf134;
pub const S5P_FIMV_D_INIT_BUFFER_OPTIONS_V8: c_uint = 0xf154;
pub const S5P_FIMV_D_SCRATCH_BUFFER_ADDR_V8: c_uint = 0xf560;
pub const S5P_FIMV_D_SCRATCH_BUFFER_SIZE_V8: c_uint = 0xf564;
pub const S5P_FIMV_D_CPB_BUFFER_ADDR_V8: c_uint = 0xf5b0;
pub const S5P_FIMV_D_CPB_BUFFER_SIZE_V8: c_uint = 0xf5b4;
pub const S5P_FIMV_D_AVAILABLE_DPB_FLAG_LOWER_V8: c_uint = 0xf5bc;
pub const S5P_FIMV_D_CPB_BUFFER_OFFSET_V8: c_uint = 0xf5c0;
pub const S5P_FIMV_D_SLICE_IF_ENABLE_V8: c_uint = 0xf5c4;
pub const S5P_FIMV_D_STREAM_DATA_SIZE_V8: c_uint = 0xf5d0;
// Display information register
pub const S5P_FIMV_D_DISPLAY_FRAME_WIDTH_V8: c_uint = 0xf600;
pub const S5P_FIMV_D_DISPLAY_FRAME_HEIGHT_V8: c_uint = 0xf604;
// Display status
pub const S5P_FIMV_D_DISPLAY_STATUS_V8: c_uint = 0xf608;
pub const S5P_FIMV_D_DISPLAY_FIRST_PLANE_ADDR_V8: c_uint = 0xf60c;
pub const S5P_FIMV_D_DISPLAY_SECOND_PLANE_ADDR_V8: c_uint = 0xf610;
pub const S5P_FIMV_D_DISPLAY_FRAME_TYPE_V8: c_uint = 0xf618;
pub const S5P_FIMV_D_DISPLAY_CROP_INFO1_V8: c_uint = 0xf61c;
pub const S5P_FIMV_D_DISPLAY_CROP_INFO2_V8: c_uint = 0xf620;
pub const S5P_FIMV_D_DISPLAY_PICTURE_PROFILE_V8: c_uint = 0xf624;
// Decoded picture information register
pub const S5P_FIMV_D_DECODED_STATUS_V8: c_uint = 0xf644;
pub const S5P_FIMV_D_DECODED_FIRST_PLANE_ADDR_V8: c_uint = 0xf648;
pub const S5P_FIMV_D_DECODED_SECOND_PLANE_ADDR_V8: c_uint = 0xf64c;
pub const S5P_FIMV_D_DECODED_THIRD_PLANE_ADDR_V8: c_uint = 0xf650;
pub const S5P_FIMV_D_DECODED_FRAME_TYPE_V8: c_uint = 0xf654;
pub const S5P_FIMV_D_DECODED_NAL_SIZE_V8: c_uint = 0xf664;
// Returned value register for specific setting
pub const S5P_FIMV_D_RET_PICTURE_TAG_TOP_V8: c_uint = 0xf674;
pub const S5P_FIMV_D_RET_PICTURE_TAG_BOT_V8: c_uint = 0xf678;
pub const S5P_FIMV_D_MVC_VIEW_ID_V8: c_uint = 0xf6d8;
// SEI related information
pub const S5P_FIMV_D_FRAME_PACK_SEI_AVAIL_V8: c_uint = 0xf6dc;
// Encoder Registers
pub const S5P_FIMV_E_FIXED_PICTURE_QP_V8: c_uint = 0xf794;
pub const S5P_FIMV_E_RC_CONFIG_V8: c_uint = 0xf798;
pub const S5P_FIMV_E_RC_QP_BOUND_V8: c_uint = 0xf79c;
pub const S5P_FIMV_E_RC_RPARAM_V8: c_uint = 0xf7a4;
pub const S5P_FIMV_E_MB_RC_CONFIG_V8: c_uint = 0xf7a8;
pub const S5P_FIMV_E_PADDING_CTRL_V8: c_uint = 0xf7ac;
pub const S5P_FIMV_E_MV_HOR_RANGE_V8: c_uint = 0xf7b4;
pub const S5P_FIMV_E_MV_VER_RANGE_V8: c_uint = 0xf7b8;
pub const S5P_FIMV_E_VBV_BUFFER_SIZE_V8: c_uint = 0xf78c;
pub const S5P_FIMV_E_VBV_INIT_DELAY_V8: c_uint = 0xf790;
pub const S5P_FIMV_E_MIN_SCRATCH_BUFFER_SIZE_V8: c_uint = 0xf894;
pub const S5P_FIMV_E_ASPECT_RATIO_V8: c_uint = 0xfb4c;
pub const S5P_FIMV_E_EXTENDED_SAR_V8: c_uint = 0xfb50;
pub const S5P_FIMV_E_H264_OPTIONS_V8: c_uint = 0xfb54;
// MFCv8 Context buffer sizes

// Buffer size defines

// BUffer alignment defines
pub const S5P_FIMV_D_ALIGN_PLANE_SIZE_V8: c_int = 64;
// MFCv8 variant defines

pub const MFC_VERSION_V8: c_uint = 0x80;
pub const MFC_NUM_PORTS_V8: c_int = 1;
