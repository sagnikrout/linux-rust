//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/sor.h
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
// Copyright (C) 2013 NVIDIA Corporation
//
pub const SOR_CTXSW: c_uint = 0x00;
pub const SOR_SUPER_STATE0: c_uint = 0x01;
pub const SOR_SUPER_STATE1: c_uint = 0x02;

pub const SOR_STATE0: c_uint = 0x03;
pub const SOR_STATE1: c_uint = 0x04;

pub const SOR_STATE_ASY_OWNER_MASK: c_uint = 0xf;

pub const SOR_CRC_CNTRL: c_uint = 0x11;

pub const SOR_DP_DEBUG_MVID: c_uint = 0x12;
pub const SOR_CLK_CNTRL: c_uint = 0x13;

pub const SOR_CAP: c_uint = 0x14;
pub const SOR_PWR: c_uint = 0x15;

pub const SOR_TEST: c_uint = 0x16;

pub const SOR_PLL0: c_uint = 0x17;

pub const SOR_PLL1: c_uint = 0x18;
// XXX: read-only bit?

pub const SOR_PLL2: c_uint = 0x19;

pub const SOR_PLL3: c_uint = 0x1a;

pub const SOR_CSTM: c_uint = 0x1b;

pub const SOR_LVDS: c_uint = 0x1c;
pub const SOR_CRCA: c_uint = 0x1d;

pub const SOR_CRCB: c_uint = 0x1e;
pub const SOR_BLANK: c_uint = 0x1f;
pub const SOR_SEQ_CTL: c_uint = 0x20;

pub const SOR_LANE_SEQ_CTL: c_uint = 0x21;

pub const SOR_PWM_DIV: c_uint = 0x32;
pub const SOR_PWM_DIV_MASK: c_uint = 0xffffff;
pub const SOR_PWM_CTL: c_uint = 0x33;

pub const SOR_PWM_CTL_DUTY_CYCLE_MASK: c_uint = 0xffffff;
pub const SOR_VCRC_A0: c_uint = 0x34;
pub const SOR_VCRC_A1: c_uint = 0x35;
pub const SOR_VCRC_B0: c_uint = 0x36;
pub const SOR_VCRC_B1: c_uint = 0x37;
pub const SOR_CCRC_A0: c_uint = 0x38;
pub const SOR_CCRC_A1: c_uint = 0x39;
pub const SOR_CCRC_B0: c_uint = 0x3a;
pub const SOR_CCRC_B1: c_uint = 0x3b;
pub const SOR_EDATA_A0: c_uint = 0x3c;
pub const SOR_EDATA_A1: c_uint = 0x3d;
pub const SOR_EDATA_B0: c_uint = 0x3e;
pub const SOR_EDATA_B1: c_uint = 0x3f;
pub const SOR_COUNT_A0: c_uint = 0x40;
pub const SOR_COUNT_A1: c_uint = 0x41;
pub const SOR_COUNT_B0: c_uint = 0x42;
pub const SOR_COUNT_B1: c_uint = 0x43;
pub const SOR_DEBUG_A0: c_uint = 0x44;
pub const SOR_DEBUG_A1: c_uint = 0x45;
pub const SOR_DEBUG_B0: c_uint = 0x46;
pub const SOR_DEBUG_B1: c_uint = 0x47;
pub const SOR_TRIG: c_uint = 0x48;
pub const SOR_MSCHECK: c_uint = 0x49;
pub const SOR_XBAR_CTRL: c_uint = 0x4a;

pub const SOR_XBAR_POL: c_uint = 0x4b;
pub const SOR_DP_LINKCTL0: c_uint = 0x4c;

pub const SOR_DP_LINKCTL1: c_uint = 0x4d;
pub const SOR_LANE_DRIVE_CURRENT0: c_uint = 0x4e;
pub const SOR_LANE_DRIVE_CURRENT1: c_uint = 0x4f;
pub const SOR_LANE4_DRIVE_CURRENT0: c_uint = 0x50;
pub const SOR_LANE4_DRIVE_CURRENT1: c_uint = 0x51;

pub const SOR_LANE_PREEMPHASIS0: c_uint = 0x52;
pub const SOR_LANE_PREEMPHASIS1: c_uint = 0x53;
pub const SOR_LANE4_PREEMPHASIS0: c_uint = 0x54;
pub const SOR_LANE4_PREEMPHASIS1: c_uint = 0x55;

pub const SOR_LANE_POSTCURSOR0: c_uint = 0x56;
pub const SOR_LANE_POSTCURSOR1: c_uint = 0x57;

pub const SOR_DP_CONFIG0: c_uint = 0x58;

pub const SOR_DP_CONFIG1: c_uint = 0x59;
pub const SOR_DP_MN0: c_uint = 0x5a;
pub const SOR_DP_MN1: c_uint = 0x5b;
pub const SOR_DP_PADCTL0: c_uint = 0x5c;

pub const SOR_DP_PADCTL1: c_uint = 0x5d;
pub const SOR_DP_DEBUG0: c_uint = 0x5e;
pub const SOR_DP_DEBUG1: c_uint = 0x5f;
pub const SOR_DP_SPARE0: c_uint = 0x60;

