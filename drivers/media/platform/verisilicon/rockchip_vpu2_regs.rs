//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/verisilicon/rockchip_vpu2_regs.h
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
// Hantro VPU codec driver
//
// Copyright (C) 2018 Rockchip Electronics Co., Ltd.
// Alpha Lin <alpha.lin@rock-chips.com>
//
// Encoder registers.

pub const VEPU_REG_ADDR_VP8_SEG_MAP: c_uint = 0x06c;

pub const VEPU_REG_VP8_CONTROL: c_uint = 0x0a0;

pub const VEPU_REG_VP8_REF_FRAME_VAL: c_uint = 0x0a4;

pub const VEPU_REG_VP8_LOOP_FILTER_REF_DELTA: c_uint = 0x0a8;

pub const VEPU_REG_VP8_LOOP_FILTER_MODE_DELTA: c_uint = 0x0ac;

pub const VEPU_REG_INTRA_AREA_CTRL: c_uint = 0x0b8;

pub const VEPU_REG_CIR_INTRA_CTRL: c_uint = 0x0bc;

pub const VEPU_REG_ADDR_IN_PLANE_0: c_uint = 0x0c0;
pub const VEPU_REG_ADDR_IN_PLANE_1: c_uint = 0x0c4;
pub const VEPU_REG_ADDR_IN_PLANE_2: c_uint = 0x0c8;
pub const VEPU_REG_STR_HDR_REM_MSB: c_uint = 0x0cc;
pub const VEPU_REG_STR_HDR_REM_LSB: c_uint = 0x0d0;
pub const VEPU_REG_STR_BUF_LIMIT: c_uint = 0x0d4;
pub const VEPU_REG_AXI_CTRL: c_uint = 0x0d8;

pub const VEPU_QP_ADJUST_MAD_DELTA_ROI: c_uint = 0x0dc;

pub const VEPU_REG_ADDR_REF_LUMA: c_uint = 0x0e0;
pub const VEPU_REG_ADDR_REF_CHROMA: c_uint = 0x0e4;
pub const VEPU_REG_QP_SUM_DIV2: c_uint = 0x0e8;

pub const VEPU_REG_ENC_CTRL0: c_uint = 0x0ec;

pub const VEPU_REG_ENC_OVER_FILL_STRM_OFFSET: c_uint = 0x0f0;

pub const VEPU_REG_INPUT_LUMA_INFO: c_uint = 0x0f4;

pub const VEPU_REG_RLC_SUM: c_uint = 0x0f8;

pub const VEPU_REG_SPLIT_PENALTY_4X4: c_uint = 0x0f8;

pub const VEPU_REG_ADDR_REC_LUMA: c_uint = 0x0fc;
pub const VEPU_REG_ADDR_REC_CHROMA: c_uint = 0x100;

pub const VEPU_REG_VP8_SEG0_QUANT_AC_Y1: c_uint = 0x104;

pub const VEPU_REG_VP8_SEG0_QUANT_DC_Y2: c_uint = 0x108;

pub const VEPU_REG_VP8_SEG0_QUANT_AC_Y2: c_uint = 0x10c;

pub const VEPU_REG_VP8_SEG0_QUANT_DC_CHR: c_uint = 0x110;

pub const VEPU_REG_VP8_SEG0_QUANT_AC_CHR: c_uint = 0x114;

pub const VEPU_REG_VP8_SEG0_QUANT_DQUT: c_uint = 0x118;

pub const VEPU_REG_VP8_SEG0_QUANT_DQUT_1: c_uint = 0x11c;

pub const VEPU_REG_VP8_BOOL_ENC_VALUE: c_uint = 0x120;
pub const VEPU_REG_CHKPT_DELTA_QP: c_uint = 0x124;

pub const VEPU_REG_VP8_ENC_CTRL2: c_uint = 0x124;

pub const VEPU_REG_ENC_CTRL1: c_uint = 0x128;

pub const VEPU_REG_INTRA_INTER_MODE: c_uint = 0x12c;

pub const VEPU_REG_ENC_CTRL2: c_uint = 0x130;

pub const VEPU_REG_ADDR_OUTPUT_STREAM: c_uint = 0x134;
pub const VEPU_REG_ADDR_OUTPUT_CTRL: c_uint = 0x138;
pub const VEPU_REG_ADDR_NEXT_PIC: c_uint = 0x13c;
pub const VEPU_REG_ADDR_MV_OUT: c_uint = 0x140;
pub const VEPU_REG_ADDR_CABAC_TBL: c_uint = 0x144;
pub const VEPU_REG_ROI1: c_uint = 0x148;

pub const VEPU_REG_ROI2: c_uint = 0x14c;

pub const VEPU_REG_STABLE_MOTION_SUM: c_uint = 0x174;
pub const VEPU_REG_STABILIZATION_OUTPUT: c_uint = 0x178;

pub const VEPU_REG_RGB2YUV_CONVERSION_COEF1: c_uint = 0x17c;

pub const VEPU_REG_RGB2YUV_CONVERSION_COEF2: c_uint = 0x180;

pub const VEPU_REG_RGB2YUV_CONVERSION_COEF3: c_uint = 0x184;

pub const VEPU_REG_RGB_MASK_MSB: c_uint = 0x188;

pub const VEPU_REG_MV_PENALTY: c_uint = 0x18c;

pub const VEPU_REG_QP_VAL: c_uint = 0x190;

pub const VEPU_REG_VP8_SEG0_QUANT_DC_Y1: c_uint = 0x190;

pub const VEPU_REG_MVC_RELATE: c_uint = 0x198;

