//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/platform/qcom/iris/iris_hfi_gen2_defines.h
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

pub const HFI_VIDEO_ARCH_LX: c_uint = 0x1;
pub const HFI_CMD_BEGIN: c_uint = 0x01000000;
pub const HFI_CMD_INIT: c_uint = 0x01000001;
pub const HFI_CMD_POWER_COLLAPSE: c_uint = 0x01000002;
pub const HFI_CMD_OPEN: c_uint = 0x01000003;
pub const HFI_CMD_CLOSE: c_uint = 0x01000004;
pub const HFI_CMD_START: c_uint = 0x01000005;
pub const HFI_CMD_STOP: c_uint = 0x01000006;
pub const HFI_CMD_DRAIN: c_uint = 0x01000007;
pub const HFI_CMD_RESUME: c_uint = 0x01000008;
pub const HFI_CMD_BUFFER: c_uint = 0x01000009;
pub const HFI_CMD_SUBSCRIBE_MODE: c_uint = 0x0100000B;
pub const HFI_CMD_SETTINGS_CHANGE: c_uint = 0x0100000C;
pub const HFI_CMD_PAUSE: c_uint = 0x01000011;
pub const HFI_CMD_END: c_uint = 0x01FFFFFF;
pub const HFI_BITMASK_BITSTREAM_WIDTH: c_uint = 0xffff0000;
pub const HFI_BITMASK_BITSTREAM_HEIGHT: c_uint = 0x0000ffff;
pub const HFI_BITMASK_FRAME_MBS_ONLY_FLAG: c_uint = 0x00000001;
pub const HFI_PROP_BEGIN: c_uint = 0x03000000;
pub const HFI_PROP_IMAGE_VERSION: c_uint = 0x03000001;
pub const HFI_PROP_INTRA_FRAME_POWER_COLLAPSE: c_uint = 0x03000002;
pub const HFI_PROP_UBWC_MAX_CHANNELS: c_uint = 0x03000003;
pub const HFI_PROP_UBWC_MAL_LENGTH: c_uint = 0x03000004;
pub const HFI_PROP_UBWC_HBB: c_uint = 0x03000005;
pub const HFI_PROP_UBWC_BANK_SWZL_LEVEL1: c_uint = 0x03000006;
pub const HFI_PROP_UBWC_BANK_SWZL_LEVEL2: c_uint = 0x03000007;
pub const HFI_PROP_UBWC_BANK_SWZL_LEVEL3: c_uint = 0x03000008;
pub const HFI_PROP_UBWC_BANK_SPREADING: c_uint = 0x03000009;
pub const HFI_PROP_CODEC: c_uint = 0x03000100;
pub const HFI_PROP_COLOR_FORMAT: c_uint = 0x03000101;
pub const HFI_PROP_BITSTREAM_RESOLUTION: c_uint = 0x03000103;
pub const HFI_PROP_LINEAR_STRIDE_SCANLINE: c_uint = 0x03000104;
pub const HFI_PROP_CROP_OFFSETS: c_uint = 0x03000105;
pub const HFI_PROP_PROFILE: c_uint = 0x03000107;
pub const HFI_PROP_LEVEL: c_uint = 0x03000108;
pub const HFI_PROP_TIER: c_uint = 0x03000109;
pub const HFI_PROP_STAGE: c_uint = 0x0300010a;
pub const HFI_PROP_PIPE: c_uint = 0x0300010b;
pub const HFI_PROP_FRAME_RATE: c_uint = 0x0300010c;
pub const HFI_PROP_LUMA_CHROMA_BIT_DEPTH: c_uint = 0x0300010f;
pub const HFI_PROP_CODED_FRAMES: c_uint = 0x03000120;
pub const HFI_PROP_CABAC_SESSION: c_uint = 0x03000121;
pub const HFI_PROP_BUFFER_HOST_MAX_COUNT: c_uint = 0x03000123;
pub const HFI_PROP_BUFFER_FW_MIN_OUTPUT_COUNT: c_uint = 0x03000124;
pub const HFI_PROP_PIC_ORDER_CNT_TYPE: c_uint = 0x03000128;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_rate_control {
    HFI_RC_VBR_CFR		= 0x00000000,
    HFI_RC_CBR_CFR		= 0x00000001,
    HFI_RC_CQ		= 0x00000002,
    HFI_RC_OFF		= 0x00000003,
    HFI_RC_CBR_VFR		= 0x00000004,
    HFI_RC_LOSSLESS		= 0x00000005,
}

