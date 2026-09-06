//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/amd/display/dc/dce/dce_audio.h
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

// Macro flag: #define AUD_COMMON_REG_LIST(id)\
// set field name

// Macro flag: #define AUD_COMMON_MASK_SH_LIST_BASE(mask_sh)\
// Macro flag: #define AUD_COMMON_MASK_SH_LIST(mask_sh)\

// Macro flag: #define AUD_DCE60_MASK_SH_LIST(mask_sh)\

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_audio_registers {
    pub AZALIA_F0_CODEC_ENDPOINT_INDEX: u32,
    pub AZALIA_F0_CODEC_ENDPOINT_DATA: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_STREAM_FORMATS: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_SUPPORTED_SIZE_RATES: u32,
    pub AZALIA_F0_CODEC_FUNCTION_PARAMETER_POWER_STATES: u32,
    pub DCCG_AUDIO_DTO_SOURCE: u32,
    pub DCCG_AUDIO_DTO0_MODULE: u32,
    pub DCCG_AUDIO_DTO0_PHASE: u32,
    pub DCCG_AUDIO_DTO1_MODULE: u32,
    pub DCCG_AUDIO_DTO1_PHASE: u32,
    pub AUDIO_RATE_CAPABILITIES: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_audio_shift {
    pub AZALIA_ENDPOINT_REG_INDEX: u8,
    pub AZALIA_ENDPOINT_REG_DATA: u8,
    pub AUDIO_RATE_CAPABILITIES: u8,
    pub CLKSTOP: u8,
    pub EPSS: u8,
    pub DCCG_AUDIO_DTO0_SOURCE_SEL: u8,
    pub DCCG_AUDIO_DTO_SEL: u8,
    pub DCCG_AUDIO_DTO0_MODULE: u8,
    pub DCCG_AUDIO_DTO0_PHASE: u8,
    pub DCCG_AUDIO_DTO1_MODULE: u8,
    pub DCCG_AUDIO_DTO1_PHASE: u8,
    pub DCCG_AUDIO_DTO2_USE_512FBR_DTO: u8,
    pub DCCG_AUDIO_DTO0_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO1_USE_512FBR_DTO: u32,
    pub CLOCK_GATING_DISABLE: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_audio_mask {
    pub AZALIA_ENDPOINT_REG_INDEX: u32,
    pub AZALIA_ENDPOINT_REG_DATA: u32,
    pub AUDIO_RATE_CAPABILITIES: u32,
    pub CLKSTOP: u32,
    pub EPSS: u32,
    pub DCCG_AUDIO_DTO0_SOURCE_SEL: u32,
    pub DCCG_AUDIO_DTO_SEL: u32,
    pub DCCG_AUDIO_DTO0_MODULE: u32,
    pub DCCG_AUDIO_DTO0_PHASE: u32,
    pub DCCG_AUDIO_DTO1_MODULE: u32,
    pub DCCG_AUDIO_DTO1_PHASE: u32,
    pub DCCG_AUDIO_DTO2_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO0_USE_512FBR_DTO: u32,
    pub DCCG_AUDIO_DTO1_USE_512FBR_DTO: u32,
    pub CLOCK_GATING_DISABLE: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dce_audio {
    pub base: audio,
    pub regs: *const dce_audio_registers,
    pub shifts: *const dce_audio_shift,
    pub masks: *const dce_audio_mask,
}

extern "C" {
    pub fn dce_aud_destroy(audio: *mut audio);
}
extern "C" {
    pub fn dce_aud_hw_init(audio: *mut audio);
}
extern "C" {
    pub fn dce_aud_az_enable(audio: *mut audio);
}
extern "C" {
    pub fn dce_aud_az_disable(audio: *mut audio);
}
extern "C" {
    pub fn dce_aud_az_disable_hbr_audio(audio: *mut audio);
}
