//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/venus/hfi_helper.h
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
// Copyright (c) 2012-2016, The Linux Foundation. All rights reserved.
// Copyright (C) 2017 Linaro Ltd.
//
pub const HFI_DOMAIN_BASE_COMMON: c_int = 0;
pub const HFI_DOMAIN_BASE_VDEC: c_uint = 0x1000000;
pub const HFI_DOMAIN_BASE_VENC: c_uint = 0x2000000;
pub const HFI_DOMAIN_BASE_VPE: c_uint = 0x3000000;
pub const HFI_VIDEO_ARCH_OX: c_uint = 0x1;
pub const HFI_ARCH_COMMON_OFFSET: c_int = 0;
pub const HFI_ARCH_OX_OFFSET: c_uint = 0x200000;
pub const HFI_OX_BASE: c_uint = 0x1000000;
pub const HFI_CMD_START_OFFSET: c_uint = 0x10000;
pub const HFI_MSG_START_OFFSET: c_uint = 0x20000;
pub const HFI_ERR_NONE: c_uint = 0x0;
pub const HFI_ERR_SYS_FATAL: c_uint = 0x1;
pub const HFI_ERR_SYS_INVALID_PARAMETER: c_uint = 0x2;
pub const HFI_ERR_SYS_VERSION_MISMATCH: c_uint = 0x3;
pub const HFI_ERR_SYS_INSUFFICIENT_RESOURCES: c_uint = 0x4;
pub const HFI_ERR_SYS_MAX_SESSIONS_REACHED: c_uint = 0x5;
pub const HFI_ERR_SYS_UNSUPPORTED_CODEC: c_uint = 0x6;
pub const HFI_ERR_SYS_SESSION_IN_USE: c_uint = 0x7;
pub const HFI_ERR_SYS_SESSION_ID_OUT_OF_RANGE: c_uint = 0x8;
pub const HFI_ERR_SYS_UNSUPPORTED_DOMAIN: c_uint = 0x9;
pub const HFI_ERR_SESSION_FATAL: c_uint = 0x1001;
pub const HFI_ERR_SESSION_INVALID_PARAMETER: c_uint = 0x1002;
pub const HFI_ERR_SESSION_BAD_POINTER: c_uint = 0x1003;
pub const HFI_ERR_SESSION_INVALID_SESSION_ID: c_uint = 0x1004;
pub const HFI_ERR_SESSION_INVALID_STREAM_ID: c_uint = 0x1005;
pub const HFI_ERR_SESSION_INCORRECT_STATE_OPERATION: c_uint = 0x1006;
pub const HFI_ERR_SESSION_UNSUPPORTED_PROPERTY: c_uint = 0x1007;
pub const HFI_ERR_SESSION_UNSUPPORTED_SETTING: c_uint = 0x1008;
pub const HFI_ERR_SESSION_INSUFFICIENT_RESOURCES: c_uint = 0x1009;
pub const HFI_ERR_SESSION_STREAM_CORRUPT_OUTPUT_STALLED: c_uint = 0x100a;
pub const HFI_ERR_SESSION_STREAM_CORRUPT: c_uint = 0x100b;
pub const HFI_ERR_SESSION_ENC_OVERFLOW: c_uint = 0x100c;
pub const HFI_ERR_SESSION_UNSUPPORTED_STREAM: c_uint = 0x100d;
pub const HFI_ERR_SESSION_CMDSIZE: c_uint = 0x100e;
pub const HFI_ERR_SESSION_UNSUPPORT_CMD: c_uint = 0x100f;
pub const HFI_ERR_SESSION_UNSUPPORT_BUFFERTYPE: c_uint = 0x1010;
pub const HFI_ERR_SESSION_BUFFERCOUNT_TOOSMALL: c_uint = 0x1011;
pub const HFI_ERR_SESSION_INVALID_SCALE_FACTOR: c_uint = 0x1012;
pub const HFI_ERR_SESSION_UPSCALE_NOT_SUPPORTED: c_uint = 0x1013;
pub const HFI_EVENT_SYS_ERROR: c_uint = 0x1;
pub const HFI_EVENT_SESSION_ERROR: c_uint = 0x2;
pub const HFI_EVENT_DATA_SEQUENCE_CHANGED_SUFFICIENT_BUF_RESOURCES: c_uint = 0x1000001;
pub const HFI_EVENT_DATA_SEQUENCE_CHANGED_INSUFFICIENT_BUF_RESOURCES: c_uint = 0x1000002;
pub const HFI_EVENT_SESSION_SEQUENCE_CHANGED: c_uint = 0x1000003;
pub const HFI_EVENT_SESSION_PROPERTY_CHANGED: c_uint = 0x1000004;
pub const HFI_EVENT_SESSION_LTRUSE_FAILED: c_uint = 0x1000005;
pub const HFI_EVENT_RELEASE_BUFFER_REFERENCE: c_uint = 0x1000006;
pub const HFI_BUFFERFLAG_EOS: c_uint = 0x00000001;
pub const HFI_BUFFERFLAG_STARTTIME: c_uint = 0x00000002;
pub const HFI_BUFFERFLAG_DECODEONLY: c_uint = 0x00000004;
pub const HFI_BUFFERFLAG_DATACORRUPT: c_uint = 0x00000008;
pub const HFI_BUFFERFLAG_ENDOFFRAME: c_uint = 0x00000010;
pub const HFI_BUFFERFLAG_SYNCFRAME: c_uint = 0x00000020;
pub const HFI_BUFFERFLAG_EXTRADATA: c_uint = 0x00000040;
pub const HFI_BUFFERFLAG_CODECCONFIG: c_uint = 0x00000080;
pub const HFI_BUFFERFLAG_TIMESTAMPINVALID: c_uint = 0x00000100;
pub const HFI_BUFFERFLAG_READONLY: c_uint = 0x00000200;
pub const HFI_BUFFERFLAG_ENDOFSUBFRAME: c_uint = 0x00000400;
pub const HFI_BUFFERFLAG_EOSEQ: c_uint = 0x00200000;
pub const HFI_BUFFERFLAG_MBAFF: c_uint = 0x08000000;
pub const HFI_BUFFERFLAG_VPE_YUV_601_709_CSC_CLAMP: c_uint = 0x10000000;
pub const HFI_BUFFERFLAG_DROP_FRAME: c_uint = 0x20000000;
pub const HFI_BUFFERFLAG_TEI: c_uint = 0x40000000;
pub const HFI_BUFFERFLAG_DISCONTINUITY: c_uint = 0x80000000;
pub const HFI_ERR_SESSION_EMPTY_BUFFER_DONE_OUTPUT_PENDING: c_uint = 0x1001001;
pub const HFI_ERR_SESSION_SAME_STATE_OPERATION: c_uint = 0x1001002;
pub const HFI_ERR_SESSION_SYNC_FRAME_NOT_DETECTED: c_uint = 0x1001003;
pub const HFI_ERR_SESSION_START_CODE_NOT_FOUND: c_uint = 0x1001004;
pub const HFI_FLUSH_INPUT: c_uint = 0x1000001;
pub const HFI_FLUSH_OUTPUT: c_uint = 0x1000002;
pub const HFI_FLUSH_OUTPUT2: c_uint = 0x1000003;
pub const HFI_FLUSH_ALL: c_uint = 0x1000004;
pub const HFI_EXTRADATA_NONE: c_uint = 0x00000000;
pub const HFI_EXTRADATA_MB_QUANTIZATION: c_uint = 0x00000001;
pub const HFI_EXTRADATA_INTERLACE_VIDEO: c_uint = 0x00000002;
pub const HFI_EXTRADATA_VC1_FRAMEDISP: c_uint = 0x00000003;
pub const HFI_EXTRADATA_VC1_SEQDISP: c_uint = 0x00000004;
pub const HFI_EXTRADATA_TIMESTAMP: c_uint = 0x00000005;
pub const HFI_EXTRADATA_S3D_FRAME_PACKING: c_uint = 0x00000006;
pub const HFI_EXTRADATA_FRAME_RATE: c_uint = 0x00000007;
pub const HFI_EXTRADATA_PANSCAN_WINDOW: c_uint = 0x00000008;
pub const HFI_EXTRADATA_RECOVERY_POINT_SEI: c_uint = 0x00000009;
pub const HFI_EXTRADATA_MPEG2_SEQDISP: c_uint = 0x0000000d;
pub const HFI_EXTRADATA_STREAM_USERDATA: c_uint = 0x0000000e;
pub const HFI_EXTRADATA_FRAME_QP: c_uint = 0x0000000f;
pub const HFI_EXTRADATA_FRAME_BITS_INFO: c_uint = 0x00000010;
pub const HFI_EXTRADATA_MULTISLICE_INFO: c_uint = 0x7f100000;
pub const HFI_EXTRADATA_NUM_CONCEALED_MB: c_uint = 0x7f100001;
pub const HFI_EXTRADATA_INDEX: c_uint = 0x7f100002;
pub const HFI_EXTRADATA_METADATA_LTR: c_uint = 0x7f100004;
pub const HFI_EXTRADATA_METADATA_FILLER: c_uint = 0x7fe00002;
pub const HFI_INDEX_EXTRADATA_INPUT_CROP: c_uint = 0x0700000e;
pub const HFI_INDEX_EXTRADATA_OUTPUT_CROP: c_uint = 0x0700000f;
pub const HFI_INDEX_EXTRADATA_DIGITAL_ZOOM: c_uint = 0x07000010;
pub const HFI_INDEX_EXTRADATA_ASPECT_RATIO: c_uint = 0x7f100003;
pub const HFI_INTERLACE_FRAME_PROGRESSIVE: c_uint = 0x01;
pub const HFI_INTERLACE_INTERLEAVE_FRAME_TOPFIELDFIRST: c_uint = 0x02;
pub const HFI_INTERLACE_INTERLEAVE_FRAME_BOTTOMFIELDFIRST: c_uint = 0x04;
pub const HFI_INTERLACE_FRAME_TOPFIELDFIRST: c_uint = 0x08;
pub const HFI_INTERLACE_FRAME_BOTTOMFIELDFIRST: c_uint = 0x10;
//
// HFI_PROPERTY_PARAM_OX_START
// HFI_DOMAIN_BASE_COMMON + HFI_ARCH_OX_OFFSET + 0x1000
//
pub const HFI_PROPERTY_PARAM_BUFFER_COUNT_ACTUAL: c_uint = 0x201001;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_PLANE_ACTUAL_CONSTRAINTS_INFO: c_uint = 0x201002;
pub const HFI_PROPERTY_PARAM_INTERLACE_FORMAT_SUPPORTED: c_uint = 0x201003;
pub const HFI_PROPERTY_PARAM_CHROMA_SITE: c_uint = 0x201004;
pub const HFI_PROPERTY_PARAM_EXTRA_DATA_HEADER_CONFIG: c_uint = 0x201005;
pub const HFI_PROPERTY_PARAM_INDEX_EXTRADATA: c_uint = 0x201006;
pub const HFI_PROPERTY_PARAM_DIVX_FORMAT: c_uint = 0x201007;
pub const HFI_PROPERTY_PARAM_BUFFER_ALLOC_MODE: c_uint = 0x201008;
pub const HFI_PROPERTY_PARAM_S3D_FRAME_PACKING_EXTRADATA: c_uint = 0x201009;
pub const HFI_PROPERTY_PARAM_ERR_DETECTION_CODE_EXTRADATA: c_uint = 0x20100a;
pub const HFI_PROPERTY_PARAM_BUFFER_ALLOC_MODE_SUPPORTED: c_uint = 0x20100b;
pub const HFI_PROPERTY_PARAM_BUFFER_SIZE_ACTUAL: c_uint = 0x20100c;
pub const HFI_PROPERTY_PARAM_BUFFER_DISPLAY_HOLD_COUNT_ACTUAL: c_uint = 0x20100d;
//
// HFI_PROPERTY_CONFIG_OX_START
// HFI_DOMAIN_BASE_COMMON + HFI_ARCH_OX_OFFSET + 0x2000
//
pub const HFI_PROPERTY_CONFIG_BUFFER_REQUIREMENTS: c_uint = 0x202001;
pub const HFI_PROPERTY_CONFIG_REALTIME: c_uint = 0x202002;
pub const HFI_PROPERTY_CONFIG_PRIORITY: c_uint = 0x202003;
pub const HFI_PROPERTY_CONFIG_BATCH_INFO: c_uint = 0x202004;
//
// HFI_PROPERTY_PARAM_VDEC_OX_START	\
// HFI_DOMAIN_BASE_VDEC + HFI_ARCH_OX_OFFSET + 0x3000
//
pub const HFI_PROPERTY_PARAM_VDEC_CONTINUE_DATA_TRANSFER: c_uint = 0x1203001;
pub const HFI_PROPERTY_PARAM_VDEC_DISPLAY_PICTURE_BUFFER_COUNT: c_uint = 0x1203002;
pub const HFI_PROPERTY_PARAM_VDEC_MULTI_VIEW_SELECT: c_uint = 0x1203003;
pub const HFI_PROPERTY_PARAM_VDEC_PICTURE_TYPE_DECODE: c_uint = 0x1203004;
pub const HFI_PROPERTY_PARAM_VDEC_OUTPUT_ORDER: c_uint = 0x1203005;
pub const HFI_PROPERTY_PARAM_VDEC_MB_QUANTIZATION: c_uint = 0x1203006;
pub const HFI_PROPERTY_PARAM_VDEC_NUM_CONCEALED_MB: c_uint = 0x1203007;
pub const HFI_PROPERTY_PARAM_VDEC_H264_ENTROPY_SWITCHING: c_uint = 0x1203008;
pub const HFI_PROPERTY_PARAM_VDEC_OUTPUT2_KEEP_ASPECT_RATIO: c_uint = 0x1203009;
pub const HFI_PROPERTY_PARAM_VDEC_FRAME_RATE_EXTRADATA: c_uint = 0x120300a;
pub const HFI_PROPERTY_PARAM_VDEC_PANSCAN_WNDW_EXTRADATA: c_uint = 0x120300b;
pub const HFI_PROPERTY_PARAM_VDEC_RECOVERY_POINT_SEI_EXTRADATA: c_uint = 0x120300c;
pub const HFI_PROPERTY_PARAM_VDEC_THUMBNAIL_MODE: c_uint = 0x120300d;
pub const HFI_PROPERTY_PARAM_VDEC_FRAME_ASSEMBLY: c_uint = 0x120300e;
pub const HFI_PROPERTY_PARAM_VDEC_DPB_COUNTS: c_uint = 0x120300e;
pub const HFI_PROPERTY_PARAM_VDEC_VC1_FRAMEDISP_EXTRADATA: c_uint = 0x1203011;
pub const HFI_PROPERTY_PARAM_VDEC_VC1_SEQDISP_EXTRADATA: c_uint = 0x1203012;
pub const HFI_PROPERTY_PARAM_VDEC_TIMESTAMP_EXTRADATA: c_uint = 0x1203013;
pub const HFI_PROPERTY_PARAM_VDEC_INTERLACE_VIDEO_EXTRADATA: c_uint = 0x1203014;
pub const HFI_PROPERTY_PARAM_VDEC_AVC_SESSION_SELECT: c_uint = 0x1203015;
pub const HFI_PROPERTY_PARAM_VDEC_MPEG2_SEQDISP_EXTRADATA: c_uint = 0x1203016;
pub const HFI_PROPERTY_PARAM_VDEC_STREAM_USERDATA_EXTRADATA: c_uint = 0x1203017;
pub const HFI_PROPERTY_PARAM_VDEC_FRAME_QP_EXTRADATA: c_uint = 0x1203018;
pub const HFI_PROPERTY_PARAM_VDEC_FRAME_BITS_INFO_EXTRADATA: c_uint = 0x1203019;
pub const HFI_PROPERTY_PARAM_VDEC_SCS_THRESHOLD: c_uint = 0x120301a;
//
// HFI_PROPERTY_CONFIG_VDEC_OX_START
// HFI_DOMAIN_BASE_VDEC + HFI_ARCH_OX_OFFSET + 0x0000
//
pub const HFI_PROPERTY_CONFIG_VDEC_POST_LOOP_DEBLOCKER: c_uint = 0x1200001;
pub const HFI_PROPERTY_CONFIG_VDEC_MB_ERROR_MAP_REPORTING: c_uint = 0x1200002;
pub const HFI_PROPERTY_CONFIG_VDEC_MB_ERROR_MAP: c_uint = 0x1200003;
pub const HFI_PROPERTY_CONFIG_VDEC_ENTROPY: c_uint = 0x1204004;
//
// HFI_PROPERTY_PARAM_VENC_OX_START
// HFI_DOMAIN_BASE_VENC + HFI_ARCH_OX_OFFSET + 0x5000
//
pub const HFI_PROPERTY_PARAM_VENC_MULTI_SLICE_INFO: c_uint = 0x2205001;
pub const HFI_PROPERTY_PARAM_VENC_H264_IDR_S3D_FRAME_PACKING_NAL: c_uint = 0x2205002;
pub const HFI_PROPERTY_PARAM_VENC_LTR_INFO: c_uint = 0x2205003;
pub const HFI_PROPERTY_PARAM_VENC_MBI_DUMPING: c_uint = 0x2205005;
//
// HFI_PROPERTY_CONFIG_VENC_OX_START
// HFI_DOMAIN_BASE_VENC + HFI_ARCH_OX_OFFSET + 0x6000
//
pub const HFI_PROPERTY_CONFIG_VENC_FRAME_QP: c_uint = 0x2206001;
//
// HFI_PROPERTY_PARAM_VPE_OX_START
// HFI_DOMAIN_BASE_VPE + HFI_ARCH_OX_OFFSET + 0x7000
//
pub const HFI_PROPERTY_PARAM_VPE_COLOR_SPACE_CONVERSION: c_uint = 0x3207001;

