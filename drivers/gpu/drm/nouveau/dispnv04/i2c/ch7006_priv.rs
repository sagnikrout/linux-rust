//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/dispnv04/i2c/ch7006_priv.h
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
// Copyright (C) 2009 Francisco Jerez.
// All Rights Reserved.
//
// Permission is hereby granted, free of charge, to any person obtaining
// a copy of this software and associated documentation files (the
// "Software"), to deal in the Software without restriction, including
// without limitation the rights to use, copy, modify, merge, publish,
// distribute, sublicense, and/or sell copies of the Software, and to
// permit persons to whom the Software is furnished to do so, subject to
// the following conditions:
//
// The above copyright notice and this permission notice (including the
// next paragraph) shall be included in all copies or substantial
// portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
// IN NO EVENT SHALL THE COPYRIGHT OWNER(S) AND/OR ITS SUPPLIERS BE
// LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
// OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION
// WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
//

pub type fixed = i64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ch7006_tv_norm {
    TV_NORM_PAL,
    TV_NORM_PAL_M,
    TV_NORM_PAL_N,
    TV_NORM_PAL_NC,
    TV_NORM_PAL_60,
    TV_NORM_NTSC_M,
    TV_NORM_NTSC_J,
    NUM_TV_NORMS
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7006_tv_norm_info {
    pub vrefresh: fixed,
    pub vdisplay: c_int,
    pub vtotal: c_int,
    pub hvirtual: c_int,
    pub subc_freq: fixed,
    pub black_level: fixed,
    pub dispmode: u32,
    pub voffset: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7006_mode {
    pub mode: drm_display_mode,
    pub enc_hdisp: c_int,
    pub enc_vdisp: c_int,
    pub subc_coeff: fixed,
    pub dispmode: u32,
    pub valid_scales: u32,
    pub valid_norms: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7006_state {
    pub regs: [u8; 0x26],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ch7006_priv {
    pub params: ch7006_encoder_params,
    pub mode: *const ch7006_mode,
    pub state: ch7006_state,
    pub saved_state: ch7006_state,
    pub scale_property: *mut drm_property,
    pub select_subconnector: c_int,
    pub subconnector: c_int,
    pub hmargin: c_int,
    pub vmargin: c_int,
    pub norm: ch7006_tv_norm,
    pub brightness: c_int,
    pub contrast: c_int,
    pub flicker: c_int,
    pub scale: c_int,
    pub chip_version: c_int,
    pub last_dpms: c_int,
}

extern "C" {
    pub fn ch7006_setup_levels(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn ch7006_setup_subcarrier(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn ch7006_setup_pll(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn ch7006_setup_power_state(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn ch7006_setup_properties(encoder: *mut drm_encoder);
}
extern "C" {
    pub fn ch7006_write(client: *mut i2c_client, addr: u8, val: u8);
}
extern "C" {
    pub fn ch7006_read(client: *mut i2c_client, addr: u8) -> u8;
}
// Some helper macros

// Fixed hardware specs
pub const CH7006_FREQ0: c_int = 14318;
pub const CH7006_MAXN: c_int = 650;
pub const CH7006_MAXM: c_int = 315;
// Register definitions
pub const CH7006_DISPMODE: c_uint = 0x00;

pub const CH7006_DISPMODE_INPUT_RES_512x384: c_uint = 0x0;
pub const CH7006_DISPMODE_INPUT_RES_720x400: c_uint = 0x1;
pub const CH7006_DISPMODE_INPUT_RES_640x400: c_uint = 0x2;
pub const CH7006_DISPMODE_INPUT_RES_640x480: c_uint = 0x3;
pub const CH7006_DISPMODE_INPUT_RES_800x600: c_uint = 0x4;
pub const CH7006_DISPMODE_INPUT_RES_NATIVE: c_uint = 0x5;

pub const CH7006_DISPMODE_OUTPUT_STD_PAL: c_uint = 0x0;
pub const CH7006_DISPMODE_OUTPUT_STD_NTSC: c_uint = 0x1;
pub const CH7006_DISPMODE_OUTPUT_STD_PAL_M: c_uint = 0x2;
pub const CH7006_DISPMODE_OUTPUT_STD_NTSC_J: c_uint = 0x3;

pub const CH7006_DISPMODE_SCALING_RATIO_5_4: c_uint = 0x0;
pub const CH7006_DISPMODE_SCALING_RATIO_1_1: c_uint = 0x1;
pub const CH7006_DISPMODE_SCALING_RATIO_7_8: c_uint = 0x2;
pub const CH7006_DISPMODE_SCALING_RATIO_5_6: c_uint = 0x3;
pub const CH7006_DISPMODE_SCALING_RATIO_3_4: c_uint = 0x4;
pub const CH7006_DISPMODE_SCALING_RATIO_7_10: c_uint = 0x5;
pub const CH7006_FFILTER: c_uint = 0x01;

pub const CH7006_FFILTER_CHROMA_NO_DCRAWL: c_uint = 0x3;
pub const CH7006_BWIDTH: c_uint = 0x03;

pub const CH7006_INPUT_FORMAT: c_uint = 0x04;

pub const CH7006_INPUT_FORMAT_FORMAT_RGB16: c_uint = 0x0;
pub const CH7006_INPUT_FORMAT_FORMAT_YCrCb24m16: c_uint = 0x1;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB24m16: c_uint = 0x2;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB15: c_uint = 0x3;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB24m12C: c_uint = 0x4;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB24m12I: c_uint = 0x5;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB24m8: c_uint = 0x6;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB16m8: c_uint = 0x7;
pub const CH7006_INPUT_FORMAT_FORMAT_RGB15m8: c_uint = 0x8;
pub const CH7006_INPUT_FORMAT_FORMAT_YCrCb24m8: c_uint = 0x9;
pub const CH7006_CLKMODE: c_uint = 0x06;

pub const CH7006_START_ACTIVE: c_uint = 0x07;

pub const CH7006_POV: c_uint = 0x08;

pub const CH7006_BLACK_LEVEL: c_uint = 0x09;

pub const CH7006_HPOS: c_uint = 0x0a;

pub const CH7006_VPOS: c_uint = 0x0b;

pub const CH7006_INPUT_SYNC: c_uint = 0x0d;

pub const CH7006_POWER: c_uint = 0x0e;

pub const CH7006_POWER_LEVEL_CVBS_OFF: c_uint = 0x0;
pub const CH7006_POWER_LEVEL_POWER_OFF: c_uint = 0x1;
pub const CH7006_POWER_LEVEL_SVIDEO_OFF: c_uint = 0x2;
pub const CH7006_POWER_LEVEL_NORMAL: c_uint = 0x3;
pub const CH7006_POWER_LEVEL_FULL_POWER_OFF: c_uint = 0x4;
pub const CH7006_DETECT: c_uint = 0x10;

pub const CH7006_CONTRAST: c_uint = 0x11;

pub const CH7006_PLLOV: c_uint = 0x13;

pub const CH7006_PLLM: c_uint = 0x14;

pub const CH7006_PLLN: c_uint = 0x15;

pub const CH7006_BCLKOUT: c_uint = 0x17;
pub const CH7006_SUBC_INC0: c_uint = 0x18;

pub const CH7006_SUBC_INC1: c_uint = 0x19;

pub const CH7006_SUBC_INC2: c_uint = 0x1a;

pub const CH7006_SUBC_INC3: c_uint = 0x1b;

pub const CH7006_SUBC_INC4: c_uint = 0x1c;

pub const CH7006_SUBC_INC5: c_uint = 0x1d;

pub const CH7006_SUBC_INC6: c_uint = 0x1e;

pub const CH7006_SUBC_INC7: c_uint = 0x1f;

pub const CH7006_PLL_CONTROL: c_uint = 0x20;

pub const CH7006_CALC_SUBC_INC0: c_uint = 0x21;

pub const CH7006_CALC_SUBC_INC1: c_uint = 0x22;

pub const CH7006_CALC_SUBC_INC2: c_uint = 0x23;

pub const CH7006_CALC_SUBC_INC3: c_uint = 0x24;

pub const CH7006_VERSION_ID: c_uint = 0x25;
