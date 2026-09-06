//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc.h
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
// Register definition file for Samsung MFC V5.1 Interface (FIMV) driver
//
// Kamil Debski, Copyright (c) 2010 Samsung Electronics
// http://www.samsung.com
//

// Number of bits that the buffer address should be shifted for particular
// MFC buffers.
pub const S5P_FIMV_START_ADDR: c_uint = 0x0000;
pub const S5P_FIMV_END_ADDR: c_uint = 0xe008;
pub const S5P_FIMV_SW_RESET: c_uint = 0x0000;
pub const S5P_FIMV_RISC_HOST_INT: c_uint = 0x0008;
// Command from HOST to RISC
pub const S5P_FIMV_HOST2RISC_CMD: c_uint = 0x0030;
pub const S5P_FIMV_HOST2RISC_ARG1: c_uint = 0x0034;
pub const S5P_FIMV_HOST2RISC_ARG2: c_uint = 0x0038;
pub const S5P_FIMV_HOST2RISC_ARG3: c_uint = 0x003c;
pub const S5P_FIMV_HOST2RISC_ARG4: c_uint = 0x0040;
// Command from RISC to HOST
pub const S5P_FIMV_RISC2HOST_CMD: c_uint = 0x0044;
pub const S5P_FIMV_RISC2HOST_CMD_MASK: c_uint = 0x1FFFF;
pub const S5P_FIMV_RISC2HOST_ARG1: c_uint = 0x0048;
pub const S5P_FIMV_RISC2HOST_ARG2: c_uint = 0x004c;
pub const S5P_FIMV_RISC2HOST_ARG3: c_uint = 0x0050;
pub const S5P_FIMV_RISC2HOST_ARG4: c_uint = 0x0054;
pub const S5P_FIMV_FW_VERSION: c_uint = 0x0058;
pub const S5P_FIMV_SYS_MEM_SZ: c_uint = 0x005c;
pub const S5P_FIMV_FW_STATUS: c_uint = 0x0080;
// Memory controller register
pub const S5P_FIMV_MC_DRAMBASE_ADR_A: c_uint = 0x0508;
pub const S5P_FIMV_MC_DRAMBASE_ADR_B: c_uint = 0x050c;
pub const S5P_FIMV_MC_STATUS: c_uint = 0x0510;
// Common register
pub const S5P_FIMV_COMMON_BASE_A: c_uint = 0x0600;
pub const S5P_FIMV_COMMON_BASE_B: c_uint = 0x0700;
// Decoder

// H.264 decoding

// vertical neighbor motion vector

// neighbor pixels for intra pred

// H264 motion vector
// MPEG4 decoding

// neighbor AC/DC coeff.

// upper neighbor motion vector

// subseq. anchor motion vector

// overlap transform line

// syntax parser
// H.263 decoding

// VC-1 decoding

// bitplane3

// bitplane2

// bitplane1
// Encoder

// reconstructed luma

// reconstructed chroma

// H.264 encoding

// upper motion vector

// entropy engine's neighbor info.

// upper intra MD

// direct cozero flag

// upper intra PRED
// H.263 encoding

// upper motion vector

// upper Q coeff.
// MPEG4 encoding

// upper motion vector

// upper Q coeff.