pub const HFI_CHROMA_SITE_0: c_uint = 0x1000001;
pub const HFI_CHROMA_SITE_1: c_uint = 0x1000002;
pub const HFI_CHROMA_SITE_2: c_uint = 0x1000003;
pub const HFI_CHROMA_SITE_3: c_uint = 0x1000004;
pub const HFI_CHROMA_SITE_4: c_uint = 0x1000005;
pub const HFI_CHROMA_SITE_5: c_uint = 0x1000006;
pub const HFI_PRIORITY_LOW: c_int = 10;
pub const HFI_PRIOIRTY_MEDIUM: c_int = 20;
pub const HFI_PRIORITY_HIGH: c_int = 30;
pub const HFI_OUTPUT_ORDER_DISPLAY: c_uint = 0x1000001;
pub const HFI_OUTPUT_ORDER_DECODE: c_uint = 0x1000002;
pub const HFI_RATE_CONTROL_OFF: c_uint = 0x1000001;
pub const HFI_RATE_CONTROL_VBR_VFR: c_uint = 0x1000002;
pub const HFI_RATE_CONTROL_VBR_CFR: c_uint = 0x1000003;
pub const HFI_RATE_CONTROL_CBR_VFR: c_uint = 0x1000004;
pub const HFI_RATE_CONTROL_CBR_CFR: c_uint = 0x1000005;
pub const HFI_RATE_CONTROL_CQ: c_uint = 0x1000008;
pub const HFI_VIDEO_CODEC_H264: c_uint = 0x00000002;
pub const HFI_VIDEO_CODEC_H263: c_uint = 0x00000004;
pub const HFI_VIDEO_CODEC_MPEG1: c_uint = 0x00000008;
pub const HFI_VIDEO_CODEC_MPEG2: c_uint = 0x00000010;
pub const HFI_VIDEO_CODEC_MPEG4: c_uint = 0x00000020;
pub const HFI_VIDEO_CODEC_DIVX_311: c_uint = 0x00000040;
pub const HFI_VIDEO_CODEC_DIVX: c_uint = 0x00000080;
pub const HFI_VIDEO_CODEC_VC1: c_uint = 0x00000100;
pub const HFI_VIDEO_CODEC_SPARK: c_uint = 0x00000200;
pub const HFI_VIDEO_CODEC_VP8: c_uint = 0x00001000;
pub const HFI_VIDEO_CODEC_HEVC: c_uint = 0x00002000;
pub const HFI_VIDEO_CODEC_VP9: c_uint = 0x00004000;
pub const HFI_VIDEO_CODEC_HEVC_HYBRID: c_uint = 0x80000000;
pub const HFI_H264_PROFILE_BASELINE: c_uint = 0x00000001;
pub const HFI_H264_PROFILE_MAIN: c_uint = 0x00000002;
pub const HFI_H264_PROFILE_HIGH: c_uint = 0x00000004;
pub const HFI_H264_PROFILE_STEREO_HIGH: c_uint = 0x00000008;
pub const HFI_H264_PROFILE_MULTIVIEW_HIGH: c_uint = 0x00000010;
pub const HFI_H264_PROFILE_CONSTRAINED_BASE: c_uint = 0x00000020;
pub const HFI_H264_PROFILE_CONSTRAINED_HIGH: c_uint = 0x00000040;
pub const HFI_H264_LEVEL_1: c_uint = 0x00000001;
pub const HFI_H264_LEVEL_1b: c_uint = 0x00000002;
pub const HFI_H264_LEVEL_11: c_uint = 0x00000004;
pub const HFI_H264_LEVEL_12: c_uint = 0x00000008;
pub const HFI_H264_LEVEL_13: c_uint = 0x00000010;
pub const HFI_H264_LEVEL_2: c_uint = 0x00000020;
pub const HFI_H264_LEVEL_21: c_uint = 0x00000040;
pub const HFI_H264_LEVEL_22: c_uint = 0x00000080;
pub const HFI_H264_LEVEL_3: c_uint = 0x00000100;
pub const HFI_H264_LEVEL_31: c_uint = 0x00000200;
pub const HFI_H264_LEVEL_32: c_uint = 0x00000400;
pub const HFI_H264_LEVEL_4: c_uint = 0x00000800;
pub const HFI_H264_LEVEL_41: c_uint = 0x00001000;
pub const HFI_H264_LEVEL_42: c_uint = 0x00002000;
pub const HFI_H264_LEVEL_5: c_uint = 0x00004000;
pub const HFI_H264_LEVEL_51: c_uint = 0x00008000;
pub const HFI_H264_LEVEL_52: c_uint = 0x00010000;
pub const HFI_H263_PROFILE_BASELINE: c_uint = 0x00000001;
pub const HFI_H263_LEVEL_10: c_uint = 0x00000001;
pub const HFI_H263_LEVEL_20: c_uint = 0x00000002;
pub const HFI_H263_LEVEL_30: c_uint = 0x00000004;
pub const HFI_H263_LEVEL_40: c_uint = 0x00000008;
pub const HFI_H263_LEVEL_45: c_uint = 0x00000010;
pub const HFI_H263_LEVEL_50: c_uint = 0x00000020;
pub const HFI_H263_LEVEL_60: c_uint = 0x00000040;
pub const HFI_H263_LEVEL_70: c_uint = 0x00000080;
pub const HFI_MPEG2_PROFILE_SIMPLE: c_uint = 0x00000001;
pub const HFI_MPEG2_PROFILE_MAIN: c_uint = 0x00000002;
pub const HFI_MPEG2_PROFILE_422: c_uint = 0x00000004;
pub const HFI_MPEG2_PROFILE_SNR: c_uint = 0x00000008;
pub const HFI_MPEG2_PROFILE_SPATIAL: c_uint = 0x00000010;
pub const HFI_MPEG2_PROFILE_HIGH: c_uint = 0x00000020;
pub const HFI_MPEG2_LEVEL_LL: c_uint = 0x00000001;
pub const HFI_MPEG2_LEVEL_ML: c_uint = 0x00000002;
pub const HFI_MPEG2_LEVEL_H14: c_uint = 0x00000004;
pub const HFI_MPEG2_LEVEL_HL: c_uint = 0x00000008;
pub const HFI_MPEG4_PROFILE_SIMPLE: c_uint = 0x00000001;
pub const HFI_MPEG4_PROFILE_ADVANCEDSIMPLE: c_uint = 0x00000002;
pub const HFI_MPEG4_LEVEL_0: c_uint = 0x00000001;
pub const HFI_MPEG4_LEVEL_0b: c_uint = 0x00000002;
pub const HFI_MPEG4_LEVEL_1: c_uint = 0x00000004;
pub const HFI_MPEG4_LEVEL_2: c_uint = 0x00000008;
pub const HFI_MPEG4_LEVEL_3: c_uint = 0x00000010;
pub const HFI_MPEG4_LEVEL_4: c_uint = 0x00000020;
pub const HFI_MPEG4_LEVEL_4a: c_uint = 0x00000040;
pub const HFI_MPEG4_LEVEL_5: c_uint = 0x00000080;
pub const HFI_MPEG4_LEVEL_6: c_uint = 0x00000100;
pub const HFI_MPEG4_LEVEL_7: c_uint = 0x00000200;
pub const HFI_MPEG4_LEVEL_8: c_uint = 0x00000400;
pub const HFI_MPEG4_LEVEL_9: c_uint = 0x00000800;
pub const HFI_MPEG4_LEVEL_3b: c_uint = 0x00001000;
pub const HFI_VC1_PROFILE_SIMPLE: c_uint = 0x00000001;
pub const HFI_VC1_PROFILE_MAIN: c_uint = 0x00000002;
pub const HFI_VC1_PROFILE_ADVANCED: c_uint = 0x00000004;
pub const HFI_VC1_LEVEL_LOW: c_uint = 0x00000001;
pub const HFI_VC1_LEVEL_MEDIUM: c_uint = 0x00000002;
pub const HFI_VC1_LEVEL_HIGH: c_uint = 0x00000004;
pub const HFI_VC1_LEVEL_0: c_uint = 0x00000008;
pub const HFI_VC1_LEVEL_1: c_uint = 0x00000010;
pub const HFI_VC1_LEVEL_2: c_uint = 0x00000020;
pub const HFI_VC1_LEVEL_3: c_uint = 0x00000040;
pub const HFI_VC1_LEVEL_4: c_uint = 0x00000080;
pub const HFI_VPX_PROFILE_SIMPLE: c_uint = 0x00000001;
pub const HFI_VPX_PROFILE_ADVANCED: c_uint = 0x00000002;
pub const HFI_VPX_PROFILE_VERSION_0: c_uint = 0x00000004;
pub const HFI_VPX_PROFILE_VERSION_1: c_uint = 0x00000008;
pub const HFI_VPX_PROFILE_VERSION_2: c_uint = 0x00000010;
pub const HFI_VPX_PROFILE_VERSION_3: c_uint = 0x00000020;
pub const HFI_DIVX_FORMAT_4: c_uint = 0x1;
pub const HFI_DIVX_FORMAT_5: c_uint = 0x2;
pub const HFI_DIVX_FORMAT_6: c_uint = 0x3;
pub const HFI_DIVX_PROFILE_QMOBILE: c_uint = 0x00000001;
pub const HFI_DIVX_PROFILE_MOBILE: c_uint = 0x00000002;
pub const HFI_DIVX_PROFILE_MT: c_uint = 0x00000004;
pub const HFI_DIVX_PROFILE_HT: c_uint = 0x00000008;
pub const HFI_DIVX_PROFILE_HD: c_uint = 0x00000010;
pub const HFI_HEVC_PROFILE_MAIN: c_uint = 0x00000001;
pub const HFI_HEVC_PROFILE_MAIN10: c_uint = 0x00000002;
pub const HFI_HEVC_PROFILE_MAIN_STILL_PIC: c_uint = 0x00000004;
pub const HFI_HEVC_LEVEL_1: c_uint = 0x00000001;
pub const HFI_HEVC_LEVEL_2: c_uint = 0x00000002;
pub const HFI_HEVC_LEVEL_21: c_uint = 0x00000004;
pub const HFI_HEVC_LEVEL_3: c_uint = 0x00000008;
pub const HFI_HEVC_LEVEL_31: c_uint = 0x00000010;
pub const HFI_HEVC_LEVEL_4: c_uint = 0x00000020;
pub const HFI_HEVC_LEVEL_41: c_uint = 0x00000040;
pub const HFI_HEVC_LEVEL_5: c_uint = 0x00000080;
pub const HFI_HEVC_LEVEL_51: c_uint = 0x00000100;
pub const HFI_HEVC_LEVEL_52: c_uint = 0x00000200;
pub const HFI_HEVC_LEVEL_6: c_uint = 0x00000400;
pub const HFI_HEVC_LEVEL_61: c_uint = 0x00000800;
pub const HFI_HEVC_LEVEL_62: c_uint = 0x00001000;
pub const HFI_HEVC_TIER_MAIN: c_uint = 0x1;
pub const HFI_HEVC_TIER_HIGH0: c_uint = 0x2;
pub const HFI_VPX_PROFILE_MAIN: c_uint = 0x00000001;
pub const HFI_VPX_LEVEL_VERSION_0: c_uint = 0x00000001;
pub const HFI_VPX_LEVEL_VERSION_1: c_uint = 0x00000002;
pub const HFI_VPX_LEVEL_VERSION_2: c_uint = 0x00000004;
pub const HFI_VPX_LEVEL_VERSION_3: c_uint = 0x00000008;
// VP9 Profile 0, 8-bit
pub const HFI_VP9_PROFILE_P0: c_uint = 0x00000001;
// VP9 Profile 2, 10-bit
pub const HFI_VP9_PROFILE_P2_10B: c_uint = 0x00000004;
pub const HFI_VP9_LEVEL_1: c_uint = 0x00000001;
pub const HFI_VP9_LEVEL_11: c_uint = 0x00000002;
pub const HFI_VP9_LEVEL_2: c_uint = 0x00000004;
pub const HFI_VP9_LEVEL_21: c_uint = 0x00000008;
pub const HFI_VP9_LEVEL_3: c_uint = 0x00000010;
pub const HFI_VP9_LEVEL_31: c_uint = 0x00000020;
pub const HFI_VP9_LEVEL_4: c_uint = 0x00000040;
pub const HFI_VP9_LEVEL_41: c_uint = 0x00000080;
pub const HFI_VP9_LEVEL_5: c_uint = 0x00000100;
pub const HFI_VP9_LEVEL_51: c_uint = 0x00000200;
pub const HFI_VP9_LEVEL_6: c_uint = 0x00000400;
pub const HFI_VP9_LEVEL_61: c_uint = 0x00000800;
pub const HFI_BUFFER_INPUT: c_uint = 0x1;
pub const HFI_BUFFER_OUTPUT: c_uint = 0x2;
pub const HFI_BUFFER_OUTPUT2: c_uint = 0x3;
pub const HFI_BUFFER_INTERNAL_PERSIST: c_uint = 0x4;
pub const HFI_BUFFER_INTERNAL_PERSIST_1: c_uint = 0x5;

