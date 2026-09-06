//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/chips-media/coda/coda_regs.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// linux/drivers/media/platform/chips-media/coda_regs.h
//
// Copyright (C) 2012 Vista Silicon SL
// Javier Martin <javier.martin@vista-silicon.com>
// Xavier Duret
//
// HW registers
pub const CODA_REG_BIT_CODE_RUN: c_uint = 0x000;

pub const CODA_REG_BIT_CODE_DOWN: c_uint = 0x004;

pub const CODA_REG_BIT_HOST_IN_REQ: c_uint = 0x008;
pub const CODA_REG_BIT_INT_CLEAR: c_uint = 0x00c;
pub const CODA_REG_BIT_INT_CLEAR_SET: c_uint = 0x1;
pub const CODA_REG_BIT_INT_STATUS: c_uint = 0x010;
pub const CODA_REG_BIT_CODE_RESET: c_uint = 0x014;

pub const CODA_REG_BIT_CUR_PC: c_uint = 0x018;
pub const CODA9_REG_BIT_SW_RESET: c_uint = 0x024;
pub const CODA9_SW_RESET_BPU_CORE: c_uint = 0x008;
pub const CODA9_SW_RESET_BPU_BUS: c_uint = 0x010;
pub const CODA9_SW_RESET_VCE_CORE: c_uint = 0x020;
pub const CODA9_SW_RESET_VCE_BUS: c_uint = 0x040;
pub const CODA9_SW_RESET_GDI_CORE: c_uint = 0x080;
pub const CODA9_SW_RESET_GDI_BUS: c_uint = 0x100;
pub const CODA9_REG_BIT_SW_RESET_STATUS: c_uint = 0x034;
// Static SW registers
pub const CODA_REG_BIT_CODE_BUF_ADDR: c_uint = 0x100;
pub const CODA_REG_BIT_WORK_BUF_ADDR: c_uint = 0x104;
pub const CODA_REG_BIT_PARA_BUF_ADDR: c_uint = 0x108;
pub const CODA_REG_BIT_STREAM_CTRL: c_uint = 0x10c;

pub const CODA_REG_BIT_FRAME_MEM_CTRL: c_uint = 0x110;

pub const CODA_REG_BIT_BIT_STREAM_PARAM: c_uint = 0x114;

pub const CODA_REG_BIT_TEMP_BUF_ADDR: c_uint = 0x118;

pub const CODADX6_REG_BIT_SEARCH_RAM_BASE_ADDR: c_uint = 0x140;
pub const CODA7_REG_BIT_AXI_SRAM_USE: c_uint = 0x140;

pub const CODA_REG_BIT_BUSY: c_uint = 0x160;
pub const CODA_REG_BIT_BUSY_FLAG: c_int = 1;
pub const CODA_REG_BIT_RUN_COMMAND: c_uint = 0x164;
pub const CODA_COMMAND_SEQ_INIT: c_int = 1;
pub const CODA_COMMAND_SEQ_END: c_int = 2;
pub const CODA_COMMAND_PIC_RUN: c_int = 3;
pub const CODA_COMMAND_SET_FRAME_BUF: c_int = 4;
pub const CODA_COMMAND_ENCODE_HEADER: c_int = 5;
pub const CODA_COMMAND_ENC_PARA_SET: c_int = 6;
pub const CODA_COMMAND_DEC_PARA_SET: c_int = 7;
pub const CODA_COMMAND_DEC_BUF_FLUSH: c_int = 8;
pub const CODA_COMMAND_RC_CHANGE_PARAMETER: c_int = 9;
pub const CODA_COMMAND_FIRMWARE_GET: c_uint = 0xf;
pub const CODA_REG_BIT_RUN_INDEX: c_uint = 0x168;