pub const VEPU_REG_ENCODE_START: c_uint = 0x19c;

pub const VEPU_REG_MB_CTRL: c_uint = 0x1a0;

pub const VEPU_REG_DATA_ENDIAN: c_uint = 0x1a4;

pub const VEPU_REG_ENC_CTRL3: c_uint = 0x1a8;

pub const VEPU_REG_ENC_CTRL4: c_uint = 0x1ac;

pub const VEPU_REG_ADDR_VP8_PROB_CNT: c_uint = 0x1b0;
pub const VEPU_REG_INTERRUPT: c_uint = 0x1b4;

// vpu decoder register
pub const VDPU_REG_DEC_CTRL0: c_uint = 0x0c8 // 50;

pub const VDPU_REG_STREAM_LEN: c_uint = 0x0cc;

pub const VDPU_REG_ERROR_CONCEALMENT: c_uint = 0x0d0;

pub const VDPU_REG_DEC_FORMAT: c_uint = 0x0d4;

pub const VDPU_REG_DATA_ENDIAN: c_uint = 0x0d8;

pub const VDPU_REG_INTERRUPT: c_uint = 0x0dc;

pub const VDPU_REG_AXI_CTRL: c_uint = 0x0e0;

pub const VDPU_REG_EN_FLAGS: c_uint = 0x0e4;

pub const VDPU_REG_SOFT_RESET: c_uint = 0x0e8;
pub const VDPU_REG_PRED_FLT: c_uint = 0x0ec;

pub const VDPU_REG_ADDITIONAL_CHROMA_ADDRESS: c_uint = 0x0f0;
pub const VDPU_REG_ADDR_QTABLE: c_uint = 0x0f4;
pub const VDPU_REG_DIRECT_MV_ADDR: c_uint = 0x0f8;
pub const VDPU_REG_ADDR_DST: c_uint = 0x0fc;
pub const VDPU_REG_ADDR_STR: c_uint = 0x100;
pub const VDPU_REG_REFBUF_RELATED: c_uint = 0x104;

pub const VDPU_REG_INITIAL_REF_PIC_LIST0: c_uint = 0x190;

pub const VDPU_REG_INITIAL_REF_PIC_LIST1: c_uint = 0x194;

pub const VDPU_REG_INITIAL_REF_PIC_LIST2: c_uint = 0x198;

pub const VDPU_REG_INITIAL_REF_PIC_LIST3: c_uint = 0x19c;

pub const VDPU_REG_INITIAL_REF_PIC_LIST4: c_uint = 0x1a0;

pub const VDPU_REG_INITIAL_REF_PIC_LIST5: c_uint = 0x1a4;

pub const VDPU_REG_INITIAL_REF_PIC_LIST6: c_uint = 0x1a8;

pub const VDPU_REG_LT_REF: c_uint = 0x1ac;
pub const VDPU_REG_VALID_REF: c_uint = 0x1b0;
pub const VDPU_REG_H264_PIC_MB_SIZE: c_uint = 0x1b8;

pub const VDPU_REG_H264_CTRL: c_uint = 0x1bc;

pub const VDPU_REG_CURRENT_FRAME: c_uint = 0x1c0;

pub const VDPU_REG_REF_FRAME: c_uint = 0x1c4;

pub const VDPU_REG_DEC_CTRL6: c_uint = 0x1c8;

pub const VDPU_REG_ENABLE_FLAG: c_uint = 0x1cc;

pub const VDPU_REG_VP8_PIC_MB_SIZE: c_uint = 0x1e0;

pub const VDPU_REG_VP8_DCT_START_BIT: c_uint = 0x1e4;

pub const VDPU_REG_VP8_CTRL0: c_uint = 0x1e8;

pub const VDPU_REG_VP8_DATA_VAL: c_uint = 0x1f0;

pub const VDPU_REG_PRED_FLT7: c_uint = 0x1f4;

pub const VDPU_REG_PRED_FLT8: c_uint = 0x1f8;

pub const VDPU_REG_PRED_FLT9: c_uint = 0x1fc;

pub const VDPU_REG_PRED_FLT10: c_uint = 0x200;

pub const VDPU_REG_FILTER_LEVEL: c_uint = 0x204;

pub const VDPU_REG_VP8_QUANTER0: c_uint = 0x208;

pub const VDPU_REG_VP8_ADDR_REF0: c_uint = 0x20c;
pub const VDPU_REG_FILTER_MB_ADJ: c_uint = 0x210;

pub const VDPU_REG_FILTER_REF_ADJ: c_uint = 0x214;

pub const VDPU_REG_VP8_ADDR_CTRL_PART: c_uint = 0x244;
pub const VDPU_REG_VP8_ADDR_REF1: c_uint = 0x250;
pub const VDPU_REG_VP8_SEGMENT_VAL: c_uint = 0x254;

pub const VDPU_REG_VP8_DCT_START_BIT2: c_uint = 0x258;

pub const VDPU_REG_VP8_QUANTER1: c_uint = 0x25c;

pub const VDPU_REG_VP8_QUANTER2: c_uint = 0x260;

pub const VDPU_REG_PRED_FLT1: c_uint = 0x264;

pub const VDPU_REG_PRED_FLT2: c_uint = 0x268;

pub const VDPU_REG_PRED_FLT3: c_uint = 0x26c;

pub const VDPU_REG_PRED_FLT4: c_uint = 0x270;

pub const VDPU_REG_PRED_FLT5: c_uint = 0x274;

pub const VDPU_REG_PRED_FLT6: c_uint = 0x278;