pub const HFI_BUFFER_TYPE_MAX: c_int = 11;
pub const HFI_BUFFER_MODE_STATIC: c_uint = 0x1000001;
pub const HFI_BUFFER_MODE_RING: c_uint = 0x1000002;
pub const HFI_BUFFER_MODE_DYNAMIC: c_uint = 0x1000003;
//
// HFI_PROPERTY_SYS_COMMON_START
// HFI_DOMAIN_BASE_COMMON + HFI_ARCH_COMMON_OFFSET + 0x0000
//
pub const HFI_PROPERTY_SYS_DEBUG_CONFIG: c_uint = 0x1;
pub const HFI_PROPERTY_SYS_RESOURCE_OCMEM_REQUIREMENT_INFO: c_uint = 0x2;
pub const HFI_PROPERTY_SYS_CONFIG_VCODEC_CLKFREQ: c_uint = 0x3;
pub const HFI_PROPERTY_SYS_IDLE_INDICATOR: c_uint = 0x4;
pub const HFI_PROPERTY_SYS_CODEC_POWER_PLANE_CTRL: c_uint = 0x5;
pub const HFI_PROPERTY_SYS_IMAGE_VERSION: c_uint = 0x6;
pub const HFI_PROPERTY_SYS_CONFIG_COVERAGE: c_uint = 0x7;
pub const HFI_PROPERTY_SYS_UBWC_CONFIG: c_uint = 0x8;
//
// HFI_PROPERTY_PARAM_COMMON_START
// HFI_DOMAIN_BASE_COMMON + HFI_ARCH_COMMON_OFFSET + 0x1000
//
pub const HFI_PROPERTY_PARAM_FRAME_SIZE: c_uint = 0x1001;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_PLANE_ACTUAL_INFO: c_uint = 0x1002;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_FORMAT_SELECT: c_uint = 0x1003;
pub const HFI_PROPERTY_PARAM_UNCOMPRESSED_FORMAT_SUPPORTED: c_uint = 0x1004;
pub const HFI_PROPERTY_PARAM_PROFILE_LEVEL_CURRENT: c_uint = 0x1005;
pub const HFI_PROPERTY_PARAM_PROFILE_LEVEL_SUPPORTED: c_uint = 0x1006;
pub const HFI_PROPERTY_PARAM_CAPABILITY_SUPPORTED: c_uint = 0x1007;
pub const HFI_PROPERTY_PARAM_PROPERTIES_SUPPORTED: c_uint = 0x1008;
pub const HFI_PROPERTY_PARAM_CODEC_SUPPORTED: c_uint = 0x1009;
pub const HFI_PROPERTY_PARAM_NAL_STREAM_FORMAT_SUPPORTED: c_uint = 0x100a;
pub const HFI_PROPERTY_PARAM_NAL_STREAM_FORMAT_SELECT: c_uint = 0x100b;
pub const HFI_PROPERTY_PARAM_MULTI_VIEW_FORMAT: c_uint = 0x100c;
pub const HFI_PROPERTY_PARAM_MAX_SEQUENCE_HEADER_SIZE: c_uint = 0x100d;
pub const HFI_PROPERTY_PARAM_CODEC_MASK_SUPPORTED: c_uint = 0x100e;
pub const HFI_PROPERTY_PARAM_MVC_BUFFER_LAYOUT: c_uint = 0x100f;
pub const HFI_PROPERTY_PARAM_MAX_SESSIONS_SUPPORTED: c_uint = 0x1010;
pub const HFI_PROPERTY_PARAM_WORK_MODE: c_uint = 0x1015;
pub const HFI_PROPERTY_PARAM_WORK_ROUTE: c_uint = 0x1017;
//
// HFI_PROPERTY_CONFIG_COMMON_START
// HFI_DOMAIN_BASE_COMMON + HFI_ARCH_COMMON_OFFSET + 0x2000
//
pub const HFI_PROPERTY_CONFIG_FRAME_RATE: c_uint = 0x2001;
pub const HFI_PROPERTY_CONFIG_VIDEOCORES_USAGE: c_uint = 0x2002;
//
// HFI_PROPERTY_PARAM_VDEC_COMMON_START
// HFI_DOMAIN_BASE_VDEC + HFI_ARCH_COMMON_OFFSET + 0x3000
//
pub const HFI_PROPERTY_PARAM_VDEC_MULTI_STREAM: c_uint = 0x1003001;
pub const HFI_PROPERTY_PARAM_VDEC_CONCEAL_COLOR: c_uint = 0x1003002;
pub const HFI_PROPERTY_PARAM_VDEC_NONCP_OUTPUT2: c_uint = 0x1003003;
pub const HFI_PROPERTY_PARAM_VDEC_PIXEL_BITDEPTH: c_uint = 0x1003007;
pub const HFI_PROPERTY_PARAM_VDEC_PIC_STRUCT: c_uint = 0x1003009;
pub const HFI_PROPERTY_PARAM_VDEC_COLOUR_SPACE: c_uint = 0x100300a;