pub const HFI_PROP_RATE_CONTROL: c_uint = 0x0300012a;
pub const HFI_PROP_TIME_DELTA_BASED_RATE_CONTROL: c_uint = 0x0300012b;
pub const HFI_PROP_QP_PACKED: c_uint = 0x0300012e;
pub const HFI_PROP_MIN_QP_PACKED: c_uint = 0x0300012f;
pub const HFI_PROP_MAX_QP_PACKED: c_uint = 0x03000130;
pub const HFI_PROP_IR_RANDOM_PERIOD: c_uint = 0x03000131;
pub const HFI_PROP_LTR_COUNT: c_uint = 0x03000134;
pub const HFI_PROP_LTR_MARK: c_uint = 0x03000135;
pub const HFI_PROP_LTR_USE: c_uint = 0x03000136;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_layer_encoding_type {
    HFI_HIER_P_SLIDING_WINDOW	= 0x1,
    HFI_HIER_P_HYBRID_LTR		= 0x2,
    HFI_HIER_B			= 0x3,
}

pub const HFI_PROP_LAYER_ENCODING_TYPE: c_uint = 0x03000138;
pub const HFI_PROP_LAYER_COUNT: c_uint = 0x03000139;
pub const HFI_PROP_TOTAL_BITRATE: c_uint = 0x0300013b;
pub const HFI_PROP_BITRATE_LAYER1: c_uint = 0x0300013c;
pub const HFI_PROP_BITRATE_LAYER2: c_uint = 0x0300013d;
pub const HFI_PROP_BITRATE_LAYER3: c_uint = 0x0300013e;
pub const HFI_PROP_BITRATE_LAYER4: c_uint = 0x0300013f;
pub const HFI_PROP_BITRATE_LAYER5: c_uint = 0x03000140;
pub const HFI_PROP_BITRATE_LAYER6: c_uint = 0x03000141;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_syncframe_request_mode {
    HFI_SYNC_FRAME_REQUEST_WITHOUT_SEQ_HDR		= 0x00000001,
    HFI_SYNC_FRAME_REQUEST_WITH_PREFIX_SEQ_HDR	= 0x00000002,
}

pub const HFI_PROP_REQUEST_SYNC_FRAME: c_uint = 0x03000145;
pub const HFI_PROP_MAX_GOP_FRAMES: c_uint = 0x03000146;
pub const HFI_PROP_MAX_B_FRAMES: c_uint = 0x03000147;
pub const HFI_PROP_QUALITY_MODE: c_uint = 0x03000148;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_seq_header_mode {
    HFI_SEQ_HEADER_SEPERATE_FRAME		= 0x00000001,
    HFI_SEQ_HEADER_JOINED_WITH_1ST_FRAME	= 0x00000002,
    HFI_SEQ_HEADER_PREFIX_WITH_SYNC_FRAME	= 0x00000004,
    HFI_SEQ_HEADER_METADATA			= 0x00000008,
}

pub const HFI_PROP_SEQ_HEADER_MODE: c_uint = 0x03000149;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_rotation {
    HFI_ROTATION_NONE = 0x00000000,
    HFI_ROTATION_90   = 0x00000001,
    HFI_ROTATION_180  = 0x00000002,
    HFI_ROTATION_270  = 0x00000003,
}

