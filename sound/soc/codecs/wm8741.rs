//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm8741.h
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
// wm8741.h  --  WM8423 ASoC driver
//
// Copyright 2010 Wolfson Microelectronics, plc
//
// Author: Ian Lartey <ian@opensource.wolfsonmicro.com>
//
// Based on wm8753.h
//
// Register values.
//
pub const WM8741_DACLLSB_ATTENUATION: c_uint = 0x00;
pub const WM8741_DACLMSB_ATTENUATION: c_uint = 0x01;
pub const WM8741_DACRLSB_ATTENUATION: c_uint = 0x02;
pub const WM8741_DACRMSB_ATTENUATION: c_uint = 0x03;
pub const WM8741_VOLUME_CONTROL: c_uint = 0x04;
pub const WM8741_FORMAT_CONTROL: c_uint = 0x05;
pub const WM8741_FILTER_CONTROL: c_uint = 0x06;
pub const WM8741_MODE_CONTROL_1: c_uint = 0x07;
pub const WM8741_MODE_CONTROL_2: c_uint = 0x08;
pub const WM8741_RESET: c_uint = 0x09;
pub const WM8741_ADDITIONAL_CONTROL_1: c_uint = 0x20;
pub const WM8741_REGISTER_COUNT: c_int = 11;
pub const WM8741_MAX_REGISTER: c_uint = 0x20;
//
// Field Definitions.
//
// R0 (0x00) - DACLLSB_ATTENUATION
//
pub const WM8741_UPDATELL: c_uint = 0x0020  /* UPDATELL */;
pub const WM8741_UPDATELL_MASK: c_uint = 0x0020  /* UPDATELL */;

pub const WM8741_LAT_4_0_MASK: c_uint = 0x001F  /* LAT[4:0] - [4:0] */;

//
// R1 (0x01) - DACLMSB_ATTENUATION
//
pub const WM8741_UPDATELM: c_uint = 0x0020  /* UPDATELM */;
pub const WM8741_UPDATELM_MASK: c_uint = 0x0020  /* UPDATELM */;

pub const WM8741_LAT_9_5_0_MASK: c_uint = 0x001F  /* LAT[9:5] - [4:0] */;

//
// R2 (0x02) - DACRLSB_ATTENUATION
//
pub const WM8741_UPDATERL: c_uint = 0x0020  /* UPDATERL */;
pub const WM8741_UPDATERL_MASK: c_uint = 0x0020  /* UPDATERL */;

pub const WM8741_RAT_4_0_MASK: c_uint = 0x001F  /* RAT[4:0] - [4:0] */;

//
// R3 (0x03) - DACRMSB_ATTENUATION
//
pub const WM8741_UPDATERM: c_uint = 0x0020  /* UPDATERM */;
pub const WM8741_UPDATERM_MASK: c_uint = 0x0020  /* UPDATERM */;

pub const WM8741_RAT_9_5_0_MASK: c_uint = 0x001F  /* RAT[9:5] - [4:0] */;

//
// R4 (0x04) - VOLUME_CONTROL
//
pub const WM8741_AMUTE: c_uint = 0x0080  /* AMUTE */;
pub const WM8741_AMUTE_MASK: c_uint = 0x0080  /* AMUTE */;

pub const WM8741_ZFLAG_MASK: c_uint = 0x0060  /* ZFLAG - [6:5] */;

pub const WM8741_IZD: c_uint = 0x0010  /* IZD */;
pub const WM8741_IZD_MASK: c_uint = 0x0010  /* IZD */;

pub const WM8741_SOFT: c_uint = 0x0008  /* SOFT MUTE */;
pub const WM8741_SOFT_MASK: c_uint = 0x0008  /* SOFT MUTE */;

pub const WM8741_ATC: c_uint = 0x0004  /* ATC */;
pub const WM8741_ATC_MASK: c_uint = 0x0004  /* ATC */;

pub const WM8741_ATT2DB: c_uint = 0x0002  /* ATT2DB */;
pub const WM8741_ATT2DB_MASK: c_uint = 0x0002  /* ATT2DB */;