pub const SOR_DP_SPARE1: c_uint = 0x61;
pub const SOR_DP_AUDIO_CTRL: c_uint = 0x62;
pub const SOR_DP_AUDIO_HBLANK_SYMBOLS: c_uint = 0x63;

pub const SOR_DP_AUDIO_VBLANK_SYMBOLS: c_uint = 0x64;

pub const SOR_DP_GENERIC_INFOFRAME_HEADER: c_uint = 0x65;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK0: c_uint = 0x66;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK1: c_uint = 0x67;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK2: c_uint = 0x68;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK3: c_uint = 0x69;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK4: c_uint = 0x6a;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK5: c_uint = 0x6b;
pub const SOR_DP_GENERIC_INFOFRAME_SUBPACK6: c_uint = 0x6c;
pub const SOR_DP_TPG: c_uint = 0x6d;

pub const SOR_DP_TPG_CONFIG: c_uint = 0x6e;
pub const SOR_DP_LQ_CSTM0: c_uint = 0x6f;
pub const SOR_DP_LQ_CSTM1: c_uint = 0x70;
pub const SOR_DP_LQ_CSTM2: c_uint = 0x71;
pub const SOR_DP_PADCTL2: c_uint = 0x73;

pub const SOR_HDMI_AUDIO_INFOFRAME_CTRL: c_uint = 0x9a;
pub const SOR_HDMI_AUDIO_INFOFRAME_STATUS: c_uint = 0x9b;
pub const SOR_HDMI_AUDIO_INFOFRAME_HEADER: c_uint = 0x9c;
pub const SOR_HDMI_AVI_INFOFRAME_CTRL: c_uint = 0x9f;

pub const SOR_HDMI_AVI_INFOFRAME_STATUS: c_uint = 0xa0;

pub const SOR_HDMI_AVI_INFOFRAME_HEADER: c_uint = 0xa1;

pub const SOR_HDMI_ACR_CTRL: c_uint = 0xb1;
pub const SOR_HDMI_ACR_0320_SUBPACK_LOW: c_uint = 0xb2;

pub const SOR_HDMI_ACR_0320_SUBPACK_HIGH: c_uint = 0xb3;

pub const SOR_HDMI_ACR_0441_SUBPACK_LOW: c_uint = 0xb4;
pub const SOR_HDMI_ACR_0441_SUBPACK_HIGH: c_uint = 0xb5;
pub const SOR_HDMI_CTRL: c_uint = 0xc0;

pub const SOR_HDMI_SPARE: c_uint = 0xcb;

pub const SOR_REFCLK: c_uint = 0xe6;

pub const SOR_INPUT_CONTROL: c_uint = 0xe8;

pub const SOR_AUDIO_CNTRL: c_uint = 0xfc;

pub const SOURCE_SELECT_MASK: c_uint = 0x3;
pub const SOURCE_SELECT_HDA: c_uint = 0x2;
pub const SOURCE_SELECT_SPDIF: c_uint = 0x1;
pub const SOURCE_SELECT_AUTO: c_uint = 0x0;

pub const SOR_AUDIO_SPARE: c_uint = 0xfe;

pub const SOR_AUDIO_NVAL_0320: c_uint = 0xff;
pub const SOR_AUDIO_NVAL_0441: c_uint = 0x100;
pub const SOR_AUDIO_NVAL_0882: c_uint = 0x101;
pub const SOR_AUDIO_NVAL_1764: c_uint = 0x102;
pub const SOR_AUDIO_NVAL_0480: c_uint = 0x103;
pub const SOR_AUDIO_NVAL_0960: c_uint = 0x104;
pub const SOR_AUDIO_NVAL_1920: c_uint = 0x105;
pub const SOR_AUDIO_HDA_CODEC_SCRATCH0: c_uint = 0x10a;

pub const SOR_AUDIO_HDA_CODEC_SCRATCH0_FMT_MASK: c_uint = 0xffff;
pub const SOR_AUDIO_HDA_ELD_BUFWR: c_uint = 0x10c;

pub const SOR_AUDIO_HDA_PRESENSE: c_uint = 0x10d;

pub const SOR_AUDIO_AVAL_0320: c_uint = 0x10f;
pub const SOR_AUDIO_AVAL_0441: c_uint = 0x110;
pub const SOR_AUDIO_AVAL_0882: c_uint = 0x111;
pub const SOR_AUDIO_AVAL_1764: c_uint = 0x112;
pub const SOR_AUDIO_AVAL_0480: c_uint = 0x113;
pub const SOR_AUDIO_AVAL_0960: c_uint = 0x114;
pub const SOR_AUDIO_AVAL_1920: c_uint = 0x115;
pub const SOR_INT_STATUS: c_uint = 0x11c;

pub const SOR_INT_MASK: c_uint = 0x11d;
pub const SOR_INT_ENABLE: c_uint = 0x11e;
pub const SOR_HDMI_VSI_INFOFRAME_CTRL: c_uint = 0x123;
pub const SOR_HDMI_VSI_INFOFRAME_STATUS: c_uint = 0x124;
pub const SOR_HDMI_VSI_INFOFRAME_HEADER: c_uint = 0x125;
pub const SOR_HDMI_AUDIO_N: c_uint = 0x13c;

pub const SOR_HDMI2_CTRL: c_uint = 0x13e;