// direct cozero flag
pub const S5P_FIMV_ENC_REF_B_LUMA_ADR: c_uint = 0x062c /* ref B Luma addr */;
pub const S5P_FIMV_ENC_REF_B_CHROMA_ADR: c_uint = 0x0630 /* ref B Chroma addr */;
pub const S5P_FIMV_ENC_CUR_LUMA_ADR: c_uint = 0x0718 /* current Luma addr */;
pub const S5P_FIMV_ENC_CUR_CHROMA_ADR: c_uint = 0x071C /* current Chroma addr */;
// Codec common register
pub const S5P_FIMV_ENC_HSIZE_PX: c_uint = 0x0818 /* frame width at encoder */;
pub const S5P_FIMV_ENC_VSIZE_PX: c_uint = 0x081c /* frame height at encoder */;
pub const S5P_FIMV_ENC_PROFILE: c_uint = 0x0830 /* profile register */;
pub const S5P_FIMV_ENC_PROFILE_H264_MAIN: c_int = 0;
pub const S5P_FIMV_ENC_PROFILE_H264_HIGH: c_int = 1;
pub const S5P_FIMV_ENC_PROFILE_H264_BASELINE: c_int = 2;
pub const S5P_FIMV_ENC_PROFILE_H264_CONSTRAINED_BASELINE: c_int = 3;
pub const S5P_FIMV_ENC_PROFILE_MPEG4_SIMPLE: c_int = 0;
pub const S5P_FIMV_ENC_PROFILE_MPEG4_ADVANCED_SIMPLE: c_int = 1;
pub const S5P_FIMV_ENC_PIC_STRUCT: c_uint = 0x083c /* picture field/frame flag */;
pub const S5P_FIMV_ENC_LF_CTRL: c_uint = 0x0848 /* loop filter control */;
pub const S5P_FIMV_ENC_ALPHA_OFF: c_uint = 0x084c /* loop filter alpha offset */;
pub const S5P_FIMV_ENC_BETA_OFF: c_uint = 0x0850 /* loop filter beta offset */;
pub const S5P_FIMV_MR_BUSIF_CTRL: c_uint = 0x0854 /* hidden, bus interface ctrl */;
pub const S5P_FIMV_ENC_PXL_CACHE_CTRL: c_uint = 0x0a00 /* pixel cache control */;
// Channel & stream interface register
pub const S5P_FIMV_SI_RTN_CHID: c_uint = 0x2000 /* Return CH inst ID register */;
pub const S5P_FIMV_SI_CH0_INST_ID: c_uint = 0x2040 /* codec instance ID */;
pub const S5P_FIMV_SI_CH1_INST_ID: c_uint = 0x2080 /* codec instance ID */;
// Decoder
pub const S5P_FIMV_SI_VRESOL: c_uint = 0x2004 /* vertical res of decoder */;
pub const S5P_FIMV_SI_HRESOL: c_uint = 0x2008 /* horizontal res of decoder */;
pub const S5P_FIMV_SI_BUF_NUMBER: c_uint = 0x200c /* number of frames in the;
pub const S5P_FIMV_SI_DISPLAY_Y_ADR: c_uint = 0x2010 /* luma addr of displayed pic */;
pub const S5P_FIMV_SI_DISPLAY_C_ADR: c_uint = 0x2014 /* chroma addrof displayed pic */;
pub const S5P_FIMV_SI_CONSUMED_BYTES: c_uint = 0x2018 /* Consumed number of bytes to;
pub const S5P_FIMV_SI_DISPLAY_STATUS: c_uint = 0x201c /* status of decoded picture */;
pub const S5P_FIMV_SI_DECODE_Y_ADR: c_uint = 0x2024 /* luma addr of decoded pic */;
pub const S5P_FIMV_SI_DECODE_C_ADR: c_uint = 0x2028 /* chroma addrof decoded pic */;
pub const S5P_FIMV_SI_DECODE_STATUS: c_uint = 0x202c /* status of decoded picture */;
pub const S5P_FIMV_SI_CH0_SB_ST_ADR: c_uint = 0x2044 /* start addr of stream buf */;
pub const S5P_FIMV_SI_CH0_SB_FRM_SIZE: c_uint = 0x2048 /* size of stream buf */;
pub const S5P_FIMV_SI_CH0_DESC_ADR: c_uint = 0x204c /* addr of descriptor buf */;
pub const S5P_FIMV_SI_CH0_CPB_SIZE: c_uint = 0x2058 /* max size of coded pic. buf */;
pub const S5P_FIMV_SI_CH0_DESC_SIZE: c_uint = 0x205c /* max size of descriptor buf */;
pub const S5P_FIMV_SI_CH1_SB_ST_ADR: c_uint = 0x2084 /* start addr of stream buf */;
pub const S5P_FIMV_SI_CH1_SB_FRM_SIZE: c_uint = 0x2088 /* size of stream buf */;
pub const S5P_FIMV_SI_CH1_DESC_ADR: c_uint = 0x208c /* addr of descriptor buf */;
pub const S5P_FIMV_SI_CH1_CPB_SIZE: c_uint = 0x2098 /* max size of coded pic. buf */;
pub const S5P_FIMV_SI_CH1_DESC_SIZE: c_uint = 0x209c /* max size of descriptor buf */;
pub const S5P_FIMV_CRC_LUMA0: c_uint = 0x2030 /* luma crc data per frame;
pub const S5P_FIMV_CRC_CHROMA0: c_uint = 0x2034 /* chroma crc data per frame;
pub const S5P_FIMV_CRC_LUMA1: c_uint = 0x2038 /* luma crc data per bottom;
pub const S5P_FIMV_CRC_CHROMA1: c_uint = 0x203c /* chroma crc data per bottom;
// Display status
pub const S5P_FIMV_DEC_STATUS_DECODING_ONLY: c_int = 0;
pub const S5P_FIMV_DEC_STATUS_DECODING_DISPLAY: c_int = 1;
pub const S5P_FIMV_DEC_STATUS_DISPLAY_ONLY: c_int = 2;
pub const S5P_FIMV_DEC_STATUS_DECODING_EMPTY: c_int = 3;
pub const S5P_FIMV_DEC_STATUS_DECODING_STATUS_MASK: c_int = 7;

pub const S5P_FIMV_DEC_STATUS_RESOLUTION_SHIFT: c_int = 4;
// Decode frame address
pub const S5P_FIMV_DECODE_Y_ADR: c_uint = 0x2024;
pub const S5P_FIMV_DECODE_C_ADR: c_uint = 0x2028;
// Decoded frame tpe
pub const S5P_FIMV_DECODE_FRAME_TYPE: c_uint = 0x2020;
pub const S5P_FIMV_DECODE_FRAME_MASK: c_int = 7;
pub const S5P_FIMV_DECODE_FRAME_SKIPPED: c_int = 0;
pub const S5P_FIMV_DECODE_FRAME_I_FRAME: c_int = 1;
pub const S5P_FIMV_DECODE_FRAME_P_FRAME: c_int = 2;
pub const S5P_FIMV_DECODE_FRAME_B_FRAME: c_int = 3;
pub const S5P_FIMV_DECODE_FRAME_OTHER_FRAME: c_int = 4;
// Sizes of buffers required for decoding

pub const S5P_FIMV_NV12M_HALIGN: c_int = 16;
pub const S5P_FIMV_NV12M_LVALIGN: c_int = 16;
pub const S5P_FIMV_NV12M_CVALIGN: c_int = 8;
pub const S5P_FIMV_NV12MT_HALIGN: c_int = 128;
pub const S5P_FIMV_NV12MT_VALIGN: c_int = 32;
pub const S5P_FIMV_NV12M_SALIGN: c_int = 2048;
pub const S5P_FIMV_NV12MT_SALIGN: c_int = 8192;
// Sizes of buffers required for encoding
pub const S5P_FIMV_ENC_UPMV_SIZE: c_uint = 0x10000;
pub const S5P_FIMV_ENC_COLFLG_SIZE: c_uint = 0x10000;
pub const S5P_FIMV_ENC_INTRAMD_SIZE: c_uint = 0x10000;
pub const S5P_FIMV_ENC_INTRAPRED_SIZE: c_uint = 0x4000;
pub const S5P_FIMV_ENC_NBORINFO_SIZE: c_uint = 0x10000;
pub const S5P_FIMV_ENC_ACDCCOEF_SIZE: c_uint = 0x10000;
// Encoder
pub const S5P_FIMV_ENC_SI_STRM_SIZE: c_uint = 0x2004 /* stream size */;
pub const S5P_FIMV_ENC_SI_PIC_CNT: c_uint = 0x2008 /* picture count */;
pub const S5P_FIMV_ENC_SI_WRITE_PTR: c_uint = 0x200c /* write pointer */;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE: c_uint = 0x2010 /* slice type(I/P/B/IDR) */;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_NON_CODED: c_int = 0;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_I: c_int = 1;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_P: c_int = 2;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_B: c_int = 3;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_SKIPPED: c_int = 4;
pub const S5P_FIMV_ENC_SI_SLICE_TYPE_OTHERS: c_int = 5;
pub const S5P_FIMV_ENCODED_Y_ADDR: c_uint = 0x2014 /* the addr of the encoded;
pub const S5P_FIMV_ENCODED_C_ADDR: c_uint = 0x2018 /* the addr of the encoded;
pub const S5P_FIMV_ENC_SI_CH0_SB_ADR: c_uint = 0x2044 /* addr of stream buf */;
pub const S5P_FIMV_ENC_SI_CH0_SB_SIZE: c_uint = 0x204c /* size of stream buf */;
pub const S5P_FIMV_ENC_SI_CH0_CUR_Y_ADR: c_uint = 0x2050 /* current Luma addr */;
pub const S5P_FIMV_ENC_SI_CH0_CUR_C_ADR: c_uint = 0x2054 /* current Chroma addr */;
pub const S5P_FIMV_ENC_SI_CH0_FRAME_INS: c_uint = 0x2058 /* frame insertion */;
pub const S5P_FIMV_ENC_SI_CH1_SB_ADR: c_uint = 0x2084 /* addr of stream buf */;
pub const S5P_FIMV_ENC_SI_CH1_SB_SIZE: c_uint = 0x208c /* size of stream buf */;
pub const S5P_FIMV_ENC_SI_CH1_CUR_Y_ADR: c_uint = 0x2090 /* current Luma addr */;
pub const S5P_FIMV_ENC_SI_CH1_CUR_C_ADR: c_uint = 0x2094 /* current Chroma addr */;
pub const S5P_FIMV_ENC_SI_CH1_FRAME_INS: c_uint = 0x2098 /* frame insertion */;
pub const S5P_FIMV_ENC_PIC_TYPE_CTRL: c_uint = 0xc504 /* pic type level control */;
pub const S5P_FIMV_ENC_B_RECON_WRITE_ON: c_uint = 0xc508 /* B frame recon write ctrl */;
pub const S5P_FIMV_ENC_MSLICE_CTRL: c_uint = 0xc50c /* multi slice control */;
pub const S5P_FIMV_ENC_MSLICE_MB: c_uint = 0xc510 /* MB number in the one slice */;
pub const S5P_FIMV_ENC_MSLICE_BIT: c_uint = 0xc514 /* bit count for one slice */;
pub const S5P_FIMV_ENC_CIR_CTRL: c_uint = 0xc518 /* number of intra refresh MB */;
pub const S5P_FIMV_ENC_MAP_FOR_CUR: c_uint = 0xc51c /* linear or tiled mode */;
pub const S5P_FIMV_ENC_PADDING_CTRL: c_uint = 0xc520 /* padding control */;
pub const S5P_FIMV_ENC_RC_CONFIG: c_uint = 0xc5a0 /* RC config */;
pub const S5P_FIMV_ENC_RC_BIT_RATE: c_uint = 0xc5a8 /* bit rate */;
pub const S5P_FIMV_ENC_RC_QBOUND: c_uint = 0xc5ac /* max/min QP */;
pub const S5P_FIMV_ENC_RC_RPARA: c_uint = 0xc5b0 /* rate control reaction coeff */;
pub const S5P_FIMV_ENC_RC_MB_CTRL: c_uint = 0xc5b4 /* MB adaptive scaling */;
// Encoder for H264 only
pub const S5P_FIMV_ENC_H264_ENTROPY_MODE: c_uint = 0xd004 /* CAVLC or CABAC */;
pub const S5P_FIMV_ENC_H264_ALPHA_OFF: c_uint = 0xd008 /* loop filter alpha offset */;
pub const S5P_FIMV_ENC_H264_BETA_OFF: c_uint = 0xd00c /* loop filter beta offset */;
pub const S5P_FIMV_ENC_H264_NUM_OF_REF: c_uint = 0xd010 /* number of reference for P/B */;
pub const S5P_FIMV_ENC_H264_TRANS_FLAG: c_uint = 0xd034 /* 8x8 transform flag in PPS &;
pub const S5P_FIMV_ENC_RC_FRAME_RATE: c_uint = 0xd0d0 /* frame rate */;
// Encoder for MPEG4 only
pub const S5P_FIMV_ENC_MPEG4_QUART_PXL: c_uint = 0xe008 /* qpel interpolation ctrl */;
// Additional
pub const S5P_FIMV_SI_CH0_DPB_CONF_CTRL: c_uint = 0x2068 /* DPB Config Control Register */;
pub const S5P_FIMV_SLICE_INT_MASK: c_int = 1;
pub const S5P_FIMV_SLICE_INT_SHIFT: c_int = 31;
pub const S5P_FIMV_DDELAY_ENA_SHIFT: c_int = 30;
pub const S5P_FIMV_DDELAY_VAL_MASK: c_uint = 0xff;
pub const S5P_FIMV_DDELAY_VAL_SHIFT: c_int = 16;
pub const S5P_FIMV_DPB_COUNT_MASK: c_uint = 0xffff;
pub const S5P_FIMV_DPB_FLUSH_MASK: c_int = 1;
pub const S5P_FIMV_DPB_FLUSH_SHIFT: c_int = 14;
pub const S5P_FIMV_SI_CH0_RELEASE_BUF: c_uint = 0x2060 /* DPB release buffer register */;
pub const S5P_FIMV_SI_CH0_HOST_WR_ADR: c_uint = 0x2064 /* address of shared memory */;
// Codec numbers

