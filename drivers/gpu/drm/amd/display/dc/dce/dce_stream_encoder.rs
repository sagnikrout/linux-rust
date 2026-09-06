//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_stream_encoder.h
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


//
// Copyright 2012-15 Advanced Micro Devices, Inc.
//
// Permission is hereby granted, free of charge, to any person obtaining a
// copy of this software and associated documentation files (the "Software"),
// to deal in the Software without restriction, including without limitation
// the rights to use, copy, modify, merge, publish, distribute, sublicense,
// and/or sell copies of the Software, and to permit persons to whom the
// Software is furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
// THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
// OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
// ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
// OTHER DEALINGS IN THE SOFTWARE.
//
// Authors: AMD
//

// Macro flag: #define DCE110STRENC_FROM_STRENC(stream_encoder)\

pub const TMDS_CNTL__TMDS_PIXEL_ENCODING_MASK: c_uint = 0x00000010L;
pub const TMDS_CNTL__TMDS_COLOR_FORMAT_MASK: c_uint = 0x00000300L;
pub const TMDS_CNTL__TMDS_PIXEL_ENCODING__SHIFT: c_uint = 0x00000004;
pub const TMDS_CNTL__TMDS_COLOR_FORMAT__SHIFT: c_uint = 0x00000008;

// Macro flag: #define SE_COMMON_REG_LIST(id)\
// Macro flag: #define SE_DCN_REG_LIST(id)\

// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCE_COMMON(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_SOC(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCE80_100(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCE110(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCE112(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCE120(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCN10(mask_sh)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_stream_encoder_shift {
    pub AFMT_GENERIC_INDEX: u8,
    pub AFMT_GENERIC0_UPDATE: u8,
    pub AFMT_GENERIC2_UPDATE: u8,
    pub AFMT_GENERIC_HB0: u8,
    pub AFMT_GENERIC_HB1: u8,
    pub AFMT_GENERIC_HB2: u8,
    pub AFMT_GENERIC_HB3: u8,
    pub AFMT_GENERIC_LOCK_STATUS: u8,
    pub AFMT_GENERIC_CONFLICT: u8,
    pub AFMT_GENERIC_CONFLICT_CLR: u8,
    pub AFMT_GENERIC0_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC1_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC2_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC3_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC4_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC5_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC6_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC7_FRAME_UPDATE_PENDING: u8,
    pub AFMT_GENERIC0_FRAME_UPDATE: u8,
    pub AFMT_GENERIC1_FRAME_UPDATE: u8,
    pub AFMT_GENERIC2_FRAME_UPDATE: u8,
    pub AFMT_GENERIC3_FRAME_UPDATE: u8,
    pub AFMT_GENERIC4_FRAME_UPDATE: u8,
    pub AFMT_GENERIC5_FRAME_UPDATE: u8,
    pub AFMT_GENERIC6_FRAME_UPDATE: u8,
    pub AFMT_GENERIC7_FRAME_UPDATE: u8,
    pub HDMI_GENERIC0_CONT: u8,
    pub HDMI_GENERIC0_SEND: u8,
    pub HDMI_GENERIC0_LINE: u8,
    pub HDMI_GENERIC1_CONT: u8,
    pub HDMI_GENERIC1_SEND: u8,
    pub HDMI_GENERIC1_LINE: u8,
    pub DP_PIXEL_ENCODING: u8,
    pub DP_COMPONENT_DEPTH: u8,
    pub DP_DYN_RANGE: u8,
    pub DP_YCBCR_RANGE: u8,
    pub HDMI_PACKET_GEN_VERSION: u8,
    pub HDMI_KEEPOUT_MODE: u8,
    pub HDMI_DEEP_COLOR_ENABLE: u8,
    pub HDMI_CLOCK_CHANNEL_RATE: u8,
    pub HDMI_DEEP_COLOR_DEPTH: u8,
    pub HDMI_GC_CONT: u8,
    pub HDMI_GC_SEND: u8,
    pub HDMI_NULL_SEND: u8,
    pub HDMI_DATA_SCRAMBLE_EN: u8,
    pub HDMI_ACP_SEND: u8,
    pub HDMI_AUDIO_INFO_SEND: u8,
    pub AFMT_AUDIO_INFO_UPDATE: u8,
    pub HDMI_AUDIO_INFO_LINE: u8,
    pub HDMI_GC_AVMUTE: u8,
    pub DP_MSE_RATE_X: u8,
    pub DP_MSE_RATE_Y: u8,
    pub DP_MSE_RATE_UPDATE_PENDING: u8,
    pub AFMT_AVI_INFO_VERSION: u8,
    pub HDMI_AVI_INFO_SEND: u8,
    pub HDMI_AVI_INFO_CONT: u8,
    pub HDMI_AVI_INFO_LINE: u8,
    pub DP_SEC_GSP0_ENABLE: u8,
    pub DP_SEC_STREAM_ENABLE: u8,
    pub DP_SEC_GSP1_ENABLE: u8,
    pub DP_SEC_GSP2_ENABLE: u8,
    pub DP_SEC_GSP3_ENABLE: u8,
    pub DP_SEC_GSP4_ENABLE: u8,
    pub DP_SEC_GSP5_ENABLE: u8,
    pub DP_SEC_GSP6_ENABLE: u8,
    pub DP_SEC_GSP7_ENABLE: u8,
    pub DP_SEC_AVI_ENABLE: u8,
    pub DP_SEC_MPG_ENABLE: u8,
    pub DP_VID_STREAM_DIS_DEFER: u8,
    pub DP_VID_STREAM_ENABLE: u8,
    pub DP_VID_STREAM_STATUS: u8,
    pub DP_STEER_FIFO_RESET: u8,
    pub DP_VID_M_N_GEN_EN: u8,
    pub DP_VID_N: u8,
    pub DP_VID_M: u8,
    pub DIG_START: u8,
    pub AFMT_AUDIO_SRC_SELECT: u8,
    pub AFMT_AUDIO_CHANNEL_ENABLE: u8,
    pub HDMI_AUDIO_PACKETS_PER_LINE: u8,
    pub HDMI_AUDIO_DELAY_EN: u8,
    pub AFMT_60958_CS_UPDATE: u8,
    pub AFMT_AUDIO_LAYOUT_OVRD: u8,
    pub AFMT_60958_OSF_OVRD: u8,
    pub HDMI_ACR_AUTO_SEND: u8,
    pub HDMI_ACR_SOURCE: u8,
    pub HDMI_ACR_AUDIO_PRIORITY: u8,
    pub HDMI_ACR_CTS_32: u8,
    pub HDMI_ACR_N_32: u8,
    pub HDMI_ACR_CTS_44: u8,
    pub HDMI_ACR_N_44: u8,
    pub HDMI_ACR_CTS_48: u8,
    pub HDMI_ACR_N_48: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_L: u8,
    pub AFMT_60958_CS_CLOCK_ACCURACY: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_R: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_2: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_3: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_4: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_5: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_6: u8,
    pub AFMT_60958_CS_CHANNEL_NUMBER_7: u8,
    pub DP_SEC_AUD_N: u8,
    pub DP_SEC_TIMESTAMP_MODE: u8,
    pub DP_SEC_ASP_ENABLE: u8,
    pub DP_SEC_ATP_ENABLE: u8,
    pub DP_SEC_AIP_ENABLE: u8,
    pub DP_SEC_ACM_ENABLE: u8,
    pub AFMT_AUDIO_SAMPLE_SEND: u8,
    pub AFMT_AUDIO_CLOCK_EN: u8,
    pub TMDS_PIXEL_ENCODING: u8,
    pub TMDS_COLOR_FORMAT: u8,
    pub DIG_STEREOSYNC_SELECT: u8,
    pub DIG_STEREOSYNC_GATE_EN: u8,
    pub DP_DB_DISABLE: u8,
    pub DP_MSA_MISC0: u8,
    pub DP_MSA_HTOTAL: u8,
    pub DP_MSA_VTOTAL: u8,
    pub DP_MSA_HSTART: u8,
    pub DP_MSA_VSTART: u8,
    pub DP_MSA_HSYNCWIDTH: u8,
    pub DP_MSA_HSYNCPOLARITY: u8,
    pub DP_MSA_VSYNCWIDTH: u8,
    pub DP_MSA_VSYNCPOLARITY: u8,
    pub DP_MSA_HWIDTH: u8,
    pub DP_MSA_VHEIGHT: u8,
    pub HDMI_DB_DISABLE: u8,
    pub DP_VID_N_MUL: u8,
    pub DP_VID_M_DOUBLE_VALUE_EN: u8,
    pub DIG_SOURCE_SELECT: u8,
    pub DAC_SOURCE_SELECT: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_stream_encoder_mask {
    pub AFMT_GENERIC_INDEX: u32,
    pub AFMT_GENERIC0_UPDATE: u32,
    pub AFMT_GENERIC2_UPDATE: u32,
    pub AFMT_GENERIC_HB0: u32,
    pub AFMT_GENERIC_HB1: u32,
    pub AFMT_GENERIC_HB2: u32,
    pub AFMT_GENERIC_HB3: u32,
    pub AFMT_GENERIC_LOCK_STATUS: u32,
    pub AFMT_GENERIC_CONFLICT: u32,
    pub AFMT_GENERIC_CONFLICT_CLR: u32,
    pub AFMT_GENERIC0_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC1_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC2_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC3_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC4_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC5_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC6_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC7_FRAME_UPDATE_PENDING: u32,
    pub AFMT_GENERIC0_FRAME_UPDATE: u32,
    pub AFMT_GENERIC1_FRAME_UPDATE: u32,
    pub AFMT_GENERIC2_FRAME_UPDATE: u32,
    pub AFMT_GENERIC3_FRAME_UPDATE: u32,
    pub AFMT_GENERIC4_FRAME_UPDATE: u32,
    pub AFMT_GENERIC5_FRAME_UPDATE: u32,
    pub AFMT_GENERIC6_FRAME_UPDATE: u32,
    pub AFMT_GENERIC7_FRAME_UPDATE: u32,
    pub HDMI_GENERIC0_CONT: u32,
    pub HDMI_GENERIC0_SEND: u32,
    pub HDMI_GENERIC0_LINE: u32,
    pub HDMI_GENERIC1_CONT: u32,
    pub HDMI_GENERIC1_SEND: u32,
    pub HDMI_GENERIC1_LINE: u32,
    pub DP_PIXEL_ENCODING: u32,
    pub DP_COMPONENT_DEPTH: u32,
    pub DP_DYN_RANGE: u32,
    pub DP_YCBCR_RANGE: u32,
    pub HDMI_PACKET_GEN_VERSION: u32,
    pub HDMI_KEEPOUT_MODE: u32,
    pub HDMI_DEEP_COLOR_ENABLE: u32,
    pub HDMI_CLOCK_CHANNEL_RATE: u32,
    pub HDMI_DEEP_COLOR_DEPTH: u32,
    pub HDMI_GC_CONT: u32,
    pub HDMI_GC_SEND: u32,
    pub HDMI_NULL_SEND: u32,
    pub HDMI_DATA_SCRAMBLE_EN: u32,
    pub HDMI_ACP_SEND: u32,
    pub HDMI_AUDIO_INFO_SEND: u32,
    pub AFMT_AUDIO_INFO_UPDATE: u32,
    pub HDMI_AUDIO_INFO_LINE: u32,
    pub HDMI_GC_AVMUTE: u32,
    pub DP_MSE_RATE_X: u32,
    pub DP_MSE_RATE_Y: u32,
    pub DP_MSE_RATE_UPDATE_PENDING: u32,
    pub AFMT_AVI_INFO_VERSION: u32,
    pub HDMI_AVI_INFO_SEND: u32,
    pub HDMI_AVI_INFO_CONT: u32,
    pub HDMI_AVI_INFO_LINE: u32,
    pub DP_SEC_GSP0_ENABLE: u32,
    pub DP_SEC_STREAM_ENABLE: u32,
    pub DP_SEC_GSP1_ENABLE: u32,
    pub DP_SEC_GSP2_ENABLE: u32,
    pub DP_SEC_GSP3_ENABLE: u32,
    pub DP_SEC_GSP4_ENABLE: u32,
    pub DP_SEC_GSP5_ENABLE: u32,
    pub DP_SEC_GSP6_ENABLE: u32,
    pub DP_SEC_GSP7_ENABLE: u32,
    pub DP_SEC_AVI_ENABLE: u32,
    pub DP_SEC_MPG_ENABLE: u32,
    pub DP_VID_STREAM_DIS_DEFER: u32,
    pub DP_VID_STREAM_ENABLE: u32,
    pub DP_VID_STREAM_STATUS: u32,
    pub DP_STEER_FIFO_RESET: u32,
    pub DP_VID_M_N_GEN_EN: u32,
    pub DP_VID_N: u32,
    pub DP_VID_M: u32,
    pub DIG_START: u32,
    pub AFMT_AUDIO_SRC_SELECT: u32,
    pub AFMT_AUDIO_CHANNEL_ENABLE: u32,
    pub HDMI_AUDIO_PACKETS_PER_LINE: u32,
    pub HDMI_AUDIO_DELAY_EN: u32,
    pub AFMT_60958_CS_UPDATE: u32,
    pub AFMT_AUDIO_LAYOUT_OVRD: u32,
    pub AFMT_60958_OSF_OVRD: u32,
    pub HDMI_ACR_AUTO_SEND: u32,
    pub HDMI_ACR_SOURCE: u32,
    pub HDMI_ACR_AUDIO_PRIORITY: u32,
    pub HDMI_ACR_CTS_32: u32,
    pub HDMI_ACR_N_32: u32,
    pub HDMI_ACR_CTS_44: u32,
    pub HDMI_ACR_N_44: u32,
    pub HDMI_ACR_CTS_48: u32,
    pub HDMI_ACR_N_48: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_L: u32,
    pub AFMT_60958_CS_CLOCK_ACCURACY: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_R: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_2: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_3: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_4: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_5: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_6: u32,
    pub AFMT_60958_CS_CHANNEL_NUMBER_7: u32,
    pub DP_SEC_AUD_N: u32,
    pub DP_SEC_TIMESTAMP_MODE: u32,
    pub DP_SEC_ASP_ENABLE: u32,
    pub DP_SEC_ATP_ENABLE: u32,
    pub DP_SEC_AIP_ENABLE: u32,
    pub DP_SEC_ACM_ENABLE: u32,
    pub AFMT_AUDIO_SAMPLE_SEND: u32,
    pub AFMT_AUDIO_CLOCK_EN: u32,
    pub TMDS_PIXEL_ENCODING: u32,
    pub DIG_STEREOSYNC_SELECT: u32,
    pub DIG_STEREOSYNC_GATE_EN: u32,
    pub TMDS_COLOR_FORMAT: u32,
    pub DP_DB_DISABLE: u32,
    pub DP_MSA_MISC0: u32,
    pub DP_MSA_HTOTAL: u32,
    pub DP_MSA_VTOTAL: u32,
    pub DP_MSA_HSTART: u32,
    pub DP_MSA_VSTART: u32,
    pub DP_MSA_HSYNCWIDTH: u32,
    pub DP_MSA_HSYNCPOLARITY: u32,
    pub DP_MSA_VSYNCWIDTH: u32,
    pub DP_MSA_VSYNCPOLARITY: u32,
    pub DP_MSA_HWIDTH: u32,
    pub DP_MSA_VHEIGHT: u32,
    pub HDMI_DB_DISABLE: u32,
    pub DP_VID_N_MUL: u32,
    pub DP_VID_M_DOUBLE_VALUE_EN: u32,
    pub DIG_SOURCE_SELECT: u32,
    pub DAC_SOURCE_SELECT: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_stream_enc_registers {
    pub AFMT_CNTL: u32,
    pub AFMT_AVI_INFO0: u32,
    pub AFMT_AVI_INFO1: u32,
    pub AFMT_AVI_INFO2: u32,
    pub AFMT_AVI_INFO3: u32,
    pub AFMT_GENERIC_0: u32,
    pub AFMT_GENERIC_1: u32,
    pub AFMT_GENERIC_2: u32,
    pub AFMT_GENERIC_3: u32,
    pub AFMT_GENERIC_4: u32,
    pub AFMT_GENERIC_5: u32,
    pub AFMT_GENERIC_6: u32,
    pub AFMT_GENERIC_7: u32,
    pub AFMT_GENERIC_HDR: u32,
    pub AFMT_INFOFRAME_CONTROL0: u32,
    pub AFMT_VBI_PACKET_CONTROL: u32,
    pub AFMT_VBI_PACKET_CONTROL1: u32,
    pub AFMT_AUDIO_PACKET_CONTROL: u32,
    pub AFMT_AUDIO_PACKET_CONTROL2: u32,
    pub AFMT_AUDIO_SRC_CONTROL: u32,
    pub AFMT_60958_0: u32,
    pub AFMT_60958_1: u32,
    pub AFMT_60958_2: u32,
    pub DIG_FE_CNTL: u32,
    pub DAC_SOURCE_SELECT: u32,
    pub DP_MSE_RATE_CNTL: u32,
    pub DP_MSE_RATE_UPDATE: u32,
    pub DP_PIXEL_FORMAT: u32,
    pub DP_SEC_CNTL: u32,
    pub DP_STEER_FIFO: u32,
    pub DP_VID_M: u32,
    pub DP_VID_N: u32,
    pub DP_VID_STREAM_CNTL: u32,
    pub DP_VID_TIMING: u32,
    pub DP_SEC_AUD_N: u32,
    pub DP_SEC_TIMESTAMP: u32,
    pub HDMI_CONTROL: u32,
    pub HDMI_GC: u32,
    pub HDMI_GENERIC_PACKET_CONTROL0: u32,
    pub HDMI_GENERIC_PACKET_CONTROL1: u32,
    pub HDMI_GENERIC_PACKET_CONTROL2: u32,
    pub HDMI_GENERIC_PACKET_CONTROL3: u32,
    pub HDMI_INFOFRAME_CONTROL0: u32,
    pub HDMI_INFOFRAME_CONTROL1: u32,
    pub HDMI_VBI_PACKET_CONTROL: u32,
    pub HDMI_AUDIO_PACKET_CONTROL: u32,
    pub HDMI_ACR_PACKET_CONTROL: u32,
    pub HDMI_ACR_32_0: u32,
    pub HDMI_ACR_32_1: u32,
    pub HDMI_ACR_44_0: u32,
    pub HDMI_ACR_44_1: u32,
    pub HDMI_ACR_48_0: u32,
    pub HDMI_ACR_48_1: u32,
    pub TMDS_CNTL: u32,
    pub DP_DB_CNTL: u32,
    pub DP_MSA_MISC: u32,
    pub DP_MSA_COLORIMETRY: u32,
    pub DP_MSA_TIMING_PARAM1: u32,
    pub DP_MSA_TIMING_PARAM2: u32,
    pub DP_MSA_TIMING_PARAM3: u32,
    pub DP_MSA_TIMING_PARAM4: u32,
    pub HDMI_DB_CONTROL: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce110_stream_encoder {
    pub base: stream_encoder,
    pub regs: *const dce110_stream_enc_registers,
    pub se_shift: *const dce_stream_encoder_shift,
    pub se_mask: *const dce_stream_encoder_mask,
}
