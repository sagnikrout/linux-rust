//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/tvp7002_reg.h
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
// Texas Instruments Triple 8-/10-BIT 165-/110-MSPS Video and Graphics
// Digitizer with Horizontal PLL registers
//
// Copyright (C) 2009 Texas Instruments Inc
// Author: Santiago Nunez-Corrales <santiago.nunez@ridgerun.com>
//
// This code is partially based upon the TVP5150 driver
// written by Mauro Carvalho Chehab <mchehab@kernel.org>,
// the TVP514x driver written by Vaibhav Hiremath <hvaibhav@ti.com>
// and the TVP7002 driver in the TI LSP 2.10.00.14
//
// Naming conventions
// ------------------
//
// FDBK:  Feedback
// DIV:   Divider
// CTL:   Control
// SEL:   Select
// IN:    Input
// OUT:   Output
// R:     Red
// G:     Green
// B:     Blue
// OFF:   Offset
// THRS:  Threshold
// DGTL:  Digital
// LVL:   Level
// PWR:   Power
// MVIS:  Macrovision
// W:     Width
// H:     Height
// ALGN:  Alignment
// CLK:   Clocks
// TOL:   Tolerance
// BWTH:  Bandwidth
// COEF:  Coefficient
// STAT:  Status
// AUTO:  Automatic
// FLD:   Field
// L:	  Line
//
pub const TVP7002_CHIP_REV: c_uint = 0x00;
pub const TVP7002_HPLL_FDBK_DIV_MSBS: c_uint = 0x01;
pub const TVP7002_HPLL_FDBK_DIV_LSBS: c_uint = 0x02;
pub const TVP7002_HPLL_CRTL: c_uint = 0x03;
pub const TVP7002_HPLL_PHASE_SEL: c_uint = 0x04;
pub const TVP7002_CLAMP_START: c_uint = 0x05;
pub const TVP7002_CLAMP_W: c_uint = 0x06;
pub const TVP7002_HSYNC_OUT_W: c_uint = 0x07;
pub const TVP7002_B_FINE_GAIN: c_uint = 0x08;
pub const TVP7002_G_FINE_GAIN: c_uint = 0x09;
pub const TVP7002_R_FINE_GAIN: c_uint = 0x0a;
pub const TVP7002_B_FINE_OFF_MSBS: c_uint = 0x0b;
pub const TVP7002_G_FINE_OFF_MSBS: c_uint = 0x0c;
pub const TVP7002_R_FINE_OFF_MSBS: c_uint = 0x0d;
pub const TVP7002_SYNC_CTL_1: c_uint = 0x0e;
pub const TVP7002_HPLL_AND_CLAMP_CTL: c_uint = 0x0f;
pub const TVP7002_SYNC_ON_G_THRS: c_uint = 0x10;
pub const TVP7002_SYNC_SEPARATOR_THRS: c_uint = 0x11;
pub const TVP7002_HPLL_PRE_COAST: c_uint = 0x12;
pub const TVP7002_HPLL_POST_COAST: c_uint = 0x13;
pub const TVP7002_SYNC_DETECT_STAT: c_uint = 0x14;
pub const TVP7002_OUT_FORMATTER: c_uint = 0x15;
pub const TVP7002_MISC_CTL_1: c_uint = 0x16;
pub const TVP7002_MISC_CTL_2: c_uint = 0x17;
pub const TVP7002_MISC_CTL_3: c_uint = 0x18;
pub const TVP7002_IN_MUX_SEL_1: c_uint = 0x19;
pub const TVP7002_IN_MUX_SEL_2: c_uint = 0x1a;
pub const TVP7002_B_AND_G_COARSE_GAIN: c_uint = 0x1b;
pub const TVP7002_R_COARSE_GAIN: c_uint = 0x1c;
pub const TVP7002_FINE_OFF_LSBS: c_uint = 0x1d;
pub const TVP7002_B_COARSE_OFF: c_uint = 0x1e;
pub const TVP7002_G_COARSE_OFF: c_uint = 0x1f;
pub const TVP7002_R_COARSE_OFF: c_uint = 0x20;
pub const TVP7002_HSOUT_OUT_START: c_uint = 0x21;
pub const TVP7002_MISC_CTL_4: c_uint = 0x22;
pub const TVP7002_B_DGTL_ALC_OUT_LSBS: c_uint = 0x23;
pub const TVP7002_G_DGTL_ALC_OUT_LSBS: c_uint = 0x24;
pub const TVP7002_R_DGTL_ALC_OUT_LSBS: c_uint = 0x25;
pub const TVP7002_AUTO_LVL_CTL_ENABLE: c_uint = 0x26;
pub const TVP7002_DGTL_ALC_OUT_MSBS: c_uint = 0x27;
pub const TVP7002_AUTO_LVL_CTL_FILTER: c_uint = 0x28;
// Reserved 0x29
pub const TVP7002_FINE_CLAMP_CTL: c_uint = 0x2a;
pub const TVP7002_PWR_CTL: c_uint = 0x2b;
pub const TVP7002_ADC_SETUP: c_uint = 0x2c;
pub const TVP7002_COARSE_CLAMP_CTL: c_uint = 0x2d;
pub const TVP7002_SOG_CLAMP: c_uint = 0x2e;
pub const TVP7002_RGB_COARSE_CLAMP_CTL: c_uint = 0x2f;
pub const TVP7002_SOG_COARSE_CLAMP_CTL: c_uint = 0x30;
pub const TVP7002_ALC_PLACEMENT: c_uint = 0x31;
// Reserved 0x32
// Reserved 0x33
pub const TVP7002_MVIS_STRIPPER_W: c_uint = 0x34;
pub const TVP7002_VSYNC_ALGN: c_uint = 0x35;
pub const TVP7002_SYNC_BYPASS: c_uint = 0x36;
pub const TVP7002_L_FRAME_STAT_LSBS: c_uint = 0x37;
pub const TVP7002_L_FRAME_STAT_MSBS: c_uint = 0x38;
pub const TVP7002_CLK_L_STAT_LSBS: c_uint = 0x39;
pub const TVP7002_CLK_L_STAT_MSBS: c_uint = 0x3a;
pub const TVP7002_HSYNC_W: c_uint = 0x3b;
pub const TVP7002_VSYNC_W: c_uint = 0x3c;
pub const TVP7002_L_LENGTH_TOL: c_uint = 0x3d;
// Reserved 0x3e
pub const TVP7002_VIDEO_BWTH_CTL: c_uint = 0x3f;
pub const TVP7002_AVID_START_PIXEL_LSBS: c_uint = 0x40;
pub const TVP7002_AVID_START_PIXEL_MSBS: c_uint = 0x41;
pub const TVP7002_AVID_STOP_PIXEL_LSBS: c_uint = 0x42;
pub const TVP7002_AVID_STOP_PIXEL_MSBS: c_uint = 0x43;
pub const TVP7002_VBLK_F_0_START_L_OFF: c_uint = 0x44;
pub const TVP7002_VBLK_F_1_START_L_OFF: c_uint = 0x45;
pub const TVP7002_VBLK_F_0_DURATION: c_uint = 0x46;
pub const TVP7002_VBLK_F_1_DURATION: c_uint = 0x47;
pub const TVP7002_FBIT_F_0_START_L_OFF: c_uint = 0x48;
pub const TVP7002_FBIT_F_1_START_L_OFF: c_uint = 0x49;
pub const TVP7002_YUV_Y_G_COEF_LSBS: c_uint = 0x4a;
pub const TVP7002_YUV_Y_G_COEF_MSBS: c_uint = 0x4b;
pub const TVP7002_YUV_Y_B_COEF_LSBS: c_uint = 0x4c;
pub const TVP7002_YUV_Y_B_COEF_MSBS: c_uint = 0x4d;
pub const TVP7002_YUV_Y_R_COEF_LSBS: c_uint = 0x4e;
pub const TVP7002_YUV_Y_R_COEF_MSBS: c_uint = 0x4f;
pub const TVP7002_YUV_U_G_COEF_LSBS: c_uint = 0x50;
pub const TVP7002_YUV_U_G_COEF_MSBS: c_uint = 0x51;
pub const TVP7002_YUV_U_B_COEF_LSBS: c_uint = 0x52;
pub const TVP7002_YUV_U_B_COEF_MSBS: c_uint = 0x53;
pub const TVP7002_YUV_U_R_COEF_LSBS: c_uint = 0x54;
pub const TVP7002_YUV_U_R_COEF_MSBS: c_uint = 0x55;
pub const TVP7002_YUV_V_G_COEF_LSBS: c_uint = 0x56;
pub const TVP7002_YUV_V_G_COEF_MSBS: c_uint = 0x57;
pub const TVP7002_YUV_V_B_COEF_LSBS: c_uint = 0x58;
pub const TVP7002_YUV_V_B_COEF_MSBS: c_uint = 0x59;
pub const TVP7002_YUV_V_R_COEF_LSBS: c_uint = 0x5a;
pub const TVP7002_YUV_V_R_COEF_MSBS: c_uint = 0x5b;
