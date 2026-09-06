//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dio/dcn10/dcn10_stream_encoder.h
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
// Copyright 2012-2026 Advanced Micro Devices, Inc.
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

// Macro flag: #define DCN10STRENC_FROM_STRENC(stream_encoder)\

// Macro flag: #define SE_DCN_REG_LIST(id)\
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_stream_enc_registers {
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
    pub DIG_FIFO_STATUS: u32,
    pub DP_MSE_RATE_CNTL: u32,
    pub DP_MSE_RATE_UPDATE: u32,
    pub DP_PIXEL_FORMAT: u32,
    pub DP_SEC_CNTL: u32,
    pub DP_SEC_CNTL1: u32,
    pub DP_SEC_CNTL2: u32,
    pub DP_SEC_CNTL5: u32,
    pub DP_SEC_CNTL6: u32,
    pub DP_STEER_FIFO: u32,
    pub DP_VID_M: u32,
    pub DP_VID_N: u32,
    pub DP_VID_STREAM_CNTL: u32,
    pub DP_VID_TIMING: u32,
    pub DP_SEC_AUD_N: u32,
    pub DP_SEC_AUD_N_READBACK: u32,
    pub DP_SEC_AUD_M_READBACK: u32,
    pub DP_SEC_TIMESTAMP: u32,
    pub HDMI_CONTROL: u32,
    pub HDMI_GC: u32,
    pub HDMI_GENERIC_PACKET_CONTROL0: u32,
    pub HDMI_GENERIC_PACKET_CONTROL1: u32,
    pub HDMI_GENERIC_PACKET_CONTROL2: u32,
    pub HDMI_GENERIC_PACKET_CONTROL3: u32,
    pub HDMI_GENERIC_PACKET_CONTROL4: u32,
    pub HDMI_GENERIC_PACKET_CONTROL5: u32,
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
    pub DP_DB_CNTL: u32,
    pub DP_MSA_MISC: u32,
    pub DP_MSA_VBID_MISC: u32,
    pub DP_MSA_COLORIMETRY: u32,
    pub DP_MSA_TIMING_PARAM1: u32,
    pub DP_MSA_TIMING_PARAM2: u32,
    pub DP_MSA_TIMING_PARAM3: u32,
    pub DP_MSA_TIMING_PARAM4: u32,
    pub HDMI_DB_CONTROL: u32,
    pub DP_DSC_CNTL: u32,
    pub DP_DSC_BYTES_PER_PIXEL: u32,
    pub DME_CONTROL: u32,
    pub DP_SEC_METADATA_TRANSMISSION: u32,
    pub HDMI_METADATA_PACKET_CONTROL: u32,
    pub DP_SEC_FRAMING4: u32,
    pub DP_GSP11_CNTL: u32,
    pub HDMI_GENERIC_PACKET_CONTROL6: u32,
    pub HDMI_GENERIC_PACKET_CONTROL7: u32,
    pub HDMI_GENERIC_PACKET_CONTROL8: u32,
    pub HDMI_GENERIC_PACKET_CONTROL9: u32,
    pub HDMI_GENERIC_PACKET_CONTROL10: u32,
    pub DIG_CLOCK_PATTERN: u32,
    pub DIG_FIFO_CTRL0: u32,
    pub DIG_FE_CLK_CNTL: u32,
    pub DIG_FE_EN_CNTL: u32,
    pub STREAM_MAPPER_CONTROL: u32,
    pub DIG_FE_AUDIO_CNTL: u32,
}

// Macro flag: #define SE_COMMON_MASK_SH_LIST_SOC(mask_sh)\
// Macro flag: #define SE_COMMON_MASK_SH_LIST_DCN10(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_stream_encoder_shift {
    pub HDMI_ACP_SEND: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_stream_encoder_mask {
    pub HDMI_ACP_SEND: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dcn10_stream_encoder {
    pub base: stream_encoder,
    pub regs: *const dcn10_stream_enc_registers,
    pub se_shift: *const dcn10_stream_encoder_shift,
    pub se_mask: *const dcn10_stream_encoder_mask,
}