pub const CODA_REG_BIT_RUN_COD_STD: c_uint = 0x16c;
pub const CODADX6_MODE_DECODE_MP4: c_int = 0;
pub const CODADX6_MODE_ENCODE_MP4: c_int = 1;
pub const CODADX6_MODE_DECODE_H264: c_int = 2;
pub const CODADX6_MODE_ENCODE_H264: c_int = 3;
pub const CODA7_MODE_DECODE_H264: c_int = 0;
pub const CODA7_MODE_DECODE_VC1: c_int = 1;
pub const CODA7_MODE_DECODE_MP2: c_int = 2;
pub const CODA7_MODE_DECODE_MP4: c_int = 3;
pub const CODA7_MODE_DECODE_DV3: c_int = 3;
pub const CODA7_MODE_DECODE_RV: c_int = 4;
pub const CODA7_MODE_DECODE_MJPG: c_int = 5;
pub const CODA7_MODE_ENCODE_H264: c_int = 8;
pub const CODA7_MODE_ENCODE_MP4: c_int = 11;
pub const CODA7_MODE_ENCODE_MJPG: c_int = 13;
pub const CODA9_MODE_DECODE_H264: c_int = 0;
pub const CODA9_MODE_DECODE_VC1: c_int = 1;
pub const CODA9_MODE_DECODE_MP2: c_int = 2;
pub const CODA9_MODE_DECODE_MP4: c_int = 3;
pub const CODA9_MODE_DECODE_DV3: c_int = 3;
pub const CODA9_MODE_DECODE_RV: c_int = 4;
pub const CODA9_MODE_DECODE_AVS: c_int = 5;
pub const CODA9_MODE_DECODE_MJPG: c_int = 6;
pub const CODA9_MODE_DECODE_VPX: c_int = 7;
pub const CODA9_MODE_ENCODE_H264: c_int = 8;
pub const CODA9_MODE_ENCODE_MP4: c_int = 11;
pub const CODA9_MODE_ENCODE_MJPG: c_int = 13;
pub const CODA_MODE_INVALID: c_uint = 0xffff;
pub const CODA_REG_BIT_INT_ENABLE: c_uint = 0x170;

pub const CODA_REG_BIT_INT_REASON: c_uint = 0x174;
pub const CODA7_REG_BIT_RUN_AUX_STD: c_uint = 0x178;
pub const CODA_MP4_AUX_MPEG4: c_int = 0;
pub const CODA_MP4_AUX_DIVX3: c_int = 1;
pub const CODA_VPX_AUX_THO: c_int = 0;
pub const CODA_VPX_AUX_VP6: c_int = 1;
pub const CODA_VPX_AUX_VP8: c_int = 2;
pub const CODA_H264_AUX_AVC: c_int = 0;
pub const CODA_H264_AUX_MVC: c_int = 1;
//
// Commands' mailbox:
// registers with offsets in the range 0x180-0x1d0
// have different meaning depending on the command being
// issued.
//
// Decoder Sequence Initialization
pub const CODA_CMD_DEC_SEQ_BB_START: c_uint = 0x180;
pub const CODA_CMD_DEC_SEQ_BB_SIZE: c_uint = 0x184;
pub const CODA_CMD_DEC_SEQ_OPTION: c_uint = 0x188;