//
// HFI_PROPERTY_CONFIG_VDEC_COMMON_START
// HFI_DOMAIN_BASE_VDEC + HFI_ARCH_COMMON_OFFSET + 0x4000
//
// HFI_PROPERTY_PARAM_VENC_COMMON_START
// HFI_DOMAIN_BASE_VENC + HFI_ARCH_COMMON_OFFSET + 0x5000
//
pub const HFI_PROPERTY_PARAM_VENC_SLICE_DELIVERY_MODE: c_uint = 0x2005001;
pub const HFI_PROPERTY_PARAM_VENC_H264_ENTROPY_CONTROL: c_uint = 0x2005002;
pub const HFI_PROPERTY_PARAM_VENC_H264_DEBLOCK_CONTROL: c_uint = 0x2005003;
pub const HFI_PROPERTY_PARAM_VENC_RATE_CONTROL: c_uint = 0x2005004;
pub const HFI_PROPERTY_PARAM_VENC_H264_PICORDER_CNT_TYPE: c_uint = 0x2005005;
pub const HFI_PROPERTY_PARAM_VENC_SESSION_QP: c_uint = 0x2005006;
pub const HFI_PROPERTY_PARAM_VENC_MPEG4_AC_PREDICTION: c_uint = 0x2005007;
pub const HFI_PROPERTY_PARAM_VENC_SESSION_QP_RANGE: c_uint = 0x2005008;
//
// Note: HFI_PROPERTY_PARAM_VENC_SESSION_QP_RANGE_V2 is
// specific to HFI_VERSION_6XX and HFI_VERSION_4XX only
//
pub const HFI_PROPERTY_PARAM_VENC_SESSION_QP_RANGE_V2: c_uint = 0x2005009;
pub const HFI_PROPERTY_PARAM_VENC_MPEG4_TIME_RESOLUTION: c_uint = 0x2005009;
pub const HFI_PROPERTY_PARAM_VENC_MPEG4_SHORT_HEADER: c_uint = 0x200500a;
pub const HFI_PROPERTY_PARAM_VENC_MPEG4_HEADER_EXTENSION: c_uint = 0x200500b;
pub const HFI_PROPERTY_PARAM_VENC_OPEN_GOP: c_uint = 0x200500c;
pub const HFI_PROPERTY_PARAM_VENC_INTRA_REFRESH: c_uint = 0x200500d;
pub const HFI_PROPERTY_PARAM_VENC_MULTI_SLICE_CONTROL: c_uint = 0x200500e;
pub const HFI_PROPERTY_PARAM_VENC_VBV_HRD_BUF_SIZE: c_uint = 0x200500f;
pub const HFI_PROPERTY_PARAM_VENC_QUALITY_VS_SPEED: c_uint = 0x2005010;
pub const HFI_PROPERTY_PARAM_VENC_ADVANCED: c_uint = 0x2005012;
pub const HFI_PROPERTY_PARAM_VENC_H264_SPS_ID: c_uint = 0x2005014;
pub const HFI_PROPERTY_PARAM_VENC_H264_PPS_ID: c_uint = 0x2005015;
pub const HFI_PROPERTY_PARAM_VENC_H264_GENERATE_AUDNAL: c_uint = 0x2005016;
pub const HFI_PROPERTY_PARAM_VENC_ASPECT_RATIO: c_uint = 0x2005017;
pub const HFI_PROPERTY_PARAM_VENC_NUMREF: c_uint = 0x2005018;
pub const HFI_PROPERTY_PARAM_VENC_MULTIREF_P: c_uint = 0x2005019;
pub const HFI_PROPERTY_PARAM_VENC_H264_NAL_SVC_EXT: c_uint = 0x200501b;
pub const HFI_PROPERTY_PARAM_VENC_LTRMODE: c_uint = 0x200501c;
pub const HFI_PROPERTY_PARAM_VENC_VIDEO_FULL_RANGE: c_uint = 0x200501d;
pub const HFI_PROPERTY_PARAM_VENC_H264_VUI_TIMING_INFO: c_uint = 0x200501e;
pub const HFI_PROPERTY_PARAM_VENC_VC1_PERF_CFG: c_uint = 0x200501f;
pub const HFI_PROPERTY_PARAM_VENC_MAX_NUM_B_FRAMES: c_uint = 0x2005020;
pub const HFI_PROPERTY_PARAM_VENC_H264_VUI_BITSTREAM_RESTRC: c_uint = 0x2005021;
pub const HFI_PROPERTY_PARAM_VENC_PRESERVE_TEXT_QUALITY: c_uint = 0x2005023;
pub const HFI_PROPERTY_PARAM_VENC_H264_TRANSFORM_8X8: c_uint = 0x2005025;
pub const HFI_PROPERTY_PARAM_VENC_HIER_P_MAX_NUM_ENH_LAYER: c_uint = 0x2005026;
pub const HFI_PROPERTY_PARAM_VENC_DISABLE_RC_TIMESTAMP: c_uint = 0x2005027;
pub const HFI_PROPERTY_PARAM_VENC_INITIAL_QP: c_uint = 0x2005028;
pub const HFI_PROPERTY_PARAM_VENC_VPX_ERROR_RESILIENCE_MODE: c_uint = 0x2005029;
pub const HFI_PROPERTY_PARAM_VENC_HIER_B_MAX_NUM_ENH_LAYER: c_uint = 0x200502c;
pub const HFI_PROPERTY_PARAM_VENC_HIER_P_HYBRID_MODE: c_uint = 0x200502f;
pub const HFI_PROPERTY_PARAM_VENC_HDR10_PQ_SEI: c_uint = 0x2005036;
//
// HFI_PROPERTY_CONFIG_VENC_COMMON_START
// HFI_DOMAIN_BASE_VENC + HFI_ARCH_COMMON_OFFSET + 0x6000
//
pub const HFI_PROPERTY_CONFIG_VENC_TARGET_BITRATE: c_uint = 0x2006001;
pub const HFI_PROPERTY_CONFIG_VENC_IDR_PERIOD: c_uint = 0x2006002;
pub const HFI_PROPERTY_CONFIG_VENC_INTRA_PERIOD: c_uint = 0x2006003;
pub const HFI_PROPERTY_CONFIG_VENC_REQUEST_SYNC_FRAME: c_uint = 0x2006004;
pub const HFI_PROPERTY_CONFIG_VENC_SLICE_SIZE: c_uint = 0x2006005;
pub const HFI_PROPERTY_CONFIG_VENC_MAX_BITRATE: c_uint = 0x2006007;
pub const HFI_PROPERTY_CONFIG_VENC_SYNC_FRAME_SEQUENCE_HEADER: c_uint = 0x2006008;
pub const HFI_PROPERTY_CONFIG_VENC_MARKLTRFRAME: c_uint = 0x2006009;
pub const HFI_PROPERTY_CONFIG_VENC_USELTRFRAME: c_uint = 0x200600a;
pub const HFI_PROPERTY_CONFIG_VENC_HIER_P_ENH_LAYER: c_uint = 0x200600b;
pub const HFI_PROPERTY_CONFIG_VENC_LTRPERIOD: c_uint = 0x200600c;
pub const HFI_PROPERTY_CONFIG_VENC_PERF_MODE: c_uint = 0x200600e;
pub const HFI_PROPERTY_CONFIG_HEIC_FRAME_QUALITY: c_uint = 0x2006014;
//
// HFI_PROPERTY_PARAM_VPE_COMMON_START
// HFI_DOMAIN_BASE_VPE + HFI_ARCH_COMMON_OFFSET + 0x7000
//
// HFI_PROPERTY_CONFIG_VPE_COMMON_START
// HFI_DOMAIN_BASE_VPE + HFI_ARCH_COMMON_OFFSET + 0x8000
//
pub const HFI_PROPERTY_CONFIG_VPE_DEINTERLACE: c_uint = 0x3008001;
pub const HFI_PROPERTY_CONFIG_VPE_OPERATIONS: c_uint = 0x3008002;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_version {
    HFI_VERSION_1XX,
    HFI_VERSION_3XX,
    HFI_VERSION_4XX,
    HFI_VERSION_6XX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_info {
    pub buffer_addr: u32,
    pub extradata_addr: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_bitrate {
    pub bitrate: u32,
    pub layer_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_h264_8x8_transform {
    pub enable_type: u32,
}

pub const HFI_CAPABILITY_FRAME_WIDTH: c_uint = 0x01;
pub const HFI_CAPABILITY_FRAME_HEIGHT: c_uint = 0x02;
pub const HFI_CAPABILITY_MBS_PER_FRAME: c_uint = 0x03;
pub const HFI_CAPABILITY_MBS_PER_SECOND: c_uint = 0x04;
pub const HFI_CAPABILITY_FRAMERATE: c_uint = 0x05;
pub const HFI_CAPABILITY_SCALE_X: c_uint = 0x06;
pub const HFI_CAPABILITY_SCALE_Y: c_uint = 0x07;
pub const HFI_CAPABILITY_BITRATE: c_uint = 0x08;
pub const HFI_CAPABILITY_BFRAME: c_uint = 0x09;
pub const HFI_CAPABILITY_PEAKBITRATE: c_uint = 0x0a;
pub const HFI_CAPABILITY_HIER_P_NUM_ENH_LAYERS: c_uint = 0x10;
pub const HFI_CAPABILITY_ENC_LTR_COUNT: c_uint = 0x11;
pub const HFI_CAPABILITY_CP_OUTPUT2_THRESH: c_uint = 0x12;
pub const HFI_CAPABILITY_HIER_B_NUM_ENH_LAYERS: c_uint = 0x13;
pub const HFI_CAPABILITY_LCU_SIZE: c_uint = 0x14;
pub const HFI_CAPABILITY_HIER_P_HYBRID_NUM_ENH_LAYERS: c_uint = 0x15;
pub const HFI_CAPABILITY_MBS_PER_SECOND_POWERSAVE: c_uint = 0x16;
pub const HFI_CAPABILITY_I_FRAME_QP: c_uint = 0x20;
pub const HFI_CAPABILITY_P_FRAME_QP: c_uint = 0x21;
pub const HFI_CAPABILITY_B_FRAME_QP: c_uint = 0x22;
pub const HFI_CAPABILITY_RATE_CONTROL_MODES: c_uint = 0x23;
pub const HFI_CAPABILITY_BLUR_WIDTH: c_uint = 0x24;
pub const HFI_CAPABILITY_BLUR_HEIGHT: c_uint = 0x25;
pub const HFI_CAPABILITY_SLICE_BYTE: c_uint = 0x27;
pub const HFI_CAPABILITY_SLICE_MB: c_uint = 0x28;
pub const HFI_CAPABILITY_MAX_VIDEOCORES: c_uint = 0x2b;
pub const HFI_CAPABILITY_MAX_WORKMODES: c_uint = 0x2c;
pub const HFI_CAPABILITY_ROTATION: c_uint = 0x2f;
pub const HFI_CAPABILITY_COLOR_SPACE_CONVERSION: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_capability {
    pub capability_type: u32,
    pub min: u32,
    pub max: u32,
    pub step_size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_capabilities {
    pub num_capabilities: u32,
    pub __counted_by(num_capabilities): hfi_capability data[],
}

pub const HFI_DEBUG_MSG_LOW: c_uint = 0x01;
pub const HFI_DEBUG_MSG_MEDIUM: c_uint = 0x02;
pub const HFI_DEBUG_MSG_HIGH: c_uint = 0x04;
pub const HFI_DEBUG_MSG_ERROR: c_uint = 0x08;
pub const HFI_DEBUG_MSG_FATAL: c_uint = 0x10;
pub const HFI_DEBUG_MSG_PERF: c_uint = 0x20;
pub const HFI_DEBUG_MODE_QUEUE: c_uint = 0x01;
pub const HFI_DEBUG_MODE_QDSS: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_debug_config {
    pub config: u32,
    pub mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ubwc_config {
    pub size: u32,
    pub packet_type: u32,
    pub 1: u32 max_channel_override :,
    pub 1: u32 mal_length_override :,
    pub 1: u32 hb_override :,
    pub 1: u32 bank_swzl_level_override :,
    pub 1: u32 bank_spreading_override :,
    pub 27: u32 reserved :,
    pub override_bit_info: },
    pub max_channels: u32,
    pub mal_length: u32,
    pub highest_bank_bit: u32,
    pub bank_swzl_level: u32,
    pub bank_spreading: u32,
    pub reserved: [u32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_enable {
    pub enable: u32,
}

pub const HFI_H264_DB_MODE_DISABLE: c_uint = 0x1;
pub const HFI_H264_DB_MODE_SKIP_SLICE_BOUNDARY: c_uint = 0x2;
pub const HFI_H264_DB_MODE_ALL_BOUNDARY: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_h264_db_control {
    pub mode: u32,
    pub slice_alpha_offset: i32,
    pub slice_beta_offset: i32,
}

pub const HFI_H264_ENTROPY_CAVLC: c_uint = 0x1;
pub const HFI_H264_ENTROPY_CABAC: c_uint = 0x2;
pub const HFI_H264_CABAC_MODEL_0: c_uint = 0x1;
pub const HFI_H264_CABAC_MODEL_1: c_uint = 0x2;
pub const HFI_H264_CABAC_MODEL_2: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_h264_entropy_control {
    pub entropy_mode: u32,
    pub cabac_model: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_framerate {
    pub buffer_type: u32,
    pub framerate: u32,
}

pub const HFI_INTRA_REFRESH_NONE: c_uint = 0x1;
pub const HFI_INTRA_REFRESH_CYCLIC: c_uint = 0x2;
pub const HFI_INTRA_REFRESH_ADAPTIVE: c_uint = 0x3;
pub const HFI_INTRA_REFRESH_CYCLIC_ADAPTIVE: c_uint = 0x4;
pub const HFI_INTRA_REFRESH_RANDOM: c_uint = 0x5;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_intra_refresh {
    pub mode: u32,
    pub air_mbs: u32,
    pub air_ref: u32,
    pub cir_mbs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_intra_refresh_3x {
    pub mode: u32,
    pub mbs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_idr_period {
    pub idr_period: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_operations_type {
    pub rotation: u32,
    pub flip: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_max_num_b_frames {
    pub max_num_b_frames: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_vc1e_perf_cfg_type {
    pub search_range_x_subsampled: [u32; 3],
    pub search_range_y_subsampled: [u32; 3],
}

//
// 0 - 7bit -> Luma (def: 16)
// 8 - 15bit -> Chroma (def: 128)
// format is valid up to v4
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_conceal_color {
    pub conceal_color: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_conceal_color_v4 {
    pub conceal_color_8bit: u32,
    pub conceal_color_10bit: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_intra_period {
    pub pframes: u32,
    pub bframes: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_mpeg4_header_extension {
    pub header_extension: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_mpeg4_time_resolution {
    pub time_increment_resolution: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_stream {
    pub buffer_type: u32,
    pub enable: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_stream_3x {
    pub buffer_type: u32,
    pub enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_view_format {
    pub views: u32,
    pub view_order: [u32; ],
}

pub const HFI_MULTI_SLICE_OFF: c_uint = 0x1;
pub const HFI_MULTI_SLICE_BY_MB_COUNT: c_uint = 0x2;
pub const HFI_MULTI_SLICE_BY_BYTE_COUNT: c_uint = 0x3;
pub const HFI_MULTI_SLICE_GOB: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_slice_control {
    pub multi_slice: u32,
    pub slice_size: u32,
}

pub const HFI_NAL_FORMAT_STARTCODES: c_uint = 0x01;
pub const HFI_NAL_FORMAT_ONE_NAL_PER_BUFFER: c_uint = 0x02;
pub const HFI_NAL_FORMAT_ONE_BYTE_LENGTH: c_uint = 0x04;
pub const HFI_NAL_FORMAT_TWO_BYTE_LENGTH: c_uint = 0x08;
pub const HFI_NAL_FORMAT_FOUR_BYTE_LENGTH: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_nal_stream_format {
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_nal_stream_format_select {
    pub format: u32,
}

pub const HFI_PICTURE_TYPE_I: c_uint = 0x01;
pub const HFI_PICTURE_TYPE_P: c_uint = 0x02;
pub const HFI_PICTURE_TYPE_B: c_uint = 0x04;
pub const HFI_PICTURE_TYPE_IDR: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_profile_level {
    pub profile: u32,
    pub level: u32,
}

pub const HFI_MAX_PROFILE_COUNT: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_profile_level_supported {
    pub profile_count: u32,
    pub __counted_by(profile_count): hfi_profile_level profile_level[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quality_vs_speed {
    pub quality_vs_speed: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_heic_frame_quality {
    pub frame_quality: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization {
    pub qp_i: u32,
    pub qp_p: u32,
    pub qp_b: u32,
    pub layer_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_initial_quantization {
    pub qp_i: u32,
    pub qp_p: u32,
    pub qp_b: u32,
    pub init_qp_enable: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization_range {
    pub min_qp: u32,
    pub max_qp: u32,
    pub layer_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization_v2 {
    pub qp_packed: u32,
    pub layer_id: u32,
    pub enable: u32,
    pub reserved: [u32; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_quantization_range_v2 {
    pub min_qp: hfi_quantization_v2,
    pub max_qp: hfi_quantization_v2,
    pub reserved: [u32; 4],
}

pub const HFI_LTR_MODE_DISABLE: c_uint = 0x0;
pub const HFI_LTR_MODE_MANUAL: c_uint = 0x1;
pub const HFI_LTR_MODE_PERIODIC: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_mode {
    pub ltr_mode: u32,
    pub ltr_count: u32,
    pub trust_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_use {
    pub ref_ltr: u32,
    pub use_constrnt: u32,
    pub frames: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_ltr_mark {
    pub mark_frame: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_mastering_display_colour_sei_payload {
    pub display_primaries_x: [u32; 3],
    pub display_primaries_y: [u32; 3],
    pub white_point_x: u32,
    pub white_point_y: u32,
    pub max_display_mastering_luminance: u32,
    pub min_display_mastering_luminance: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_content_light_level_sei_payload {
    pub max_content_light: u32,
    pub max_pic_average_light: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_hdr10_pq_sei {
    pub mastering: hfi_mastering_display_colour_sei_payload,
    pub cll: hfi_content_light_level_sei_payload,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_framesize {
    pub buffer_type: u32,
    pub width: u32,
    pub height: u32,
}

pub const HFI_VENC_PERFMODE_MAX_QUALITY: c_uint = 0x1;
pub const HFI_VENC_PERFMODE_POWER_SAVE: c_uint = 0x2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_perf_mode {
    pub video_perf_mode: u32,
}

pub const VIDC_CORE_ID_DEFAULT: c_int = 0;
pub const VIDC_CORE_ID_1: c_int = 1;
pub const VIDC_CORE_ID_2: c_int = 2;
pub const VIDC_CORE_ID_3: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_videocores_usage_type {
    pub video_core_enable_mask: u32,
}

pub const VIDC_WORK_MODE_1: c_int = 1;
pub const VIDC_WORK_MODE_2: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_video_work_mode {
    pub video_work_mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_video_work_route {
    pub video_work_route: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_h264_vui_timing_info {
    pub enable: u32,
    pub fixed_framerate: u32,
    pub time_scale: u32,
}

pub const VIDC_BITDEPTH_8: c_uint = 0x00000;
pub const VIDC_BITDEPTH_10: c_uint = 0x20002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_bit_depth {
    pub buffer_type: u32,
    pub bit_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_picture_type {
    pub is_sync_frame: u32,
    pub picture_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_pic_struct {
    pub progressive_only: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_colour_space {
    pub colour_space: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_extradata_input_crop {
    pub size: u32,
    pub version: u32,
    pub port_index: u32,
    pub left: u32,
    pub top: u32,
    pub width: u32,
    pub height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_dpb_counts {
    pub max_dpb_count: u32,
    pub max_ref_frames: u32,
    pub max_dec_buffering: u32,
    pub max_reorder_frames: u32,
    pub fw_min_cnt: u32,
}

pub const HFI_COLOR_FORMAT_MONOCHROME: c_uint = 0x01;
pub const HFI_COLOR_FORMAT_NV12: c_uint = 0x02;
pub const HFI_COLOR_FORMAT_NV21: c_uint = 0x03;
pub const HFI_COLOR_FORMAT_NV12_4x4TILE: c_uint = 0x04;
pub const HFI_COLOR_FORMAT_NV21_4x4TILE: c_uint = 0x05;
pub const HFI_COLOR_FORMAT_YUYV: c_uint = 0x06;
pub const HFI_COLOR_FORMAT_YVYU: c_uint = 0x07;
pub const HFI_COLOR_FORMAT_UYVY: c_uint = 0x08;
pub const HFI_COLOR_FORMAT_VYUY: c_uint = 0x09;
pub const HFI_COLOR_FORMAT_RGB565: c_uint = 0x0a;
pub const HFI_COLOR_FORMAT_BGR565: c_uint = 0x0b;
pub const HFI_COLOR_FORMAT_RGB888: c_uint = 0x0c;
pub const HFI_COLOR_FORMAT_BGR888: c_uint = 0x0d;
pub const HFI_COLOR_FORMAT_YUV444: c_uint = 0x0e;
pub const HFI_COLOR_FORMAT_RGBA8888: c_uint = 0x10;
pub const HFI_COLOR_FORMAT_UBWC_BASE: c_uint = 0x8000;
pub const HFI_COLOR_FORMAT_10_BIT_BASE: c_uint = 0x4000;
pub const HFI_COLOR_FORMAT_YUV420_TP10: c_uint = 0x4002;
pub const HFI_COLOR_FORMAT_P010: c_uint = 0x4003;
pub const HFI_COLOR_FORMAT_NV12_UBWC: c_uint = 0x8002;
pub const HFI_COLOR_FORMAT_YUV420_TP10_UBWC: c_uint = 0xc002;
pub const HFI_COLOR_FORMAT_P010_UBWC: c_uint = 0xc003;
pub const HFI_COLOR_FORMAT_RGBA8888_UBWC: c_uint = 0x8010;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_format_select {
    pub buffer_type: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_constraints {
    pub stride_multiples: u32,
    pub max_stride: u32,
    pub min_plane_buffer_height_multiple: u32,
    pub buffer_alignment: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_info {
    pub format: u32,
    pub num_planes: u32,
    pub plane_constraints: hfi_uncompressed_plane_constraints,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_format_supported {
    pub buffer_type: u32,
    pub format_entries: u32,
    pub plane_info: hfi_uncompressed_plane_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual {
    pub actual_stride: c_int,
    pub actual_plane_buffer_height: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual_info {
    pub buffer_type: u32,
    pub num_planes: u32,
    pub plane_format: [hfi_uncompressed_plane_actual; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_uncompressed_plane_actual_constraints_info {
    pub buffer_type: u32,
    pub num_planes: u32,
    pub plane_format: [hfi_uncompressed_plane_constraints; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_codec_supported {
    pub dec_codecs: u32,
    pub enc_codecs: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_properties_supported {
    pub num_properties: u32,
    pub properties: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_max_sessions_supported {
    pub max_sessions: u32,
}

pub const HFI_MAX_MATRIX_COEFFS: c_int = 9;
pub const HFI_MAX_BIAS_COEFFS: c_int = 3;
pub const HFI_MAX_LIMIT_COEFFS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_vpe_color_space_conversion {
    pub csc_matrix: [u32; HFI_MAX_MATRIX_COEFFS],
    pub csc_bias: [u32; HFI_MAX_BIAS_COEFFS],
    pub csc_limit: [u32; HFI_MAX_LIMIT_COEFFS],
}

pub const HFI_ROTATE_NONE: c_uint = 0x1;
pub const HFI_ROTATE_90: c_uint = 0x2;
pub const HFI_ROTATE_180: c_uint = 0x3;
pub const HFI_ROTATE_270: c_uint = 0x4;
pub const HFI_FLIP_NONE: c_uint = 0x1;
pub const HFI_FLIP_HORIZONTAL: c_uint = 0x2;
pub const HFI_FLIP_VERTICAL: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_operations {
    pub rotate: u32,
    pub flip: u32,
}

pub const HFI_RESOURCE_OCMEM: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_resource_ocmem {
    pub size: u32,
    pub mem: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_resource_ocmem_requirement {
    pub session_domain: u32,
    pub width: u32,
    pub height: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_resource_ocmem_requirement_info {
    pub num_entries: u32,
    pub __counted_by(num_entries): hfi_resource_ocmem_requirement requirements[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_property_sys_image_version_info_type {
    pub string_size: u32,
    pub str_image_version: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_codec_mask_supported {
    pub codecs: u32,
    pub video_domains: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_seq_header_info {
    pub max_hader_len: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_aspect_ratio {
    pub aspect_width: u32,
    pub aspect_height: u32,
}

pub const HFI_MVC_BUFFER_LAYOUT_TOP_BOTTOM: c_int = 0;
pub const HFI_MVC_BUFFER_LAYOUT_SIDEBYSIDE: c_int = 1;
pub const HFI_MVC_BUFFER_LAYOUT_SEQ: c_int = 2;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_mvc_buffer_layout_descp_type {
    pub layout_type: u32,
    pub bright_view_first: u32,
    pub ngap: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_scs_threshold {
    pub threshold_value: u32,
}

pub const HFI_TEST_SSR_SW_ERR_FATAL: c_uint = 0x1;
pub const HFI_TEST_SSR_SW_DIV_BY_ZERO: c_uint = 0x2;
pub const HFI_TEST_SSR_HW_WDOG_IRQ: c_uint = 0x3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_alloc_mode {
    pub type: u32,
    pub mode: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_index_extradata_config {
    pub enable: u32,
    pub index_extra_data_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_extradata_header {
    pub size: u32,
    pub version: u32,
    pub port_index: u32,
    pub type: u32,
    pub data_size: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_batch_info {
    pub input_batch_count: u32,
    pub output_batch_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_count_actual {
    pub type: u32,
    pub count_actual: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_count_actual_4xx {
    pub type: u32,
    pub count_actual: u32,
    pub count_min_host: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_size_actual {
    pub type: u32,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_display_hold_count_actual {
    pub type: u32,
    pub hold_count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_requirements {
    pub type: u32,
    pub size: u32,
    pub region_size: u32,
    pub hold_count: u32,
    pub count_min: u32,
    pub count_actual: u32,
    pub contiguous: u32,
    pub alignment: u32,
}

// On HFI 4XX, some of the struct members have been swapped.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_data_payload {
    pub size: u32,
    pub data: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_enable_picture {
    pub picture_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_display_picture_buffer_count {
    pub enable: c_int,
    pub count: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_extra_data_header_config {
    pub type: u32,
    pub buffer_type: u32,
    pub version: u32,
    pub port_index: u32,
    pub client_extra_data_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_interlace_format_supported {
    pub buffer_type: u32,
    pub format: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_buffer_alloc_mode_supported {
    pub buffer_type: u32,
    pub num_entries: u32,
    pub data: [u32; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_mb_error_map {
    pub error_map_size: u32,
    pub error_map: [u8; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_metadata_pass_through {
    pub enable: c_int,
    pub size: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_multi_view_select {
    pub view_index: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_hybrid_hierp {
    pub layers: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_pkt_hdr {
    pub size: u32,
    pub pkt_type: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_hdr_pkt {
    pub hdr: hfi_pkt_hdr,
    pub session_id: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_session_pkt {
    pub shdr: hfi_session_hdr_pkt,
}