pub const WM8741_VOL_RAMP: c_uint = 0x0001  /* VOL_RAMP */;
pub const WM8741_VOL_RAMP_MASK: c_uint = 0x0001  /* VOL_RAMP */;

//
// R5 (0x05) - FORMAT_CONTROL
//
pub const WM8741_PWDN: c_uint = 0x0080  /* PWDN */;
pub const WM8741_PWDN_MASK: c_uint = 0x0080  /* PWDN */;

pub const WM8741_REV: c_uint = 0x0040  /* REV */;
pub const WM8741_REV_MASK: c_uint = 0x0040  /* REV */;

pub const WM8741_BCP: c_uint = 0x0020  /* BCP */;
pub const WM8741_BCP_MASK: c_uint = 0x0020  /* BCP */;

pub const WM8741_LRP: c_uint = 0x0010  /* LRP */;
pub const WM8741_LRP_MASK: c_uint = 0x0010  /* LRP */;

pub const WM8741_FMT_MASK: c_uint = 0x000C  /* FMT - [3:2] */;

pub const WM8741_IWL_MASK: c_uint = 0x0003  /* IWL - [1:0] */;

//
// R6 (0x06) - FILTER_CONTROL
//
pub const WM8741_ZFLAG_HI: c_uint = 0x0080  /* ZFLAG_HI */;
pub const WM8741_ZFLAG_HI_MASK: c_uint = 0x0080  /* ZFLAG_HI */;

pub const WM8741_DEEMPH_MASK: c_uint = 0x0060  /* DEEMPH - [6:5] */;

pub const WM8741_DSDFILT_MASK: c_uint = 0x0018  /* DSDFILT - [4:3] */;

pub const WM8741_FIRSEL_MASK: c_uint = 0x0007  /* FIRSEL - [2:0] */;

//
// R7 (0x07) - MODE_CONTROL_1
//
pub const WM8741_MODE8X: c_uint = 0x0080  /* MODE8X */;
pub const WM8741_MODE8X_MASK: c_uint = 0x0080  /* MODE8X */;

pub const WM8741_OSR_MASK: c_uint = 0x0060  /* OSR - [6:5] */;

pub const WM8741_SR_MASK: c_uint = 0x001C  /* SR - [4:2] */;

pub const WM8741_MODESEL_MASK: c_uint = 0x0003  /* MODESEL - [1:0] */;

//
// R8 (0x08) - MODE_CONTROL_2
//
pub const WM8741_DSD_GAIN: c_uint = 0x0040  /* DSD_GAIN */;
pub const WM8741_DSD_GAIN_MASK: c_uint = 0x0040  /* DSD_GAIN */;

pub const WM8741_SDOUT: c_uint = 0x0020  /* SDOUT */;
pub const WM8741_SDOUT_MASK: c_uint = 0x0020  /* SDOUT */;

pub const WM8741_DOUT: c_uint = 0x0010  /* DOUT */;
pub const WM8741_DOUT_MASK: c_uint = 0x0010  /* DOUT */;

pub const WM8741_DIFF_MASK: c_uint = 0x000C  /* DIFF - [3:2] */;

pub const WM8741_DITHER_MASK: c_uint = 0x0003  /* DITHER - [1:0] */;

// DIFF field values

//
// R32 (0x20) - ADDITONAL_CONTROL_1
//
pub const WM8741_DSD_LEVEL: c_uint = 0x0002  /* DSD_LEVEL */;
pub const WM8741_DSD_LEVEL_MASK: c_uint = 0x0002  /* DSD_LEVEL */;

pub const WM8741_DSD_NO_NOTCH: c_uint = 0x0001  /* DSD_NO_NOTCH */;
pub const WM8741_DSD_NO_NOTCH_MASK: c_uint = 0x0001  /* DSD_NO_NOTCH */;

pub const WM8741_SYSCLK: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm8741_platform_data {
    pub /: *mut *mut u32 diff_mode; / Differential Output Mode,
}