pub const CODA_CMD_DEC_SEQ_SRC_SIZE: c_uint = 0x18c;
pub const CODA_CMD_DEC_SEQ_START_BYTE: c_uint = 0x190;
pub const CODA_CMD_DEC_SEQ_PS_BB_START: c_uint = 0x194;
pub const CODA_CMD_DEC_SEQ_PS_BB_SIZE: c_uint = 0x198;
pub const CODA_CMD_DEC_SEQ_JPG_THUMB_EN: c_uint = 0x19c;
pub const CODA_CMD_DEC_SEQ_MP4_ASP_CLASS: c_uint = 0x19c;
pub const CODA_MP4_CLASS_MPEG4: c_int = 0;
pub const CODA_CMD_DEC_SEQ_X264_MV_EN: c_uint = 0x19c;
pub const CODA_CMD_DEC_SEQ_SPP_CHUNK_SIZE: c_uint = 0x1a0;
pub const CODA7_RET_DEC_SEQ_ASPECT: c_uint = 0x1b0;
pub const CODA9_RET_DEC_SEQ_BITRATE: c_uint = 0x1b4;
pub const CODA_RET_DEC_SEQ_SUCCESS: c_uint = 0x1c0;
pub const CODA_RET_DEC_SEQ_SRC_FMT: c_uint = 0x1c4 /* SRC_SIZE on CODA7 */;
pub const CODA_RET_DEC_SEQ_SRC_SIZE: c_uint = 0x1c4;
pub const CODA_RET_DEC_SEQ_SRC_F_RATE: c_uint = 0x1c8;
pub const CODA9_RET_DEC_SEQ_ASPECT: c_uint = 0x1c8;
pub const CODA_RET_DEC_SEQ_FRAME_NEED: c_uint = 0x1cc;
pub const CODA_RET_DEC_SEQ_FRAME_DELAY: c_uint = 0x1d0;
pub const CODA_RET_DEC_SEQ_INFO: c_uint = 0x1d4;
pub const CODA_RET_DEC_SEQ_CROP_LEFT_RIGHT: c_uint = 0x1d8;
pub const CODA_RET_DEC_SEQ_CROP_TOP_BOTTOM: c_uint = 0x1dc;
pub const CODA_RET_DEC_SEQ_NEXT_FRAME_NUM: c_uint = 0x1e0;
pub const CODA_RET_DEC_SEQ_ERR_REASON: c_uint = 0x1e0;
pub const CODA_RET_DEC_SEQ_FRATE_NR: c_uint = 0x1e4;
pub const CODA_RET_DEC_SEQ_FRATE_DR: c_uint = 0x1e8;
pub const CODA_RET_DEC_SEQ_JPG_PARA: c_uint = 0x1e4;
pub const CODA_RET_DEC_SEQ_JPG_THUMB_IND: c_uint = 0x1e8;
pub const CODA7_RET_DEC_SEQ_HEADER_REPORT: c_uint = 0x1ec;
// Decoder Picture Run
pub const CODA_CMD_DEC_PIC_ROT_MODE: c_uint = 0x180;
pub const CODA_CMD_DEC_PIC_ROT_ADDR_Y: c_uint = 0x184;
pub const CODA9_CMD_DEC_PIC_ROT_INDEX: c_uint = 0x184;
pub const CODA_CMD_DEC_PIC_ROT_ADDR_CB: c_uint = 0x188;
pub const CODA9_CMD_DEC_PIC_ROT_ADDR_Y: c_uint = 0x188;
pub const CODA_CMD_DEC_PIC_ROT_ADDR_CR: c_uint = 0x18c;
pub const CODA9_CMD_DEC_PIC_ROT_ADDR_CB: c_uint = 0x18c;
pub const CODA_CMD_DEC_PIC_ROT_STRIDE: c_uint = 0x190;
pub const CODA9_CMD_DEC_PIC_ROT_ADDR_CR: c_uint = 0x190;
pub const CODA9_CMD_DEC_PIC_ROT_STRIDE: c_uint = 0x1b8;
pub const CODA_CMD_DEC_PIC_OPTION: c_uint = 0x194;

pub const CODA_CMD_DEC_PIC_SKIP_NUM: c_uint = 0x198;
pub const CODA_CMD_DEC_PIC_CHUNK_SIZE: c_uint = 0x19c;
pub const CODA_CMD_DEC_PIC_BB_START: c_uint = 0x1a0;
pub const CODA_CMD_DEC_PIC_START_BYTE: c_uint = 0x1a4;
pub const CODA_RET_DEC_PIC_SIZE: c_uint = 0x1bc;
pub const CODA_RET_DEC_PIC_FRAME_NUM: c_uint = 0x1c0;
pub const CODA_RET_DEC_PIC_FRAME_IDX: c_uint = 0x1c4;
pub const CODA_RET_DEC_PIC_ERR_MB: c_uint = 0x1c8;
pub const CODA_RET_DEC_PIC_TYPE: c_uint = 0x1cc;
pub const CODA_PIC_TYPE_MASK: c_uint = 0x7;
pub const CODA_PIC_TYPE_MASK_VC1: c_uint = 0x3f;

