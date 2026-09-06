//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_vpu_buffer.h
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
// Copyright (c) 2022-2024 Qualcomm Innovation Center, Inc. All rights reserved.
//
pub const MIN_BUFFERS: c_int = 4;
pub const DMA_ALIGNMENT: c_int = 256;
pub const HFI_ALIGNMENT_4096: c_int = 4096;
pub const NUM_HW_PIC_BUF: c_int = 32;
pub const LCU_MAX_SIZE_PELS: c_int = 64;
pub const LCU_MIN_SIZE_PELS: c_int = 16;

pub const MAX_TILE_COLUMNS: c_int = 32;

pub const H264D_MAX_SLICE: c_int = 1800;
pub const SIZE_H264D_BUFTAB_T: c_int = 256;

pub const SIZE_H264D_VPP_CMD_PER_BUF: c_int = 512;

pub const SIZE_SLIST_BUF_H264: c_int = 512;
pub const H264_DISPLAY_BUF_SIZE: c_int = 3328;
pub const H264_NUM_FRM_INFO: c_int = 66;
pub const H265_NUM_TILE_COL: c_int = 32;
pub const H265_NUM_TILE_ROW: c_int = 128;

pub const SIZE_ONE_SLICE_BUF: c_int = 256;
pub const VP9_NUM_FRAME_INFO_BUF: c_int = 32;

pub const BUFFER_ALIGNMENT_16_BYTES: c_int = 16;
pub const BUFFER_ALIGNMENT_32_BYTES: c_int = 32;
pub const BUFFER_ALIGNMENT_64_BYTES: c_int = 64;
pub const BUFFER_ALIGNMENT_256_BYTES: c_int = 256;
pub const BUFFER_ALIGNMENT_512_BYTES: c_int = 512;

pub const MAX_FE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE: c_int = 64;
pub const MAX_FE_NBR_CTRL_LCU32_LINE_BUFFER_SIZE: c_int = 64;
pub const MAX_FE_NBR_CTRL_LCU16_LINE_BUFFER_SIZE: c_int = 64;

pub const SIZE_SEI_USERDATA: c_int = 4096;

pub const H264_CABAC_HDR_RATIO_HD_TOT: c_int = 1;
pub const H264_CABAC_RES_RATIO_HD_TOT: c_int = 3;
pub const H265D_MAX_SLICE: c_int = 3600;

pub const H265_CABAC_HDR_RATIO_HD_TOT: c_int = 2;
pub const H265_CABAC_RES_RATIO_HD_TOT: c_int = 2;

pub const SIZE_THREE_DIMENSION_USERDATA: c_int = 768;
pub const SIZE_H265D_ARP: c_int = 9728;

pub const VPX_DECODER_FRAME_BIN_HDR_BUDGET: c_int = 1;
pub const VPX_DECODER_FRAME_BIN_RES_BUDGET: c_int = 3;
pub const VPX_DECODER_FRAME_BIN_DENOMINATOR: c_int = 2;

pub const FE_LFT_CTRL_LINE_NUMBERS: c_int = 4;
pub const FE_LFT_DB_DATA_LINE_NUMBERS: c_int = 2;
pub const FE_LFT_LR_DATA_LINE_NUMBERS: c_int = 4;
pub const FE_TOP_CTRL_LINE_NUMBERS: c_int = 3;
pub const FE_TOP_DATA_LUMA_LINE_NUMBERS: c_int = 2;
pub const FE_TOP_DATA_CHROMA_LINE_NUMBERS: c_int = 3;
pub const FE_SDC_DATA_PER_BLOCK: c_int = 16;
pub const SE_CTRL_DATA_PER_BLOCK: c_int = 2020;
pub const MAX_PE_NBR_DATA_LCU16_LINE_BUFFER_SIZE: c_int = 96;
pub const MAX_PE_NBR_DATA_LCU32_LINE_BUFFER_SIZE: c_int = 192;
pub const MAX_FE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE: c_int = 64;
pub const MAX_SE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE: c_int = 16;
pub const MAX_PE_NBR_DATA_LCU64_LINE_BUFFER_SIZE: c_int = 384;
pub const MAX_FE_NBR_DATA_LUMA_LINE_BUFFER_SIZE: c_int = 640;
pub const AV1_CABAC_HDR_RATIO_HD_TOT: c_int = 2;
pub const AV1_CABAC_RES_RATIO_HD_TOT: c_int = 2;
pub const AV1D_LCU_MAX_SIZE_PELS: c_int = 128;
pub const AV1D_LCU_MIN_SIZE_PELS: c_int = 64;
pub const AV1D_MAX_TILE_COLS: c_int = 64;
pub const MAX_PE_NBR_DATA_LCU32_LINE_BUFFER_SIZE: c_int = 192;
pub const MAX_PE_NBR_DATA_LCU16_LINE_BUFFER_SIZE: c_int = 96;
pub const AV1D_NUM_HW_PIC_BUF: c_int = 16;
pub const AV1D_NUM_FRAME_HEADERS: c_int = 16;
pub const SIZE_AV1D_SEQUENCE_HEADER: c_int = 768;
pub const SIZE_AV1D_METADATA: c_int = 512;
pub const SIZE_AV1D_FRAME_HEADER: c_int = 1280;
pub const SIZE_AV1D_TILE_OFFSET: c_int = 65536;
pub const SIZE_AV1D_QM: c_int = 3328;
pub const SIZE_AV1D_PROB_TABLE: c_int = 22784;

pub const IRIS_METADATA_STRIDE_MULTIPLE: c_int = 64;
pub const IRIS_METADATA_HEIGHT_MULTIPLE: c_int = 16;
pub const HFI_BUFFER_ARP_ENC: c_int = 204800;
pub const LOG2_16: c_int = 4;
pub const LOG2_32: c_int = 5;
pub const LLB_UNIT_SIZE: c_int = 16;
pub const MAX_WIDTH: c_int = 4096;
pub const MAX_HEIGHT: c_int = 2304;

pub const BITS_PER_PIX: c_int = 16;
pub const NUM_LINES_LUMA: c_int = 10;
pub const NUM_LINES_CHROMA: c_int = 6;
pub const AV1D_LCU_MAX_SIZE_PELS: c_int = 128;
pub const AV1D_LCU_MIN_SIZE_PELS: c_int = 64;
pub const AV1D_MAX_TILE_COLS: c_int = 64;
pub const BITS_PER_CTRL_PACK: c_int = 128;
pub const NUM_CTRL_PACK_LCU: c_int = 10;
extern "C" {
    pub fn DIV_ROUND_UP(_arg: frame_width, _arg: 16) -> *mut return MAX_FE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: frame_height, _arg: 16) -> *mut return MAX_FE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: frame_width, _arg: 16) -> *mut return MAX_SE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: frame_height, _arg: 16) -> *mut return MAX_SE_NBR_CTRL_LCU64_LINE_BUFFER_SIZE;
}
extern "C" {
    pub fn DIV_ROUND_UP(_arg: frame_width, _arg: 16) -> *mut return MAX_PE_NBR_DATA_LCU64_LINE_BUFFER_SIZE;
}
extern "C" {
    pub fn size_h264d_qp(_arg: frame_width, _arg: frame_height) -> return;
}
extern "C" {
    pub fn iris_vpu_buf_size(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> u32;
}
extern "C" {
    pub fn iris_vpu33_buf_size(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> u32;
}
extern "C" {
    pub fn iris_vpu4x_buf_size(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> u32;
}
extern "C" {
    pub fn iris_vpu_buf_count(inst: *mut iris_inst, buffer_type: iris_buffer_type) -> c_int;
}