pub const HFI_PROP_ROTATION: c_uint = 0x0300014b;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_flip {
    HFI_DISABLE_FLIP    = 0x00000000,
    HFI_HORIZONTAL_FLIP = 0x00000001,
    HFI_VERTICAL_FLIP   = 0x00000002,
}

pub const HFI_PROP_FLIP: c_uint = 0x0300014c;
pub const HFI_PROP_SIGNAL_COLOR_INFO: c_uint = 0x03000155;
pub const HFI_PROP_PICTURE_TYPE: c_uint = 0x03000162;
pub const HFI_PROP_DEC_DEFAULT_HEADER: c_uint = 0x03000168;
pub const HFI_PROP_DEC_START_FROM_RAP_FRAME: c_uint = 0x03000169;
pub const HFI_PROP_NO_OUTPUT: c_uint = 0x0300016a;
pub const HFI_PROP_BUFFER_MARK: c_uint = 0x0300016c;
pub const HFI_PROP_WORST_COMPRESSION_RATIO: c_uint = 0x03000174;
pub const HFI_PROP_WORST_COMPLEXITY_FACTOR: c_uint = 0x03000175;
pub const HFI_PROP_RAW_RESOLUTION: c_uint = 0x03000178;
pub const HFI_PROP_TOTAL_PEAK_BITRATE: c_uint = 0x0300017C;
pub const HFI_PROP_AV1_FILM_GRAIN_PRESENT: c_uint = 0x03000180;
pub const HFI_PROP_AV1_SUPER_BLOCK_ENABLED: c_uint = 0x03000181;
pub const HFI_PROP_AV1_OP_POINT: c_uint = 0x03000182;
pub const HFI_PROP_IR_CYCLIC_PERIOD: c_uint = 0x0300017E;
pub const HFI_PROP_OPB_ENABLE: c_uint = 0x03000184;
pub const HFI_PROP_AV1_TILE_ROWS_COLUMNS: c_uint = 0x03000187;
pub const HFI_PROP_AV1_DRAP_CONFIG: c_uint = 0x03000189;
pub const HFI_PROP_UBWC_STRIDE_SCANLINE: c_uint = 0x03000190;
pub const HFI_PROP_COMV_BUFFER_COUNT: c_uint = 0x03000193;
pub const HFI_PROP_AV1_UNIFORM_TILE_SPACING: c_uint = 0x03000197;
pub const HFI_PROP_END: c_uint = 0x03FFFFFF;
pub const HFI_SESSION_ERROR_BEGIN: c_uint = 0x04000000;
pub const HFI_ERROR_UNKNOWN_SESSION: c_uint = 0x04000001;
pub const HFI_ERROR_MAX_SESSIONS: c_uint = 0x04000002;
pub const HFI_ERROR_FATAL: c_uint = 0x04000003;
pub const HFI_ERROR_INVALID_STATE: c_uint = 0x04000004;
pub const HFI_ERROR_INSUFFICIENT_RESOURCES: c_uint = 0x04000005;
pub const HFI_ERROR_BUFFER_NOT_SET: c_uint = 0x04000006;
pub const HFI_ERROR_STREAM_UNSUPPORTED: c_uint = 0x04000008;
pub const HFI_SESSION_ERROR_END: c_uint = 0x04FFFFFF;
pub const HFI_SYSTEM_ERROR_BEGIN: c_uint = 0x05000000;
pub const HFI_SYS_ERROR_WD_TIMEOUT: c_uint = 0x05000001;
pub const HFI_SYSTEM_ERROR_END: c_uint = 0x05FFFFFF;
pub const HFI_INFORMATION_BEGIN: c_uint = 0x06000000;
pub const HFI_INFO_UNSUPPORTED: c_uint = 0x06000001;
pub const HFI_INFO_DATA_CORRUPT: c_uint = 0x06000002;
pub const HFI_INFO_BUFFER_OVERFLOW: c_uint = 0x06000004;
pub const HFI_INFO_HFI_FLAG_DRAIN_LAST: c_uint = 0x06000006;
pub const HFI_INFO_HFI_FLAG_PSC_LAST: c_uint = 0x06000007;
pub const HFI_INFORMATION_END: c_uint = 0x06FFFFFF;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_property_mode_type {
    HFI_MODE_PORT_SETTINGS_CHANGE		= 0x00000001,
    HFI_MODE_PROPERTY			= 0x00000002,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_color_format {
    HFI_COLOR_FMT_OPAQUE			= 0,
    HFI_COLOR_FMT_NV12			= 1,
    HFI_COLOR_FMT_NV12_UBWC			= 2,
    HFI_COLOR_FMT_P010			= 3,
    HFI_COLOR_FMT_TP10_UBWC			= 4,
    HFI_COLOR_FMT_RGBA8888			= 5,
    HFI_COLOR_FMT_RGBA8888_UBWC		= 6,
    HFI_COLOR_FMT_NV21			= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_codec_type {
    HFI_CODEC_DECODE_AVC			= 1,
    HFI_CODEC_ENCODE_AVC			= 2,
    HFI_CODEC_DECODE_HEVC			= 3,
    HFI_CODEC_ENCODE_HEVC			= 4,
    HFI_CODEC_DECODE_VP9			= 5,
    HFI_CODEC_DECODE_AV1			= 7,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_picture_type {
    HFI_GEN2_PICTURE_IDR		= 0x00000001,
    HFI_GEN2_PICTURE_P			= 0x00000002,
    HFI_GEN2_PICTURE_B			= 0x00000004,
    HFI_GEN2_PICTURE_I			= 0x00000008,
    HFI_GEN2_PICTURE_CRA		= 0x00000010,
    HFI_GEN2_PICTURE_BLA		= 0x00000020,
    HFI_GEN2_PICTURE_NOSHOW     = 0x00000040,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_buffer_type {
    HFI_BUFFER_BITSTREAM			= 0x00000001,
    HFI_BUFFER_RAW				= 0x00000002,
    HFI_BUFFER_METADATA			= 0x00000003,
    HFI_BUFFER_SUBCACHE			= 0x00000004,
    HFI_BUFFER_PARTIAL_DATA			= 0x00000005,
    HFI_BUFFER_DPB				= 0x00000006,
    HFI_BUFFER_BIN				= 0x00000007,
    HFI_BUFFER_LINE				= 0x00000008,
    HFI_BUFFER_ARP				= 0x00000009,
    HFI_BUFFER_COMV				= 0x0000000A,
    HFI_BUFFER_NON_COMV			= 0x0000000B,
    HFI_BUFFER_PERSIST			= 0x0000000C,
    HFI_BUFFER_VPSS				= 0x0000000D,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_buffer_host_flags {
    HFI_BUF_HOST_FLAG_RELEASE		= 0x00000001,
    HFI_BUF_HOST_FLAG_READONLY		= 0x00000010,
    HFI_BUF_HOST_FLAG_CODEC_CONFIG		= 0x00000100,
    HFI_BUF_HOST_FLAGS_CB_NON_SECURE	= 0x00000200,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_buffer_firmware_flags {
    HFI_BUF_FW_FLAG_RELEASE_DONE		= 0x00000001,
    HFI_BUF_FW_FLAG_READONLY		= 0x00000010,
    HFI_BUF_FW_FLAG_LAST			= 0x10000000,
    HFI_BUF_FW_FLAG_PSC_LAST		= 0x20000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hfi_packet_firmware_flags {
    HFI_FW_FLAGS_SUCCESS			= 0x00000001,
    HFI_FW_FLAGS_INFORMATION		= 0x00000002,
    HFI_FW_FLAGS_SESSION_ERROR		= 0x00000004,
    HFI_FW_FLAGS_SYSTEM_ERROR		= 0x00000008,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfi_debug_header {
    pub size: u32,
    pub debug_level: u32,
    pub reserved: [u32; 2],
}