pub const CODA_RET_DEC_PIC_POST: c_uint = 0x1d0;
pub const CODA_RET_DEC_PIC_MVC_REPORT: c_uint = 0x1d0;
pub const CODA_RET_DEC_PIC_OPTION: c_uint = 0x1d4;
pub const CODA_RET_DEC_PIC_SUCCESS: c_uint = 0x1d8;
pub const CODA_RET_DEC_PIC_CUR_IDX: c_uint = 0x1dc;
pub const CODA_RET_DEC_PIC_CROP_LEFT_RIGHT: c_uint = 0x1e0;
pub const CODA_RET_DEC_PIC_CROP_TOP_BOTTOM: c_uint = 0x1e4;
pub const CODA_RET_DEC_PIC_FRAME_NEED: c_uint = 0x1ec;
pub const CODA9_RET_DEC_PIC_VP8_PIC_REPORT: c_uint = 0x1e8;
pub const CODA9_RET_DEC_PIC_ASPECT: c_uint = 0x1f0;
pub const CODA9_RET_DEC_PIC_VP8_SCALE_INFO: c_uint = 0x1f0;
pub const CODA9_RET_DEC_PIC_FRATE_NR: c_uint = 0x1f4;
pub const CODA9_RET_DEC_PIC_FRATE_DR: c_uint = 0x1f8;
// Encoder Sequence Initialization
pub const CODA_CMD_ENC_SEQ_BB_START: c_uint = 0x180;
pub const CODA_CMD_ENC_SEQ_BB_SIZE: c_uint = 0x184;
pub const CODA_CMD_ENC_SEQ_OPTION: c_uint = 0x188;
pub const CODA7_OPTION_AVCINTRA16X16ONLY_OFFSET: c_int = 9;
pub const CODA9_OPTION_MVC_PREFIX_NAL_OFFSET: c_int = 9;
pub const CODA7_OPTION_GAMMA_OFFSET: c_int = 8;
pub const CODA9_OPTION_MVC_PARASET_REFRESH_OFFSET: c_int = 8;
pub const CODA7_OPTION_RCQPMAX_OFFSET: c_int = 7;
pub const CODA9_OPTION_GAMMA_OFFSET: c_int = 7;
pub const CODADX6_OPTION_GAMMA_OFFSET: c_int = 7;
pub const CODA7_OPTION_RCQPMIN_OFFSET: c_int = 6;
pub const CODA9_OPTION_RCQPMAX_OFFSET: c_int = 6;
pub const CODA_OPTION_LIMITQP_OFFSET: c_int = 6;
pub const CODA_OPTION_RCINTRAQP_OFFSET: c_int = 5;
pub const CODA_OPTION_FMO_OFFSET: c_int = 4;
pub const CODA9_OPTION_MVC_INTERVIEW_OFFSET: c_int = 4;
pub const CODA_OPTION_AVC_AUD_OFFSET: c_int = 2;
pub const CODA_OPTION_SLICEREPORT_OFFSET: c_int = 1;
pub const CODA_CMD_ENC_SEQ_COD_STD: c_uint = 0x18c;
pub const CODA_STD_MPEG4: c_int = 0;
pub const CODA9_STD_H264: c_int = 0;
pub const CODA_STD_H263: c_int = 1;
pub const CODA_STD_H264: c_int = 2;
pub const CODA9_STD_MPEG4: c_int = 3;
pub const CODA_CMD_ENC_SEQ_SRC_SIZE: c_uint = 0x190;
pub const CODA7_PICWIDTH_OFFSET: c_int = 16;
pub const CODA7_PICWIDTH_MASK: c_uint = 0xffff;
pub const CODADX6_PICWIDTH_OFFSET: c_int = 10;
pub const CODADX6_PICWIDTH_MASK: c_uint = 0x3ff;
pub const CODA_PICHEIGHT_OFFSET: c_int = 0;
pub const CODADX6_PICHEIGHT_MASK: c_uint = 0x3ff;
pub const CODA7_PICHEIGHT_MASK: c_uint = 0xffff;
pub const CODA_CMD_ENC_SEQ_SRC_F_RATE: c_uint = 0x194;
pub const CODA_FRATE_RES_OFFSET: c_int = 0;
pub const CODA_FRATE_RES_MASK: c_uint = 0xffff;
pub const CODA_FRATE_DIV_OFFSET: c_int = 16;
pub const CODA_FRATE_DIV_MASK: c_uint = 0xffff;
pub const CODA_CMD_ENC_SEQ_MP4_PARA: c_uint = 0x198;
pub const CODA_MP4PARAM_VERID_OFFSET: c_int = 6;
pub const CODA_MP4PARAM_VERID_MASK: c_uint = 0x01;
pub const CODA_MP4PARAM_INTRADCVLCTHR_OFFSET: c_int = 2;
pub const CODA_MP4PARAM_INTRADCVLCTHR_MASK: c_uint = 0x07;
pub const CODA_MP4PARAM_REVERSIBLEVLCENABLE_OFFSET: c_int = 1;
pub const CODA_MP4PARAM_REVERSIBLEVLCENABLE_MASK: c_uint = 0x01;
pub const CODA_MP4PARAM_DATAPARTITIONENABLE_OFFSET: c_int = 0;
pub const CODA_MP4PARAM_DATAPARTITIONENABLE_MASK: c_uint = 0x01;
pub const CODA_CMD_ENC_SEQ_263_PARA: c_uint = 0x19c;
pub const CODA_263PARAM_ANNEXJENABLE_OFFSET: c_int = 2;
pub const CODA_263PARAM_ANNEXJENABLE_MASK: c_uint = 0x01;
pub const CODA_263PARAM_ANNEXKENABLE_OFFSET: c_int = 1;
pub const CODA_263PARAM_ANNEXKENABLE_MASK: c_uint = 0x01;
pub const CODA_263PARAM_ANNEXTENABLE_OFFSET: c_int = 0;
pub const CODA_263PARAM_ANNEXTENABLE_MASK: c_uint = 0x01;
pub const CODA_CMD_ENC_SEQ_264_PARA: c_uint = 0x1a0;
pub const CODA_264PARAM_DEBLKFILTEROFFSETBETA_OFFSET: c_int = 12;
pub const CODA_264PARAM_DEBLKFILTEROFFSETBETA_MASK: c_uint = 0x0f;
pub const CODA_264PARAM_DEBLKFILTEROFFSETALPHA_OFFSET: c_int = 8;
pub const CODA_264PARAM_DEBLKFILTEROFFSETALPHA_MASK: c_uint = 0x0f;
pub const CODA_264PARAM_DISABLEDEBLK_OFFSET: c_int = 6;
pub const CODA_264PARAM_DISABLEDEBLK_MASK: c_uint = 0x03;
pub const CODA_264PARAM_CONSTRAINEDINTRAPREDFLAG_OFFSET: c_int = 5;
pub const CODA_264PARAM_CONSTRAINEDINTRAPREDFLAG_MASK: c_uint = 0x01;
pub const CODA_264PARAM_CHROMAQPOFFSET_OFFSET: c_int = 0;
pub const CODA_264PARAM_CHROMAQPOFFSET_MASK: c_uint = 0x1f;
pub const CODA_CMD_ENC_SEQ_SLICE_MODE: c_uint = 0x1a4;
pub const CODA_SLICING_SIZE_OFFSET: c_int = 2;
pub const CODA_SLICING_SIZE_MASK: c_uint = 0x3fffffff;
pub const CODA_SLICING_UNIT_OFFSET: c_int = 1;
pub const CODA_SLICING_UNIT_MASK: c_uint = 0x01;
pub const CODA_SLICING_MODE_OFFSET: c_int = 0;
pub const CODA_SLICING_MODE_MASK: c_uint = 0x01;
pub const CODA_CMD_ENC_SEQ_GOP_SIZE: c_uint = 0x1a8;
pub const CODA_GOP_SIZE_OFFSET: c_int = 0;
pub const CODA_GOP_SIZE_MASK: c_uint = 0x3f;
pub const CODA_CMD_ENC_SEQ_RC_PARA: c_uint = 0x1ac;
pub const CODA_RATECONTROL_AUTOSKIP_OFFSET: c_int = 31;
pub const CODA_RATECONTROL_AUTOSKIP_MASK: c_uint = 0x01;
pub const CODA_RATECONTROL_INITIALDELAY_OFFSET: c_int = 16;
pub const CODA_RATECONTROL_INITIALDELAY_MASK: c_uint = 0x7fff;
pub const CODA_RATECONTROL_BITRATE_OFFSET: c_int = 1;
pub const CODA_RATECONTROL_BITRATE_MASK: c_uint = 0x7fff;
pub const CODA_RATECONTROL_ENABLE_OFFSET: c_int = 0;
pub const CODA_RATECONTROL_ENABLE_MASK: c_uint = 0x01;
pub const CODA_CMD_ENC_SEQ_RC_BUF_SIZE: c_uint = 0x1b0;
pub const CODA_CMD_ENC_SEQ_INTRA_REFRESH: c_uint = 0x1b4;
pub const CODADX6_CMD_ENC_SEQ_FMO: c_uint = 0x1b8;
pub const CODA_FMOPARAM_TYPE_OFFSET: c_int = 4;
pub const CODA_FMOPARAM_TYPE_MASK: c_int = 1;
pub const CODA_FMOPARAM_SLICENUM_OFFSET: c_int = 0;
pub const CODA_FMOPARAM_SLICENUM_MASK: c_uint = 0x0f;
pub const CODADX6_CMD_ENC_SEQ_INTRA_QP: c_uint = 0x1bc;
pub const CODA7_CMD_ENC_SEQ_SEARCH_BASE: c_uint = 0x1b8;
pub const CODA7_CMD_ENC_SEQ_SEARCH_SIZE: c_uint = 0x1bc;
pub const CODA7_CMD_ENC_SEQ_INTRA_QP: c_uint = 0x1c4;
pub const CODA_CMD_ENC_SEQ_RC_QP_MIN_MAX: c_uint = 0x1c8;
pub const CODA_QPMIN_OFFSET: c_int = 8;
pub const CODA_QPMIN_MASK: c_uint = 0x3f;
pub const CODA_QPMAX_OFFSET: c_int = 0;
pub const CODA_QPMAX_MASK: c_uint = 0x3f;
pub const CODA_CMD_ENC_SEQ_RC_GAMMA: c_uint = 0x1cc;
pub const CODA_GAMMA_OFFSET: c_int = 0;
pub const CODA_GAMMA_MASK: c_uint = 0xffff;
pub const CODA_CMD_ENC_SEQ_RC_INTERVAL_MODE: c_uint = 0x1d0;
pub const CODA9_CMD_ENC_SEQ_INTRA_WEIGHT: c_uint = 0x1d4;
pub const CODA9_CMD_ENC_SEQ_ME_OPTION: c_uint = 0x1d8;
pub const CODA_RET_ENC_SEQ_SUCCESS: c_uint = 0x1c0;
pub const CODA_CMD_ENC_SEQ_JPG_PARA: c_uint = 0x198;
pub const CODA_CMD_ENC_SEQ_JPG_RST_INTERVAL: c_uint = 0x19C;
pub const CODA_CMD_ENC_SEQ_JPG_THUMB_EN: c_uint = 0x1a0;
pub const CODA_CMD_ENC_SEQ_JPG_THUMB_SIZE: c_uint = 0x1a4;
pub const CODA_CMD_ENC_SEQ_JPG_THUMB_OFFSET: c_uint = 0x1a8;
// Encoder Parameter Change
pub const CODA_CMD_ENC_PARAM_CHANGE_ENABLE: c_uint = 0x180;

