//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/samsung/s5p-mfc/regs-mfc-v6.h
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
// Register definition file for Samsung MFC V6.x Interface (FIMV) driver
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// Number of bits that the buffer address should be shifted for particular
// MFC buffers.
pub const S5P_FIMV_MEM_OFFSET_V6: c_int = 0;
pub const S5P_FIMV_START_ADDR_V6: c_uint = 0x0000;
pub const S5P_FIMV_END_ADDR_V6: c_uint = 0xfd80;
pub const S5P_FIMV_REG_CLEAR_BEGIN_V6: c_uint = 0xf000;
pub const S5P_FIMV_REG_CLEAR_COUNT_V6: c_int = 1024;
// Codec Common Registers
pub const S5P_FIMV_RISC_ON_V6: c_uint = 0x0000;
pub const S5P_FIMV_RISC2HOST_INT_V6: c_uint = 0x003C;
pub const S5P_FIMV_HOST2RISC_INT_V6: c_uint = 0x0044;
pub const S5P_FIMV_RISC_BASE_ADDRESS_V6: c_uint = 0x0054;
pub const S5P_FIMV_MFC_RESET_V6: c_uint = 0x1070;
pub const S5P_FIMV_HOST2RISC_CMD_V6: c_uint = 0x1100;
pub const S5P_FIMV_H2R_CMD_EMPTY_V6: c_int = 0;
pub const S5P_FIMV_H2R_CMD_SYS_INIT_V6: c_int = 1;
pub const S5P_FIMV_H2R_CMD_OPEN_INSTANCE_V6: c_int = 2;
pub const S5P_FIMV_CH_SEQ_HEADER_V6: c_int = 3;
pub const S5P_FIMV_CH_INIT_BUFS_V6: c_int = 4;
pub const S5P_FIMV_CH_FRAME_START_V6: c_int = 5;
pub const S5P_FIMV_H2R_CMD_CLOSE_INSTANCE_V6: c_int = 6;
pub const S5P_FIMV_H2R_CMD_SLEEP_V6: c_int = 7;
pub const S5P_FIMV_H2R_CMD_WAKEUP_V6: c_int = 8;
pub const S5P_FIMV_CH_LAST_FRAME_V6: c_int = 9;
pub const S5P_FIMV_H2R_CMD_FLUSH_V6: c_int = 10;
pub const S5P_FIMV_H2R_CMD_NAL_ABORT_V6: c_int = 11;
// RMVME: REALLOC used?
pub const S5P_FIMV_CH_FRAME_START_REALLOC_V6: c_int = 5;
pub const S5P_FIMV_RISC2HOST_CMD_V6: c_uint = 0x1104;
pub const S5P_FIMV_R2H_CMD_EMPTY_V6: c_int = 0;
pub const S5P_FIMV_R2H_CMD_SYS_INIT_RET_V6: c_int = 1;
pub const S5P_FIMV_R2H_CMD_OPEN_INSTANCE_RET_V6: c_int = 2;
pub const S5P_FIMV_R2H_CMD_SEQ_DONE_RET_V6: c_int = 3;
pub const S5P_FIMV_R2H_CMD_INIT_BUFFERS_RET_V6: c_int = 4;
pub const S5P_FIMV_R2H_CMD_CLOSE_INSTANCE_RET_V6: c_int = 6;
pub const S5P_FIMV_R2H_CMD_SLEEP_RET_V6: c_int = 7;
pub const S5P_FIMV_R2H_CMD_WAKEUP_RET_V6: c_int = 8;
pub const S5P_FIMV_R2H_CMD_COMPLETE_SEQ_RET_V6: c_int = 9;
pub const S5P_FIMV_R2H_CMD_DPB_FLUSH_RET_V6: c_int = 10;
pub const S5P_FIMV_R2H_CMD_NAL_ABORT_RET_V6: c_int = 11;
pub const S5P_FIMV_R2H_CMD_FW_STATUS_RET_V6: c_int = 12;
pub const S5P_FIMV_R2H_CMD_FRAME_DONE_RET_V6: c_int = 13;
pub const S5P_FIMV_R2H_CMD_FIELD_DONE_RET_V6: c_int = 14;
pub const S5P_FIMV_R2H_CMD_SLICE_DONE_RET_V6: c_int = 15;
pub const S5P_FIMV_R2H_CMD_ENC_BUFFER_FUL_RET_V6: c_int = 16;
pub const S5P_FIMV_R2H_CMD_ERR_RET_V6: c_int = 32;
pub const S5P_FIMV_MFC_BUS_RESET_CTRL: c_uint = 0x7110;
pub const S5P_FIMV_FW_VERSION_V6: c_uint = 0xf000;
pub const S5P_FIMV_INSTANCE_ID_V6: c_uint = 0xf008;
pub const S5P_FIMV_CODEC_TYPE_V6: c_uint = 0xf00c;
pub const S5P_FIMV_CONTEXT_MEM_ADDR_V6: c_uint = 0xf014;
pub const S5P_FIMV_CONTEXT_MEM_SIZE_V6: c_uint = 0xf018;
pub const S5P_FIMV_PIXEL_FORMAT_V6: c_uint = 0xf020;
pub const S5P_FIMV_METADATA_ENABLE_V6: c_uint = 0xf024;
pub const S5P_FIMV_DBG_BUFFER_ADDR_V6: c_uint = 0xf030;
pub const S5P_FIMV_DBG_BUFFER_SIZE_V6: c_uint = 0xf034;
pub const S5P_FIMV_RET_INSTANCE_ID_V6: c_uint = 0xf070;
pub const S5P_FIMV_ERROR_CODE_V6: c_uint = 0xf074;
pub const S5P_FIMV_ERR_WARNINGS_START_V6: c_int = 160;
pub const S5P_FIMV_ERR_DEC_MASK_V6: c_uint = 0xffff;
pub const S5P_FIMV_ERR_DEC_SHIFT_V6: c_int = 0;
pub const S5P_FIMV_ERR_DSPL_MASK_V6: c_uint = 0xffff0000;
pub const S5P_FIMV_ERR_DSPL_SHIFT_V6: c_int = 16;
pub const S5P_FIMV_DBG_BUFFER_OUTPUT_SIZE_V6: c_uint = 0xf078;
pub const S5P_FIMV_METADATA_STATUS_V6: c_uint = 0xf07C;
pub const S5P_FIMV_METADATA_ADDR_MB_INFO_V6: c_uint = 0xf080;
pub const S5P_FIMV_METADATA_SIZE_MB_INFO_V6: c_uint = 0xf084;
// Decoder Registers
pub const S5P_FIMV_D_CRC_CTRL_V6: c_uint = 0xf0b0;
pub const S5P_FIMV_D_DEC_OPTIONS_V6: c_uint = 0xf0b4;
pub const S5P_FIMV_D_OPT_FMO_ASO_CTRL_MASK_V6: c_int = 4;
pub const S5P_FIMV_D_OPT_DDELAY_EN_SHIFT_V6: c_int = 3;
pub const S5P_FIMV_D_OPT_LF_CTRL_SHIFT_V6: c_int = 1;
pub const S5P_FIMV_D_OPT_LF_CTRL_MASK_V6: c_uint = 0x3;
pub const S5P_FIMV_D_OPT_TILE_MODE_SHIFT_V6: c_int = 0;
pub const S5P_FIMV_D_DISPLAY_DELAY_V6: c_uint = 0xf0b8;
pub const S5P_FIMV_D_SET_FRAME_WIDTH_V6: c_uint = 0xf0bc;
pub const S5P_FIMV_D_SET_FRAME_HEIGHT_V6: c_uint = 0xf0c0;
pub const S5P_FIMV_D_SEI_ENABLE_V6: c_uint = 0xf0c4;
// Buffer setting registers
pub const S5P_FIMV_D_MIN_NUM_DPB_V6: c_uint = 0xf0f0;
pub const S5P_FIMV_D_MIN_LUMA_DPB_SIZE_V6: c_uint = 0xf0f4;
pub const S5P_FIMV_D_MIN_CHROMA_DPB_SIZE_V6: c_uint = 0xf0f8;
pub const S5P_FIMV_D_MVC_NUM_VIEWS_V6: c_uint = 0xf0fc;
pub const S5P_FIMV_D_MIN_NUM_MV_V6: c_uint = 0xf100;
pub const S5P_FIMV_D_NUM_DPB_V6: c_uint = 0xf130;
pub const S5P_FIMV_D_LUMA_DPB_SIZE_V6: c_uint = 0xf134;
pub const S5P_FIMV_D_CHROMA_DPB_SIZE_V6: c_uint = 0xf138;
pub const S5P_FIMV_D_MV_BUFFER_SIZE_V6: c_uint = 0xf13c;
pub const S5P_FIMV_D_LUMA_DPB_V6: c_uint = 0xf140;
pub const S5P_FIMV_D_CHROMA_DPB_V6: c_uint = 0xf240;
pub const S5P_FIMV_D_MV_BUFFER_V6: c_uint = 0xf340;
pub const S5P_FIMV_D_SCRATCH_BUFFER_ADDR_V6: c_uint = 0xf440;
pub const S5P_FIMV_D_SCRATCH_BUFFER_SIZE_V6: c_uint = 0xf444;
pub const S5P_FIMV_D_METADATA_BUFFER_ADDR_V6: c_uint = 0xf448;
pub const S5P_FIMV_D_METADATA_BUFFER_SIZE_V6: c_uint = 0xf44c;
pub const S5P_FIMV_D_NUM_MV_V6: c_uint = 0xf478;
pub const S5P_FIMV_D_CPB_BUFFER_ADDR_V6: c_uint = 0xf4b0;
pub const S5P_FIMV_D_CPB_BUFFER_SIZE_V6: c_uint = 0xf4b4;
pub const S5P_FIMV_D_AVAILABLE_DPB_FLAG_UPPER_V6: c_uint = 0xf4b8;
pub const S5P_FIMV_D_AVAILABLE_DPB_FLAG_LOWER_V6: c_uint = 0xf4bc;
pub const S5P_FIMV_D_CPB_BUFFER_OFFSET_V6: c_uint = 0xf4c0;
pub const S5P_FIMV_D_SLICE_IF_ENABLE_V6: c_uint = 0xf4c4;
pub const S5P_FIMV_D_PICTURE_TAG_V6: c_uint = 0xf4c8;
pub const S5P_FIMV_D_STREAM_DATA_SIZE_V6: c_uint = 0xf4d0;
pub const S5P_FIMV_D_INIT_BUFFER_OPTIONS_V6: c_uint = 0xf47c;
// Display information register
pub const S5P_FIMV_D_DISPLAY_FRAME_WIDTH_V6: c_uint = 0xf500;
pub const S5P_FIMV_D_DISPLAY_FRAME_HEIGHT_V6: c_uint = 0xf504;
// Display status
pub const S5P_FIMV_D_DISPLAY_STATUS_V6: c_uint = 0xf508;
pub const S5P_FIMV_D_DISPLAY_LUMA_ADDR_V6: c_uint = 0xf50c;
pub const S5P_FIMV_D_DISPLAY_CHROMA_ADDR_V6: c_uint = 0xf510;
pub const S5P_FIMV_D_DISPLAY_FRAME_TYPE_V6: c_uint = 0xf514;
pub const S5P_FIMV_D_DISPLAY_CROP_INFO1_V6: c_uint = 0xf518;
pub const S5P_FIMV_D_DISPLAY_CROP_INFO2_V6: c_uint = 0xf51c;
pub const S5P_FIMV_D_DISPLAY_PICTURE_PROFILE_V6: c_uint = 0xf520;
pub const S5P_FIMV_D_DISPLAY_LUMA_CRC_TOP_V6: c_uint = 0xf524;
pub const S5P_FIMV_D_DISPLAY_CHROMA_CRC_TOP_V6: c_uint = 0xf528;
pub const S5P_FIMV_D_DISPLAY_LUMA_CRC_BOT_V6: c_uint = 0xf52c;
pub const S5P_FIMV_D_DISPLAY_CHROMA_CRC_BOT_V6: c_uint = 0xf530;
pub const S5P_FIMV_D_DISPLAY_ASPECT_RATIO_V6: c_uint = 0xf534;
pub const S5P_FIMV_D_DISPLAY_EXTENDED_AR_V6: c_uint = 0xf538;
// Decoded picture information register
pub const S5P_FIMV_D_DECODED_FRAME_WIDTH_V6: c_uint = 0xf53c;
pub const S5P_FIMV_D_DECODED_FRAME_HEIGHT_V6: c_uint = 0xf540;
pub const S5P_FIMV_D_DECODED_STATUS_V6: c_uint = 0xf544;
pub const S5P_FIMV_DEC_CRC_GEN_MASK_V6: c_uint = 0x1;
pub const S5P_FIMV_DEC_CRC_GEN_SHIFT_V6: c_int = 6;
pub const S5P_FIMV_D_DECODED_LUMA_ADDR_V6: c_uint = 0xf548;
pub const S5P_FIMV_D_DECODED_CHROMA_ADDR_V6: c_uint = 0xf54c;
pub const S5P_FIMV_D_DECODED_FRAME_TYPE_V6: c_uint = 0xf550;
pub const S5P_FIMV_DECODE_FRAME_MASK_V6: c_int = 7;
pub const S5P_FIMV_D_DECODED_CROP_INFO1_V6: c_uint = 0xf554;
pub const S5P_FIMV_D_DECODED_CROP_INFO2_V6: c_uint = 0xf558;
pub const S5P_FIMV_D_DECODED_PICTURE_PROFILE_V6: c_uint = 0xf55c;
pub const S5P_FIMV_D_DECODED_NAL_SIZE_V6: c_uint = 0xf560;
pub const S5P_FIMV_D_DECODED_LUMA_CRC_TOP_V6: c_uint = 0xf564;
pub const S5P_FIMV_D_DECODED_CHROMA_CRC_TOP_V6: c_uint = 0xf568;
pub const S5P_FIMV_D_DECODED_LUMA_CRC_BOT_V6: c_uint = 0xf56c;
pub const S5P_FIMV_D_DECODED_CHROMA_CRC_BOT_V6: c_uint = 0xf570;
// Returned value register for specific setting
pub const S5P_FIMV_D_RET_PICTURE_TAG_TOP_V6: c_uint = 0xf574;
pub const S5P_FIMV_D_RET_PICTURE_TAG_BOT_V6: c_uint = 0xf578;
pub const S5P_FIMV_D_RET_PICTURE_TIME_TOP_V6: c_uint = 0xf57c;
pub const S5P_FIMV_D_RET_PICTURE_TIME_BOT_V6: c_uint = 0xf580;
pub const S5P_FIMV_D_CHROMA_FORMAT_V6: c_uint = 0xf588;
pub const S5P_FIMV_D_MPEG4_INFO_V6: c_uint = 0xf58c;
pub const S5P_FIMV_D_H264_INFO_V6: c_uint = 0xf590;
pub const S5P_FIMV_D_METADATA_ADDR_CONCEALED_MB_V6: c_uint = 0xf594;
pub const S5P_FIMV_D_METADATA_SIZE_CONCEALED_MB_V6: c_uint = 0xf598;
pub const S5P_FIMV_D_METADATA_ADDR_VC1_PARAM_V6: c_uint = 0xf59c;
pub const S5P_FIMV_D_METADATA_SIZE_VC1_PARAM_V6: c_uint = 0xf5a0;
pub const S5P_FIMV_D_METADATA_ADDR_SEI_NAL_V6: c_uint = 0xf5a4;
pub const S5P_FIMV_D_METADATA_SIZE_SEI_NAL_V6: c_uint = 0xf5a8;
pub const S5P_FIMV_D_METADATA_ADDR_VUI_V6: c_uint = 0xf5ac;
pub const S5P_FIMV_D_METADATA_SIZE_VUI_V6: c_uint = 0xf5b0;
pub const S5P_FIMV_D_MVC_VIEW_ID_V6: c_uint = 0xf5b4;
// SEI related information
pub const S5P_FIMV_D_FRAME_PACK_SEI_AVAIL_V6: c_uint = 0xf5f0;
pub const S5P_FIMV_D_FRAME_PACK_ARRGMENT_ID_V6: c_uint = 0xf5f4;
pub const S5P_FIMV_D_FRAME_PACK_SEI_INFO_V6: c_uint = 0xf5f8;
pub const S5P_FIMV_D_FRAME_PACK_GRID_POS_V6: c_uint = 0xf5fc;
// Encoder Registers
pub const S5P_FIMV_E_FRAME_WIDTH_V6: c_uint = 0xf770;
pub const S5P_FIMV_E_FRAME_HEIGHT_V6: c_uint = 0xf774;
pub const S5P_FIMV_E_CROPPED_FRAME_WIDTH_V6: c_uint = 0xf778;
pub const S5P_FIMV_E_CROPPED_FRAME_HEIGHT_V6: c_uint = 0xf77c;
pub const S5P_FIMV_E_FRAME_CROP_OFFSET_V6: c_uint = 0xf780;
pub const S5P_FIMV_E_ENC_OPTIONS_V6: c_uint = 0xf784;
pub const S5P_FIMV_E_PICTURE_PROFILE_V6: c_uint = 0xf788;
pub const S5P_FIMV_E_FIXED_PICTURE_QP_V6: c_uint = 0xf790;
pub const S5P_FIMV_E_RC_CONFIG_V6: c_uint = 0xf794;
pub const S5P_FIMV_E_RC_QP_BOUND_V6: c_uint = 0xf798;
pub const S5P_FIMV_E_RC_RPARAM_V6: c_uint = 0xf79c;
pub const S5P_FIMV_E_MB_RC_CONFIG_V6: c_uint = 0xf7a0;
pub const S5P_FIMV_E_PADDING_CTRL_V6: c_uint = 0xf7a4;
pub const S5P_FIMV_E_MV_HOR_RANGE_V6: c_uint = 0xf7ac;
pub const S5P_FIMV_E_MV_VER_RANGE_V6: c_uint = 0xf7b0;
pub const S5P_FIMV_E_MV_RANGE_V6_MASK: c_uint = 0x3fff;
pub const S5P_FIMV_E_VBV_BUFFER_SIZE_V6: c_uint = 0xf84c;
pub const S5P_FIMV_E_VBV_INIT_DELAY_V6: c_uint = 0xf850;
pub const S5P_FIMV_E_NUM_DPB_V6: c_uint = 0xf890;
pub const S5P_FIMV_E_LUMA_DPB_V6: c_uint = 0xf8c0;
pub const S5P_FIMV_E_CHROMA_DPB_V6: c_uint = 0xf904;
pub const S5P_FIMV_E_ME_BUFFER_V6: c_uint = 0xf948;
pub const S5P_FIMV_E_SCRATCH_BUFFER_ADDR_V6: c_uint = 0xf98c;
pub const S5P_FIMV_E_SCRATCH_BUFFER_SIZE_V6: c_uint = 0xf990;
pub const S5P_FIMV_E_TMV_BUFFER0_V6: c_uint = 0xf994;
pub const S5P_FIMV_E_TMV_BUFFER1_V6: c_uint = 0xf998;
pub const S5P_FIMV_E_SOURCE_LUMA_ADDR_V6: c_uint = 0xf9f0;
pub const S5P_FIMV_E_SOURCE_CHROMA_ADDR_V6: c_uint = 0xf9f4;
pub const S5P_FIMV_E_STREAM_BUFFER_ADDR_V6: c_uint = 0xf9f8;
pub const S5P_FIMV_E_STREAM_BUFFER_SIZE_V6: c_uint = 0xf9fc;
pub const S5P_FIMV_E_ROI_BUFFER_ADDR_V6: c_uint = 0xfA00;
pub const S5P_FIMV_E_PARAM_CHANGE_V6: c_uint = 0xfa04;
pub const S5P_FIMV_E_IR_SIZE_V6: c_uint = 0xfa08;
pub const S5P_FIMV_E_GOP_CONFIG_V6: c_uint = 0xfa0c;
pub const S5P_FIMV_E_MSLICE_MODE_V6: c_uint = 0xfa10;
pub const S5P_FIMV_E_MSLICE_SIZE_MB_V6: c_uint = 0xfa14;
pub const S5P_FIMV_E_MSLICE_SIZE_BITS_V6: c_uint = 0xfa18;
pub const S5P_FIMV_E_FRAME_INSERTION_V6: c_uint = 0xfa1c;
pub const S5P_FIMV_E_RC_FRAME_RATE_V6: c_uint = 0xfa20;
pub const S5P_FIMV_E_RC_BIT_RATE_V6: c_uint = 0xfa24;
pub const S5P_FIMV_E_RC_QP_OFFSET_V6: c_uint = 0xfa28;
pub const S5P_FIMV_E_RC_ROI_CTRL_V6: c_uint = 0xfa2c;
pub const S5P_FIMV_E_PICTURE_TAG_V6: c_uint = 0xfa30;
pub const S5P_FIMV_E_BIT_COUNT_ENABLE_V6: c_uint = 0xfa34;
pub const S5P_FIMV_E_MAX_BIT_COUNT_V6: c_uint = 0xfa38;
pub const S5P_FIMV_E_MIN_BIT_COUNT_V6: c_uint = 0xfa3c;
pub const S5P_FIMV_E_METADATA_BUFFER_ADDR_V6: c_uint = 0xfa40;
pub const S5P_FIMV_E_METADATA_BUFFER_SIZE_V6: c_uint = 0xfa44;
pub const S5P_FIMV_E_STREAM_SIZE_V6: c_uint = 0xfa80;
pub const S5P_FIMV_E_SLICE_TYPE_V6: c_uint = 0xfa84;
pub const S5P_FIMV_E_PICTURE_COUNT_V6: c_uint = 0xfa88;
pub const S5P_FIMV_E_RET_PICTURE_TAG_V6: c_uint = 0xfa8c;
pub const S5P_FIMV_E_STREAM_BUFFER_WRITE_POINTER_V6: c_uint = 0xfa90;
pub const S5P_FIMV_E_ENCODED_SOURCE_LUMA_ADDR_V6: c_uint = 0xfa94;
pub const S5P_FIMV_E_ENCODED_SOURCE_CHROMA_ADDR_V6: c_uint = 0xfa98;
pub const S5P_FIMV_E_RECON_LUMA_DPB_ADDR_V6: c_uint = 0xfa9c;
pub const S5P_FIMV_E_RECON_CHROMA_DPB_ADDR_V6: c_uint = 0xfaa0;
pub const S5P_FIMV_E_METADATA_ADDR_ENC_SLICE_V6: c_uint = 0xfaa4;
pub const S5P_FIMV_E_METADATA_SIZE_ENC_SLICE_V6: c_uint = 0xfaa8;
pub const S5P_FIMV_E_MPEG4_OPTIONS_V6: c_uint = 0xfb10;
pub const S5P_FIMV_E_MPEG4_HEC_PERIOD_V6: c_uint = 0xfb14;
pub const S5P_FIMV_E_ASPECT_RATIO_V6: c_uint = 0xfb50;
pub const S5P_FIMV_E_EXTENDED_SAR_V6: c_uint = 0xfb54;
pub const S5P_FIMV_E_H264_OPTIONS_V6: c_uint = 0xfb58;
pub const S5P_FIMV_E_H264_LF_ALPHA_OFFSET_V6: c_uint = 0xfb5c;
pub const S5P_FIMV_E_H264_LF_BETA_OFFSET_V6: c_uint = 0xfb60;
pub const S5P_FIMV_E_H264_I_PERIOD_V6: c_uint = 0xfb64;
pub const S5P_FIMV_E_H264_FMO_SLICE_GRP_MAP_TYPE_V6: c_uint = 0xfb68;
pub const S5P_FIMV_E_H264_FMO_NUM_SLICE_GRP_MINUS1_V6: c_uint = 0xfb6c;
pub const S5P_FIMV_E_H264_FMO_SLICE_GRP_CHANGE_DIR_V6: c_uint = 0xfb70;
pub const S5P_FIMV_E_H264_FMO_SLICE_GRP_CHANGE_RATE_MINUS1_V6: c_uint = 0xfb74;
pub const S5P_FIMV_E_H264_FMO_RUN_LENGTH_MINUS1_0_V6: c_uint = 0xfb78;
pub const S5P_FIMV_E_H264_FMO_RUN_LENGTH_MINUS1_1_V6: c_uint = 0xfb7c;
pub const S5P_FIMV_E_H264_FMO_RUN_LENGTH_MINUS1_2_V6: c_uint = 0xfb80;
pub const S5P_FIMV_E_H264_FMO_RUN_LENGTH_MINUS1_3_V6: c_uint = 0xfb84;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_0_V6: c_uint = 0xfb88;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_1_V6: c_uint = 0xfb8c;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_2_V6: c_uint = 0xfb90;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_3_V6: c_uint = 0xfb94;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_4_V6: c_uint = 0xfb98;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_5_V6: c_uint = 0xfb9c;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_6_V6: c_uint = 0xfba0;
pub const S5P_FIMV_E_H264_ASO_SLICE_ORDER_7_V6: c_uint = 0xfba4;
pub const S5P_FIMV_E_H264_CHROMA_QP_OFFSET_V6: c_uint = 0xfba8;
pub const S5P_FIMV_E_H264_NUM_T_LAYER_V6: c_uint = 0xfbac;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER0_V6: c_uint = 0xfbb0;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER1_V6: c_uint = 0xfbb4;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER2_V6: c_uint = 0xfbb8;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER3_V6: c_uint = 0xfbbc;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER4_V6: c_uint = 0xfbc0;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER5_V6: c_uint = 0xfbc4;
pub const S5P_FIMV_E_H264_HIERARCHICAL_QP_LAYER6_V6: c_uint = 0xfbc8;
pub const S5P_FIMV_E_H264_FRAME_PACKING_SEI_INFO_V6: c_uint = 0xfc4c;
pub const S5P_FIMV_ENC_FP_ARRANGEMENT_TYPE_SIDE_BY_SIDE_V6: c_int = 0;
pub const S5P_FIMV_ENC_FP_ARRANGEMENT_TYPE_TOP_BOTTOM_V6: c_int = 1;
pub const S5P_FIMV_ENC_FP_ARRANGEMENT_TYPE_TEMPORAL_V6: c_int = 2;
pub const S5P_FIMV_E_MVC_FRAME_QP_VIEW1_V6: c_uint = 0xfd40;
pub const S5P_FIMV_E_MVC_RC_FRAME_RATE_VIEW1_V6: c_uint = 0xfd44;
pub const S5P_FIMV_E_MVC_RC_BIT_RATE_VIEW1_V6: c_uint = 0xfd48;
pub const S5P_FIMV_E_MVC_RC_QBOUND_VIEW1_V6: c_uint = 0xfd4c;
pub const S5P_FIMV_E_MVC_RC_RPARA_VIEW1_V6: c_uint = 0xfd50;
pub const S5P_FIMV_E_MVC_INTER_VIEW_PREDICTION_ON_V6: c_uint = 0xfd80;
// Codec numbers