pub const S5P_FIMV_CODEC_H264_DEC: c_int = 0;
pub const S5P_FIMV_CODEC_VC1_DEC: c_int = 1;
pub const S5P_FIMV_CODEC_MPEG4_DEC: c_int = 2;
pub const S5P_FIMV_CODEC_MPEG2_DEC: c_int = 3;
pub const S5P_FIMV_CODEC_H263_DEC: c_int = 4;
pub const S5P_FIMV_CODEC_VC1RCV_DEC: c_int = 5;
pub const S5P_FIMV_CODEC_H264_ENC: c_int = 16;
pub const S5P_FIMV_CODEC_MPEG4_ENC: c_int = 17;
pub const S5P_FIMV_CODEC_H263_ENC: c_int = 18;
// Channel Control Register
pub const S5P_FIMV_CH_SEQ_HEADER: c_int = 1;
pub const S5P_FIMV_CH_FRAME_START: c_int = 2;
pub const S5P_FIMV_CH_LAST_FRAME: c_int = 3;
pub const S5P_FIMV_CH_INIT_BUFS: c_int = 4;
pub const S5P_FIMV_CH_FRAME_START_REALLOC: c_int = 5;
pub const S5P_FIMV_CH_MASK: c_int = 7;
pub const S5P_FIMV_CH_SHIFT: c_int = 16;
// Host to RISC command
pub const S5P_FIMV_H2R_CMD_EMPTY: c_int = 0;
pub const S5P_FIMV_H2R_CMD_OPEN_INSTANCE: c_int = 1;
pub const S5P_FIMV_H2R_CMD_CLOSE_INSTANCE: c_int = 2;
pub const S5P_FIMV_H2R_CMD_SYS_INIT: c_int = 3;
pub const S5P_FIMV_H2R_CMD_FLUSH: c_int = 4;
pub const S5P_FIMV_H2R_CMD_SLEEP: c_int = 5;
pub const S5P_FIMV_H2R_CMD_WAKEUP: c_int = 6;
pub const S5P_FIMV_R2H_CMD_EMPTY: c_int = 0;
pub const S5P_FIMV_R2H_CMD_OPEN_INSTANCE_RET: c_int = 1;
pub const S5P_FIMV_R2H_CMD_CLOSE_INSTANCE_RET: c_int = 2;
pub const S5P_FIMV_R2H_CMD_RSV_RET: c_int = 3;
pub const S5P_FIMV_R2H_CMD_SEQ_DONE_RET: c_int = 4;
pub const S5P_FIMV_R2H_CMD_FRAME_DONE_RET: c_int = 5;
pub const S5P_FIMV_R2H_CMD_SLICE_DONE_RET: c_int = 6;
pub const S5P_FIMV_R2H_CMD_ENC_COMPLETE_RET: c_int = 7;
pub const S5P_FIMV_R2H_CMD_SYS_INIT_RET: c_int = 8;
pub const S5P_FIMV_R2H_CMD_FW_STATUS_RET: c_int = 9;
pub const S5P_FIMV_R2H_CMD_SLEEP_RET: c_int = 10;
pub const S5P_FIMV_R2H_CMD_WAKEUP_RET: c_int = 11;
pub const S5P_FIMV_R2H_CMD_FLUSH_RET: c_int = 12;
pub const S5P_FIMV_R2H_CMD_INIT_BUFFERS_RET: c_int = 15;
pub const S5P_FIMV_R2H_CMD_EDFU_INIT_RET: c_int = 16;
pub const S5P_FIMV_R2H_CMD_ERR_RET: c_int = 32;
// Dummy definition for MFCv6 compatibility