pub const CODA_CMD_ENC_PARAM_RC_GOP: c_uint = 0x184;
pub const CODA_CMD_ENC_PARAM_RC_INTRA_QP: c_uint = 0x188;
pub const CODA_CMD_ENC_PARAM_RC_BITRATE: c_uint = 0x18c;
pub const CODA_CMD_ENC_PARAM_RC_FRAME_RATE: c_uint = 0x190;
pub const CODA_CMD_ENC_PARAM_INTRA_MB_NUM: c_uint = 0x194;
pub const CODA_CMD_ENC_PARAM_SLICE_MODE: c_uint = 0x198;
pub const CODA_CMD_ENC_PARAM_HEC_MODE: c_uint = 0x19c;
pub const CODA_RET_ENC_PARAM_CHANGE_SUCCESS: c_uint = 0x1c0;
// Encoder Picture Run
pub const CODA9_CMD_ENC_PIC_SRC_INDEX: c_uint = 0x180;
pub const CODA9_CMD_ENC_PIC_SRC_STRIDE: c_uint = 0x184;
pub const CODA9_CMD_ENC_PIC_SUB_FRAME_SYNC: c_uint = 0x1a4;
pub const CODA9_CMD_ENC_PIC_SRC_ADDR_Y: c_uint = 0x1a8;
pub const CODA9_CMD_ENC_PIC_SRC_ADDR_CB: c_uint = 0x1ac;
pub const CODA9_CMD_ENC_PIC_SRC_ADDR_CR: c_uint = 0x1b0;
pub const CODA_CMD_ENC_PIC_SRC_ADDR_Y: c_uint = 0x180;
pub const CODA_CMD_ENC_PIC_SRC_ADDR_CB: c_uint = 0x184;
pub const CODA_CMD_ENC_PIC_SRC_ADDR_CR: c_uint = 0x188;
pub const CODA_CMD_ENC_PIC_QS: c_uint = 0x18c;
pub const CODA_CMD_ENC_PIC_ROT_MODE: c_uint = 0x190;