pub const S5P_FIMV_CODEC_H264_DEC_V6: c_int = 0;
pub const S5P_FIMV_CODEC_H264_MVC_DEC_V6: c_int = 1;
pub const S5P_FIMV_CODEC_MPEG4_DEC_V6: c_int = 3;
pub const S5P_FIMV_CODEC_FIMV1_DEC_V6: c_int = 4;
pub const S5P_FIMV_CODEC_FIMV2_DEC_V6: c_int = 5;
pub const S5P_FIMV_CODEC_FIMV3_DEC_V6: c_int = 6;
pub const S5P_FIMV_CODEC_FIMV4_DEC_V6: c_int = 7;
pub const S5P_FIMV_CODEC_H263_DEC_V6: c_int = 8;
pub const S5P_FIMV_CODEC_VC1RCV_DEC_V6: c_int = 9;
pub const S5P_FIMV_CODEC_VC1_DEC_V6: c_int = 10;
// FIXME: Add 11~12
pub const S5P_FIMV_CODEC_MPEG2_DEC_V6: c_int = 13;
pub const S5P_FIMV_CODEC_VP8_DEC_V6: c_int = 14;
// FIXME: Add 15~16
pub const S5P_FIMV_CODEC_H264_ENC_V6: c_int = 20;
pub const S5P_FIMV_CODEC_H264_MVC_ENC_V6: c_int = 21;
pub const S5P_FIMV_CODEC_MPEG4_ENC_V6: c_int = 23;
pub const S5P_FIMV_CODEC_H263_ENC_V6: c_int = 24;
pub const S5P_FIMV_NV12M_HALIGN_V6: c_int = 16;
pub const S5P_FIMV_NV12MT_HALIGN_V6: c_int = 16;
pub const S5P_FIMV_NV12MT_VALIGN_V6: c_int = 16;
pub const S5P_FIMV_TMV_BUFFER_ALIGN_V6: c_int = 16;
pub const S5P_FIMV_LUMA_DPB_BUFFER_ALIGN_V6: c_int = 256;
pub const S5P_FIMV_CHROMA_DPB_BUFFER_ALIGN_V6: c_int = 256;
pub const S5P_FIMV_ME_BUFFER_ALIGN_V6: c_int = 256;
pub const S5P_FIMV_SCRATCH_BUFFER_ALIGN_V6: c_int = 256;
pub const S5P_FIMV_LUMA_MB_TO_PIXEL_V6: c_int = 256;
pub const S5P_FIMV_CHROMA_MB_TO_PIXEL_V6: c_int = 128;
pub const S5P_FIMV_NUM_TMV_BUFFERS_V6: c_int = 2;

pub const S5P_FIMV_NUM_PIXELS_IN_MB_ROW_V6: c_int = 16;
pub const S5P_FIMV_NUM_PIXELS_IN_MB_COL_V6: c_int = 16;
// Buffer size requirements defined by hardware

// MFC Context buffer sizes

// MFCv6 variant defines

pub const MFC_VERSION_V6: c_uint = 0x61;
pub const MFC_NUM_PORTS_V6: c_int = 1;