pub const S5P_FIMV_REG_CLEAR_BEGIN: c_int = 0;
pub const S5P_FIMV_REG_CLEAR_COUNT: c_int = 0;
// Error handling defines
pub const S5P_FIMV_ERR_NO_VALID_SEQ_HDR: c_int = 67;
pub const S5P_FIMV_ERR_INCOMPLETE_FRAME: c_int = 124;
pub const S5P_FIMV_ERR_TIMEOUT: c_int = 140;
pub const S5P_FIMV_ERR_WARNINGS_START: c_int = 145;
pub const S5P_FIMV_ERR_DEC_MASK: c_uint = 0xFFFF;
pub const S5P_FIMV_ERR_DEC_SHIFT: c_int = 0;
pub const S5P_FIMV_ERR_DSPL_MASK: c_uint = 0xFFFF0000;
pub const S5P_FIMV_ERR_DSPL_SHIFT: c_int = 16;
// Shared memory registers' offsets
// An offset of the start position in the stream when
// the start position is not aligned
pub const S5P_FIMV_SHARED_CROP_INFO_H: c_uint = 0x0020;
pub const S5P_FIMV_SHARED_CROP_LEFT_MASK: c_uint = 0xFFFF;
pub const S5P_FIMV_SHARED_CROP_LEFT_SHIFT: c_int = 0;
pub const S5P_FIMV_SHARED_CROP_RIGHT_MASK: c_uint = 0xFFFF0000;
pub const S5P_FIMV_SHARED_CROP_RIGHT_SHIFT: c_int = 16;
pub const S5P_FIMV_SHARED_CROP_INFO_V: c_uint = 0x0024;
pub const S5P_FIMV_SHARED_CROP_TOP_MASK: c_uint = 0xFFFF;
pub const S5P_FIMV_SHARED_CROP_TOP_SHIFT: c_int = 0;
pub const S5P_FIMV_SHARED_CROP_BOTTOM_MASK: c_uint = 0xFFFF0000;
pub const S5P_FIMV_SHARED_CROP_BOTTOM_SHIFT: c_int = 16;
pub const S5P_FIMV_SHARED_SET_FRAME_TAG: c_uint = 0x0004;
pub const S5P_FIMV_SHARED_GET_FRAME_TAG_TOP: c_uint = 0x0008;
pub const S5P_FIMV_SHARED_GET_FRAME_TAG_BOT: c_uint = 0x000C;
pub const S5P_FIMV_SHARED_START_BYTE_NUM: c_uint = 0x0018;
pub const S5P_FIMV_SHARED_RC_VOP_TIMING: c_uint = 0x0030;
pub const S5P_FIMV_SHARED_LUMA_DPB_SIZE: c_uint = 0x0064;
pub const S5P_FIMV_SHARED_CHROMA_DPB_SIZE: c_uint = 0x0068;
pub const S5P_FIMV_SHARED_MV_SIZE: c_uint = 0x006C;
pub const S5P_FIMV_SHARED_PIC_TIME_TOP: c_uint = 0x0010;
pub const S5P_FIMV_SHARED_PIC_TIME_BOTTOM: c_uint = 0x0014;
pub const S5P_FIMV_SHARED_EXT_ENC_CONTROL: c_uint = 0x0028;
pub const S5P_FIMV_SHARED_P_B_FRAME_QP: c_uint = 0x0070;
pub const S5P_FIMV_SHARED_ASPECT_RATIO_IDC: c_uint = 0x0074;
pub const S5P_FIMV_SHARED_EXTENDED_SAR: c_uint = 0x0078;
pub const S5P_FIMV_SHARED_H264_I_PERIOD: c_uint = 0x009C;
pub const S5P_FIMV_SHARED_RC_CONTROL_CONFIG: c_uint = 0x00A0;
pub const S5P_FIMV_SHARED_DISP_FRAME_TYPE_SHIFT: c_int = 2;
// Offset used by the hardware to store addresses
pub const MFC_OFFSET_SHIFT: c_int = 11;

pub const MFC_VERSION: c_uint = 0x51;
pub const MFC_NUM_PORTS: c_int = 2;
pub const S5P_FIMV_SHARED_FRAME_PACK_SEI_AVAIL: c_uint = 0x16C;
pub const S5P_FIMV_SHARED_FRAME_PACK_ARRGMENT_ID: c_uint = 0x170;
pub const S5P_FIMV_SHARED_FRAME_PACK_SEI_INFO: c_uint = 0x174;
pub const S5P_FIMV_SHARED_FRAME_PACK_GRID_POS: c_uint = 0x178;
// Values for resolution change in display status
pub const S5P_FIMV_RES_INCREASE: c_int = 1;
pub const S5P_FIMV_RES_DECREASE: c_int = 2;