pub const CODA_CMD_ENC_PIC_OPTION: c_uint = 0x194;

pub const CODA_CMD_ENC_PIC_BB_START: c_uint = 0x198;
pub const CODA_CMD_ENC_PIC_BB_SIZE: c_uint = 0x19c;
pub const CODA_RET_ENC_FRAME_NUM: c_uint = 0x1c0;
pub const CODA_RET_ENC_PIC_TYPE: c_uint = 0x1c4;
pub const CODA_RET_ENC_PIC_FRAME_IDX: c_uint = 0x1c8;
pub const CODA_RET_ENC_PIC_SLICE_NUM: c_uint = 0x1cc;
pub const CODA_RET_ENC_PIC_FLAG: c_uint = 0x1d0;
pub const CODA_RET_ENC_PIC_SUCCESS: c_uint = 0x1d8;
// Set Frame Buffer
pub const CODA_CMD_SET_FRAME_BUF_NUM: c_uint = 0x180;
pub const CODA_CMD_SET_FRAME_BUF_STRIDE: c_uint = 0x184;
pub const CODA_CMD_SET_FRAME_SLICE_BB_START: c_uint = 0x188;
pub const CODA_CMD_SET_FRAME_SLICE_BB_SIZE: c_uint = 0x18c;
pub const CODA9_CMD_SET_FRAME_SUBSAMP_A: c_uint = 0x188;
pub const CODA9_CMD_SET_FRAME_SUBSAMP_B: c_uint = 0x18c;
pub const CODA7_CMD_SET_FRAME_AXI_BIT_ADDR: c_uint = 0x190;
pub const CODA7_CMD_SET_FRAME_AXI_IPACDC_ADDR: c_uint = 0x194;
pub const CODA7_CMD_SET_FRAME_AXI_DBKY_ADDR: c_uint = 0x198;
pub const CODA7_CMD_SET_FRAME_AXI_DBKC_ADDR: c_uint = 0x19c;
pub const CODA7_CMD_SET_FRAME_AXI_OVL_ADDR: c_uint = 0x1a0;
pub const CODA7_CMD_SET_FRAME_MAX_DEC_SIZE: c_uint = 0x1a4;
pub const CODA9_CMD_SET_FRAME_AXI_BTP_ADDR: c_uint = 0x1a4;
pub const CODA7_CMD_SET_FRAME_SOURCE_BUF_STRIDE: c_uint = 0x1a8;
pub const CODA9_CMD_SET_FRAME_CACHE_SIZE: c_uint = 0x1a8;
pub const CODA9_CMD_SET_FRAME_CACHE_CONFIG: c_uint = 0x1ac;
pub const CODA9_CACHE_BYPASS_OFFSET: c_int = 28;
pub const CODA9_CACHE_DUALCONF_OFFSET: c_int = 26;
pub const CODA9_CACHE_PAGEMERGE_OFFSET: c_int = 24;
pub const CODA9_CACHE_LUMA_BUFFER_SIZE_OFFSET: c_int = 16;
pub const CODA9_CACHE_CB_BUFFER_SIZE_OFFSET: c_int = 8;
pub const CODA9_CACHE_CR_BUFFER_SIZE_OFFSET: c_int = 0;
pub const CODA9_CMD_SET_FRAME_SUBSAMP_A_MVC: c_uint = 0x1b0;
pub const CODA9_CMD_SET_FRAME_SUBSAMP_B_MVC: c_uint = 0x1b4;
pub const CODA9_CMD_SET_FRAME_DP_BUF_BASE: c_uint = 0x1b0;
pub const CODA9_CMD_SET_FRAME_DP_BUF_SIZE: c_uint = 0x1b4;
pub const CODA9_CMD_SET_FRAME_MAX_DEC_SIZE: c_uint = 0x1b8;
pub const CODA9_CMD_SET_FRAME_DELAY: c_uint = 0x1bc;
// Encoder Header
pub const CODA_CMD_ENC_HEADER_CODE: c_uint = 0x180;
pub const CODA_GAMMA_OFFSET: c_int = 0;
pub const CODA_HEADER_H264_SPS: c_int = 0;
pub const CODA_HEADER_H264_PPS: c_int = 1;
pub const CODA_HEADER_MP4V_VOL: c_int = 0;
pub const CODA_HEADER_MP4V_VOS: c_int = 1;
pub const CODA_HEADER_MP4V_VIS: c_int = 2;

pub const CODA_CMD_ENC_HEADER_BB_START: c_uint = 0x184;
pub const CODA_CMD_ENC_HEADER_BB_SIZE: c_uint = 0x188;
pub const CODA9_CMD_ENC_HEADER_FRAME_CROP_H: c_uint = 0x18c;
pub const CODA9_CMD_ENC_HEADER_FRAME_CROP_V: c_uint = 0x190;
// Get Version
pub const CODA_CMD_FIRMWARE_VERNUM: c_uint = 0x1c0;

pub const CODA9_CMD_FIRMWARE_CODE_REV: c_uint = 0x1c4;
pub const CODA9_GDMA_BASE: c_uint = 0x1000;

pub const CODA9_JPEG_BASE: c_uint = 0x3000;

pub const CODA9_JPEG_ERRMB_MCU_POS_Y_MASK: c_uint = 0xfff;

pub const CODA9_JPEG_MCU_BLOCK_NUM_OFFSET: c_int = 16;
pub const CODA9_JPEG_COMP_NUM_OFFSET: c_int = 12;
pub const CODA9_JPEG_COMP0_INFO_OFFSET: c_int = 8;
pub const CODA9_JPEG_COMP1_INFO_OFFSET: c_int = 4;
pub const CODA9_JPEG_COMP2_INFO_OFFSET: c_int = 0;

pub const CODA9_JPEG_ROT_MIR_MODE_MASK: c_uint = 0xf;

pub const CODA9_JPEG_BUS_REQ_NUM_OFFSET: c_int = 0;
pub const CODA9_JPEG_BUS_REQ_NUM_MASK: c_uint = 0x7;

