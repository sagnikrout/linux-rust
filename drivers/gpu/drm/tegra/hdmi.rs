//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/tegra/hdmi.h
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
// Copyright (C) 2012 Avionic Design GmbH
// Copyright (C) 2012 NVIDIA CORPORATION.  All rights reserved.
//
pub const TEGRA_HDMI_H: c_int = 1;
// register definitions
pub const HDMI_CTXSW: c_uint = 0x00;
pub const HDMI_NV_PDISP_SOR_STATE0: c_uint = 0x01;

pub const HDMI_NV_PDISP_SOR_STATE1: c_uint = 0x02;

pub const HDMI_NV_PDISP_SOR_STATE2: c_uint = 0x03;

pub const HDMI_NV_PDISP_RG_HDCP_AN_MSB: c_uint = 0x04;
pub const HDMI_NV_PDISP_RG_HDCP_AN_LSB: c_uint = 0x05;
pub const HDMI_NV_PDISP_RG_HDCP_CN_MSB: c_uint = 0x06;
pub const HDMI_NV_PDISP_RG_HDCP_CN_LSB: c_uint = 0x07;
pub const HDMI_NV_PDISP_RG_HDCP_AKSV_MSB: c_uint = 0x08;
pub const HDMI_NV_PDISP_RG_HDCP_AKSV_LSB: c_uint = 0x09;
pub const HDMI_NV_PDISP_RG_HDCP_BKSV_MSB: c_uint = 0x0a;
pub const HDMI_NV_PDISP_RG_HDCP_BKSV_LSB: c_uint = 0x0b;
pub const HDMI_NV_PDISP_RG_HDCP_CKSV_MSB: c_uint = 0x0c;
pub const HDMI_NV_PDISP_RG_HDCP_CKSV_LSB: c_uint = 0x0d;
pub const HDMI_NV_PDISP_RG_HDCP_DKSV_MSB: c_uint = 0x0e;
pub const HDMI_NV_PDISP_RG_HDCP_DKSV_LSB: c_uint = 0x0f;
pub const HDMI_NV_PDISP_RG_HDCP_CTRL: c_uint = 0x10;
pub const HDMI_NV_PDISP_RG_HDCP_CMODE: c_uint = 0x11;
pub const HDMI_NV_PDISP_RG_HDCP_MPRIME_MSB: c_uint = 0x12;
pub const HDMI_NV_PDISP_RG_HDCP_MPRIME_LSB: c_uint = 0x13;
pub const HDMI_NV_PDISP_RG_HDCP_SPRIME_MSB: c_uint = 0x14;
pub const HDMI_NV_PDISP_RG_HDCP_SPRIME_LSB2: c_uint = 0x15;
pub const HDMI_NV_PDISP_RG_HDCP_SPRIME_LSB1: c_uint = 0x16;
pub const HDMI_NV_PDISP_RG_HDCP_RI: c_uint = 0x17;
pub const HDMI_NV_PDISP_RG_HDCP_CS_MSB: c_uint = 0x18;
pub const HDMI_NV_PDISP_RG_HDCP_CS_LSB: c_uint = 0x19;
pub const HDMI_NV_PDISP_HDMI_AUDIO_EMU0: c_uint = 0x1a;
pub const HDMI_NV_PDISP_HDMI_AUDIO_EMU_RDATA0: c_uint = 0x1b;
pub const HDMI_NV_PDISP_HDMI_AUDIO_EMU1: c_uint = 0x1c;
pub const HDMI_NV_PDISP_HDMI_AUDIO_EMU2: c_uint = 0x1d;
pub const HDMI_NV_PDISP_HDMI_AUDIO_INFOFRAME_CTRL: c_uint = 0x1e;
pub const HDMI_NV_PDISP_HDMI_AUDIO_INFOFRAME_STATUS: c_uint = 0x1f;
pub const HDMI_NV_PDISP_HDMI_AUDIO_INFOFRAME_HEADER: c_uint = 0x20;
pub const HDMI_NV_PDISP_HDMI_AUDIO_INFOFRAME_SUBPACK0_LOW: c_uint = 0x21;
pub const HDMI_NV_PDISP_HDMI_AUDIO_INFOFRAME_SUBPACK0_HIGH: c_uint = 0x22;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_CTRL: c_uint = 0x23;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_STATUS: c_uint = 0x24;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_HEADER: c_uint = 0x25;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_SUBPACK0_LOW: c_uint = 0x26;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_SUBPACK0_HIGH: c_uint = 0x27;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_SUBPACK1_LOW: c_uint = 0x28;
pub const HDMI_NV_PDISP_HDMI_AVI_INFOFRAME_SUBPACK1_HIGH: c_uint = 0x29;

pub const HDMI_NV_PDISP_HDMI_GENERIC_CTRL: c_uint = 0x2a;

pub const HDMI_NV_PDISP_HDMI_GENERIC_STATUS: c_uint = 0x2b;
pub const HDMI_NV_PDISP_HDMI_GENERIC_HEADER: c_uint = 0x2c;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK0_LOW: c_uint = 0x2d;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK0_HIGH: c_uint = 0x2e;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK1_LOW: c_uint = 0x2f;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK1_HIGH: c_uint = 0x30;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK2_LOW: c_uint = 0x31;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK2_HIGH: c_uint = 0x32;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK3_LOW: c_uint = 0x33;
pub const HDMI_NV_PDISP_HDMI_GENERIC_SUBPACK3_HIGH: c_uint = 0x34;
pub const HDMI_NV_PDISP_HDMI_ACR_CTRL: c_uint = 0x35;
pub const HDMI_NV_PDISP_HDMI_ACR_0320_SUBPACK_LOW: c_uint = 0x36;
pub const HDMI_NV_PDISP_HDMI_ACR_0320_SUBPACK_HIGH: c_uint = 0x37;
pub const HDMI_NV_PDISP_HDMI_ACR_0441_SUBPACK_LOW: c_uint = 0x38;
pub const HDMI_NV_PDISP_HDMI_ACR_0441_SUBPACK_HIGH: c_uint = 0x39;
pub const HDMI_NV_PDISP_HDMI_ACR_0882_SUBPACK_LOW: c_uint = 0x3a;
pub const HDMI_NV_PDISP_HDMI_ACR_0882_SUBPACK_HIGH: c_uint = 0x3b;
pub const HDMI_NV_PDISP_HDMI_ACR_1764_SUBPACK_LOW: c_uint = 0x3c;
pub const HDMI_NV_PDISP_HDMI_ACR_1764_SUBPACK_HIGH: c_uint = 0x3d;
pub const HDMI_NV_PDISP_HDMI_ACR_0480_SUBPACK_LOW: c_uint = 0x3e;
pub const HDMI_NV_PDISP_HDMI_ACR_0480_SUBPACK_HIGH: c_uint = 0x3f;
pub const HDMI_NV_PDISP_HDMI_ACR_0960_SUBPACK_LOW: c_uint = 0x40;
pub const HDMI_NV_PDISP_HDMI_ACR_0960_SUBPACK_HIGH: c_uint = 0x41;
pub const HDMI_NV_PDISP_HDMI_ACR_1920_SUBPACK_LOW: c_uint = 0x42;
pub const HDMI_NV_PDISP_HDMI_ACR_1920_SUBPACK_HIGH: c_uint = 0x43;

pub const HDMI_NV_PDISP_HDMI_CTRL: c_uint = 0x44;

pub const HDMI_NV_PDISP_HDMI_VSYNC_KEEPOUT: c_uint = 0x45;
pub const HDMI_NV_PDISP_HDMI_VSYNC_WINDOW: c_uint = 0x46;

pub const HDMI_NV_PDISP_HDMI_GCP_CTRL: c_uint = 0x47;
pub const HDMI_NV_PDISP_HDMI_GCP_STATUS: c_uint = 0x48;
pub const HDMI_NV_PDISP_HDMI_GCP_SUBPACK: c_uint = 0x49;
pub const HDMI_NV_PDISP_HDMI_CHANNEL_STATUS1: c_uint = 0x4a;
pub const HDMI_NV_PDISP_HDMI_CHANNEL_STATUS2: c_uint = 0x4b;
pub const HDMI_NV_PDISP_HDMI_EMU0: c_uint = 0x4c;
pub const HDMI_NV_PDISP_HDMI_EMU1: c_uint = 0x4d;
pub const HDMI_NV_PDISP_HDMI_EMU1_RDATA: c_uint = 0x4e;
pub const HDMI_NV_PDISP_HDMI_SPARE: c_uint = 0x4f;

pub const HDMI_NV_PDISP_HDMI_SPDIF_CHN_STATUS1: c_uint = 0x50;
pub const HDMI_NV_PDISP_HDMI_SPDIF_CHN_STATUS2: c_uint = 0x51;
pub const HDMI_NV_PDISP_HDMI_HDCPRIF_ROM_CTRL: c_uint = 0x53;
pub const HDMI_NV_PDISP_SOR_CAP: c_uint = 0x54;
pub const HDMI_NV_PDISP_SOR_PWR: c_uint = 0x55;

pub const HDMI_NV_PDISP_SOR_TEST: c_uint = 0x56;
pub const HDMI_NV_PDISP_SOR_PLL0: c_uint = 0x57;

pub const HDMI_NV_PDISP_SOR_PLL1: c_uint = 0x58;

pub const HDMI_NV_PDISP_SOR_PLL2: c_uint = 0x59;
pub const HDMI_NV_PDISP_SOR_CSTM: c_uint = 0x5a;

pub const HDMI_NV_PDISP_SOR_LVDS: c_uint = 0x5b;
pub const HDMI_NV_PDISP_SOR_CRCA: c_uint = 0x5c;
pub const HDMI_NV_PDISP_SOR_CRCB: c_uint = 0x5d;
pub const HDMI_NV_PDISP_SOR_BLANK: c_uint = 0x5e;
pub const HDMI_NV_PDISP_SOR_SEQ_CTL: c_uint = 0x5f;

pub const HDMI_NV_PDISP_SOR_VCRCA0: c_uint = 0x72;
pub const HDMI_NV_PDISP_SOR_VCRCA1: c_uint = 0x73;
pub const HDMI_NV_PDISP_SOR_CCRCA0: c_uint = 0x74;
pub const HDMI_NV_PDISP_SOR_CCRCA1: c_uint = 0x75;
pub const HDMI_NV_PDISP_SOR_EDATAA0: c_uint = 0x76;
pub const HDMI_NV_PDISP_SOR_EDATAA1: c_uint = 0x77;
pub const HDMI_NV_PDISP_SOR_COUNTA0: c_uint = 0x78;
pub const HDMI_NV_PDISP_SOR_COUNTA1: c_uint = 0x79;
pub const HDMI_NV_PDISP_SOR_DEBUGA0: c_uint = 0x7a;
pub const HDMI_NV_PDISP_SOR_DEBUGA1: c_uint = 0x7b;
pub const HDMI_NV_PDISP_SOR_TRIG: c_uint = 0x7c;
pub const HDMI_NV_PDISP_SOR_MSCHECK: c_uint = 0x7d;
pub const HDMI_NV_PDISP_SOR_LANE_DRIVE_CURRENT: c_uint = 0x7e;

pub const DRIVE_CURRENT_1_500_mA: c_uint = 0x00;
pub const DRIVE_CURRENT_1_875_mA: c_uint = 0x01;
pub const DRIVE_CURRENT_2_250_mA: c_uint = 0x02;
pub const DRIVE_CURRENT_2_625_mA: c_uint = 0x03;
pub const DRIVE_CURRENT_3_000_mA: c_uint = 0x04;
pub const DRIVE_CURRENT_3_375_mA: c_uint = 0x05;
pub const DRIVE_CURRENT_3_750_mA: c_uint = 0x06;
pub const DRIVE_CURRENT_4_125_mA: c_uint = 0x07;
pub const DRIVE_CURRENT_4_500_mA: c_uint = 0x08;
pub const DRIVE_CURRENT_4_875_mA: c_uint = 0x09;
pub const DRIVE_CURRENT_5_250_mA: c_uint = 0x0a;
pub const DRIVE_CURRENT_5_625_mA: c_uint = 0x0b;
pub const DRIVE_CURRENT_6_000_mA: c_uint = 0x0c;
pub const DRIVE_CURRENT_6_375_mA: c_uint = 0x0d;
pub const DRIVE_CURRENT_6_750_mA: c_uint = 0x0e;
pub const DRIVE_CURRENT_7_125_mA: c_uint = 0x0f;
pub const DRIVE_CURRENT_7_500_mA: c_uint = 0x10;
pub const DRIVE_CURRENT_7_875_mA: c_uint = 0x11;
pub const DRIVE_CURRENT_8_250_mA: c_uint = 0x12;
pub const DRIVE_CURRENT_8_625_mA: c_uint = 0x13;
pub const DRIVE_CURRENT_9_000_mA: c_uint = 0x14;
pub const DRIVE_CURRENT_9_375_mA: c_uint = 0x15;
pub const DRIVE_CURRENT_9_750_mA: c_uint = 0x16;
pub const DRIVE_CURRENT_10_125_mA: c_uint = 0x17;
pub const DRIVE_CURRENT_10_500_mA: c_uint = 0x18;
pub const DRIVE_CURRENT_10_875_mA: c_uint = 0x19;
pub const DRIVE_CURRENT_11_250_mA: c_uint = 0x1a;
pub const DRIVE_CURRENT_11_625_mA: c_uint = 0x1b;
pub const DRIVE_CURRENT_12_000_mA: c_uint = 0x1c;
pub const DRIVE_CURRENT_12_375_mA: c_uint = 0x1d;
pub const DRIVE_CURRENT_12_750_mA: c_uint = 0x1e;
pub const DRIVE_CURRENT_13_125_mA: c_uint = 0x1f;
pub const DRIVE_CURRENT_13_500_mA: c_uint = 0x20;
pub const DRIVE_CURRENT_13_875_mA: c_uint = 0x21;
pub const DRIVE_CURRENT_14_250_mA: c_uint = 0x22;
pub const DRIVE_CURRENT_14_625_mA: c_uint = 0x23;
pub const DRIVE_CURRENT_15_000_mA: c_uint = 0x24;
pub const DRIVE_CURRENT_15_375_mA: c_uint = 0x25;
pub const DRIVE_CURRENT_15_750_mA: c_uint = 0x26;
pub const DRIVE_CURRENT_16_125_mA: c_uint = 0x27;
pub const DRIVE_CURRENT_16_500_mA: c_uint = 0x28;
pub const DRIVE_CURRENT_16_875_mA: c_uint = 0x29;
pub const DRIVE_CURRENT_17_250_mA: c_uint = 0x2a;
pub const DRIVE_CURRENT_17_625_mA: c_uint = 0x2b;
pub const DRIVE_CURRENT_18_000_mA: c_uint = 0x2c;
pub const DRIVE_CURRENT_18_375_mA: c_uint = 0x2d;
pub const DRIVE_CURRENT_18_750_mA: c_uint = 0x2e;
pub const DRIVE_CURRENT_19_125_mA: c_uint = 0x2f;
pub const DRIVE_CURRENT_19_500_mA: c_uint = 0x30;
pub const DRIVE_CURRENT_19_875_mA: c_uint = 0x31;
pub const DRIVE_CURRENT_20_250_mA: c_uint = 0x32;
pub const DRIVE_CURRENT_20_625_mA: c_uint = 0x33;
pub const DRIVE_CURRENT_21_000_mA: c_uint = 0x34;
pub const DRIVE_CURRENT_21_375_mA: c_uint = 0x35;
pub const DRIVE_CURRENT_21_750_mA: c_uint = 0x36;
pub const DRIVE_CURRENT_22_125_mA: c_uint = 0x37;
pub const DRIVE_CURRENT_22_500_mA: c_uint = 0x38;
pub const DRIVE_CURRENT_22_875_mA: c_uint = 0x39;
pub const DRIVE_CURRENT_23_250_mA: c_uint = 0x3a;
pub const DRIVE_CURRENT_23_625_mA: c_uint = 0x3b;
pub const DRIVE_CURRENT_24_000_mA: c_uint = 0x3c;
pub const DRIVE_CURRENT_24_375_mA: c_uint = 0x3d;
pub const DRIVE_CURRENT_24_750_mA: c_uint = 0x3e;
pub const DRIVE_CURRENT_0_000_mA_T114: c_uint = 0x00;
pub const DRIVE_CURRENT_0_400_mA_T114: c_uint = 0x01;
pub const DRIVE_CURRENT_0_800_mA_T114: c_uint = 0x02;
pub const DRIVE_CURRENT_1_200_mA_T114: c_uint = 0x03;
pub const DRIVE_CURRENT_1_600_mA_T114: c_uint = 0x04;
pub const DRIVE_CURRENT_2_000_mA_T114: c_uint = 0x05;
pub const DRIVE_CURRENT_2_400_mA_T114: c_uint = 0x06;
pub const DRIVE_CURRENT_2_800_mA_T114: c_uint = 0x07;
pub const DRIVE_CURRENT_3_200_mA_T114: c_uint = 0x08;
pub const DRIVE_CURRENT_3_600_mA_T114: c_uint = 0x09;
pub const DRIVE_CURRENT_4_000_mA_T114: c_uint = 0x0a;
pub const DRIVE_CURRENT_4_400_mA_T114: c_uint = 0x0b;
pub const DRIVE_CURRENT_4_800_mA_T114: c_uint = 0x0c;
pub const DRIVE_CURRENT_5_200_mA_T114: c_uint = 0x0d;
pub const DRIVE_CURRENT_5_600_mA_T114: c_uint = 0x0e;
pub const DRIVE_CURRENT_6_000_mA_T114: c_uint = 0x0f;
pub const DRIVE_CURRENT_6_400_mA_T114: c_uint = 0x10;
pub const DRIVE_CURRENT_6_800_mA_T114: c_uint = 0x11;
pub const DRIVE_CURRENT_7_200_mA_T114: c_uint = 0x12;
pub const DRIVE_CURRENT_7_600_mA_T114: c_uint = 0x13;
pub const DRIVE_CURRENT_8_000_mA_T114: c_uint = 0x14;
pub const DRIVE_CURRENT_8_400_mA_T114: c_uint = 0x15;
pub const DRIVE_CURRENT_8_800_mA_T114: c_uint = 0x16;
pub const DRIVE_CURRENT_9_200_mA_T114: c_uint = 0x17;
pub const DRIVE_CURRENT_9_600_mA_T114: c_uint = 0x18;
pub const DRIVE_CURRENT_10_000_mA_T114: c_uint = 0x19;
pub const DRIVE_CURRENT_10_400_mA_T114: c_uint = 0x1a;
pub const DRIVE_CURRENT_10_800_mA_T114: c_uint = 0x1b;
pub const DRIVE_CURRENT_11_200_mA_T114: c_uint = 0x1c;
pub const DRIVE_CURRENT_11_600_mA_T114: c_uint = 0x1d;
pub const DRIVE_CURRENT_12_000_mA_T114: c_uint = 0x1e;
pub const DRIVE_CURRENT_12_400_mA_T114: c_uint = 0x1f;
pub const DRIVE_CURRENT_12_800_mA_T114: c_uint = 0x20;
pub const DRIVE_CURRENT_13_200_mA_T114: c_uint = 0x21;
pub const DRIVE_CURRENT_13_600_mA_T114: c_uint = 0x22;
pub const DRIVE_CURRENT_14_000_mA_T114: c_uint = 0x23;
pub const DRIVE_CURRENT_14_400_mA_T114: c_uint = 0x24;
pub const DRIVE_CURRENT_14_800_mA_T114: c_uint = 0x25;
pub const DRIVE_CURRENT_15_200_mA_T114: c_uint = 0x26;
pub const DRIVE_CURRENT_15_600_mA_T114: c_uint = 0x27;
pub const DRIVE_CURRENT_16_000_mA_T114: c_uint = 0x28;
pub const DRIVE_CURRENT_16_400_mA_T114: c_uint = 0x29;
pub const DRIVE_CURRENT_16_800_mA_T114: c_uint = 0x2a;
pub const DRIVE_CURRENT_17_200_mA_T114: c_uint = 0x2b;
pub const DRIVE_CURRENT_17_600_mA_T114: c_uint = 0x2c;
pub const DRIVE_CURRENT_18_000_mA_T114: c_uint = 0x2d;
pub const DRIVE_CURRENT_18_400_mA_T114: c_uint = 0x2e;
pub const DRIVE_CURRENT_18_800_mA_T114: c_uint = 0x2f;
pub const DRIVE_CURRENT_19_200_mA_T114: c_uint = 0x30;
pub const DRIVE_CURRENT_19_600_mA_T114: c_uint = 0x31;
pub const DRIVE_CURRENT_20_000_mA_T114: c_uint = 0x32;
pub const DRIVE_CURRENT_20_400_mA_T114: c_uint = 0x33;
pub const DRIVE_CURRENT_20_800_mA_T114: c_uint = 0x34;
pub const DRIVE_CURRENT_21_200_mA_T114: c_uint = 0x35;
pub const DRIVE_CURRENT_21_600_mA_T114: c_uint = 0x36;
pub const DRIVE_CURRENT_22_000_mA_T114: c_uint = 0x37;
pub const DRIVE_CURRENT_22_400_mA_T114: c_uint = 0x38;
pub const DRIVE_CURRENT_22_800_mA_T114: c_uint = 0x39;
pub const DRIVE_CURRENT_23_200_mA_T114: c_uint = 0x3a;
pub const DRIVE_CURRENT_23_600_mA_T114: c_uint = 0x3b;
pub const DRIVE_CURRENT_24_000_mA_T114: c_uint = 0x3c;
pub const DRIVE_CURRENT_24_400_mA_T114: c_uint = 0x3d;
pub const DRIVE_CURRENT_24_800_mA_T114: c_uint = 0x3e;
pub const DRIVE_CURRENT_25_200_mA_T114: c_uint = 0x3f;
pub const DRIVE_CURRENT_25_400_mA_T114: c_uint = 0x40;
pub const DRIVE_CURRENT_25_800_mA_T114: c_uint = 0x41;
pub const DRIVE_CURRENT_26_200_mA_T114: c_uint = 0x42;
pub const DRIVE_CURRENT_26_600_mA_T114: c_uint = 0x43;
pub const DRIVE_CURRENT_27_000_mA_T114: c_uint = 0x44;
pub const DRIVE_CURRENT_27_400_mA_T114: c_uint = 0x45;
pub const DRIVE_CURRENT_27_800_mA_T114: c_uint = 0x46;
pub const DRIVE_CURRENT_28_200_mA_T114: c_uint = 0x47;
pub const HDMI_NV_PDISP_AUDIO_DEBUG0: c_uint = 0x7f;
pub const HDMI_NV_PDISP_AUDIO_DEBUG1: c_uint = 0x80;
pub const HDMI_NV_PDISP_AUDIO_DEBUG2: c_uint = 0x81;

pub const HDMI_NV_PDISP_AUDIO_PULSE_WIDTH: c_uint = 0x89;
pub const HDMI_NV_PDISP_AUDIO_THRESHOLD: c_uint = 0x8a;
pub const HDMI_NV_PDISP_AUDIO_CNTRL0: c_uint = 0x8b;

pub const HDMI_NV_PDISP_AUDIO_N: c_uint = 0x8c;

pub const HDMI_NV_PDISP_HDCPRIF_ROM_TIMING: c_uint = 0x94;
pub const HDMI_NV_PDISP_SOR_REFCLK: c_uint = 0x95;

pub const HDMI_NV_PDISP_CRC_CONTROL: c_uint = 0x96;
pub const HDMI_NV_PDISP_INPUT_CONTROL: c_uint = 0x97;

pub const HDMI_NV_PDISP_SCRATCH: c_uint = 0x98;
pub const HDMI_NV_PDISP_PE_CURRENT: c_uint = 0x99;

pub const PE_CURRENT_0_0_mA: c_uint = 0x0;
pub const PE_CURRENT_0_5_mA: c_uint = 0x1;
pub const PE_CURRENT_1_0_mA: c_uint = 0x2;
pub const PE_CURRENT_1_5_mA: c_uint = 0x3;
pub const PE_CURRENT_2_0_mA: c_uint = 0x4;
pub const PE_CURRENT_2_5_mA: c_uint = 0x5;
pub const PE_CURRENT_3_0_mA: c_uint = 0x6;
pub const PE_CURRENT_3_5_mA: c_uint = 0x7;
pub const PE_CURRENT_4_0_mA: c_uint = 0x8;
pub const PE_CURRENT_4_5_mA: c_uint = 0x9;
pub const PE_CURRENT_5_0_mA: c_uint = 0xa;
pub const PE_CURRENT_5_5_mA: c_uint = 0xb;
pub const PE_CURRENT_6_0_mA: c_uint = 0xc;
pub const PE_CURRENT_6_5_mA: c_uint = 0xd;
pub const PE_CURRENT_7_0_mA: c_uint = 0xe;
pub const PE_CURRENT_7_5_mA: c_uint = 0xf;
pub const PE_CURRENT_0_mA_T114: c_uint = 0x0;
pub const PE_CURRENT_1_mA_T114: c_uint = 0x1;
pub const PE_CURRENT_2_mA_T114: c_uint = 0x2;
pub const PE_CURRENT_3_mA_T114: c_uint = 0x3;
pub const PE_CURRENT_4_mA_T114: c_uint = 0x4;
pub const PE_CURRENT_5_mA_T114: c_uint = 0x5;
pub const PE_CURRENT_6_mA_T114: c_uint = 0x6;
pub const PE_CURRENT_7_mA_T114: c_uint = 0x7;
pub const PE_CURRENT_8_mA_T114: c_uint = 0x8;
pub const PE_CURRENT_9_mA_T114: c_uint = 0x9;
pub const PE_CURRENT_10_mA_T114: c_uint = 0xa;
pub const PE_CURRENT_11_mA_T114: c_uint = 0xb;
pub const PE_CURRENT_12_mA_T114: c_uint = 0xc;
pub const PE_CURRENT_13_mA_T114: c_uint = 0xd;
pub const PE_CURRENT_14_mA_T114: c_uint = 0xe;
pub const PE_CURRENT_15_mA_T114: c_uint = 0xf;
pub const HDMI_NV_PDISP_KEY_CTRL: c_uint = 0x9a;
pub const HDMI_NV_PDISP_KEY_DEBUG0: c_uint = 0x9b;
pub const HDMI_NV_PDISP_KEY_DEBUG1: c_uint = 0x9c;
pub const HDMI_NV_PDISP_KEY_DEBUG2: c_uint = 0x9d;
pub const HDMI_NV_PDISP_KEY_HDCP_KEY_0: c_uint = 0x9e;
pub const HDMI_NV_PDISP_KEY_HDCP_KEY_1: c_uint = 0x9f;
pub const HDMI_NV_PDISP_KEY_HDCP_KEY_2: c_uint = 0xa0;
pub const HDMI_NV_PDISP_KEY_HDCP_KEY_3: c_uint = 0xa1;
pub const HDMI_NV_PDISP_KEY_HDCP_KEY_TRIG: c_uint = 0xa2;
pub const HDMI_NV_PDISP_KEY_SKEY_INDEX: c_uint = 0xa3;
pub const HDMI_NV_PDISP_SOR_AUDIO_CNTRL0: c_uint = 0xac;

pub const HDMI_NV_PDISP_SOR_AUDIO_SPARE0: c_uint = 0xae;

pub const HDMI_NV_PDISP_SOR_AUDIO_HDA_CODEC_SCRATCH0: c_uint = 0xba;

pub const SOR_AUDIO_HDA_CODEC_SCRATCH0_FMT_MASK: c_uint = 0xffff;
pub const HDMI_NV_PDISP_SOR_AUDIO_HDA_CODEC_SCRATCH1: c_uint = 0xbb;
pub const HDMI_NV_PDISP_SOR_AUDIO_HDA_ELD_BUFWR: c_uint = 0xbc;
pub const HDMI_NV_PDISP_SOR_AUDIO_HDA_PRESENSE: c_uint = 0xbd;

pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_0320: c_uint = 0xbf;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_0441: c_uint = 0xc0;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_0882: c_uint = 0xc1;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_1764: c_uint = 0xc2;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_0480: c_uint = 0xc3;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_0960: c_uint = 0xc4;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_1920: c_uint = 0xc5;
pub const HDMI_NV_PDISP_SOR_AUDIO_AVAL_DEFAULT: c_uint = 0xc5;
pub const HDMI_NV_PDISP_INT_STATUS: c_uint = 0xcc;

pub const HDMI_NV_PDISP_INT_MASK: c_uint = 0xcd;
pub const HDMI_NV_PDISP_INT_ENABLE: c_uint = 0xce;
pub const HDMI_NV_PDISP_SOR_IO_PEAK_CURRENT: c_uint = 0xd1;

pub const PEAK_CURRENT_0_000_mA: c_uint = 0x00;
pub const PEAK_CURRENT_0_200_mA: c_uint = 0x01;
pub const PEAK_CURRENT_0_400_mA: c_uint = 0x02;
pub const PEAK_CURRENT_0_600_mA: c_uint = 0x03;
pub const PEAK_CURRENT_0_800_mA: c_uint = 0x04;
pub const PEAK_CURRENT_1_000_mA: c_uint = 0x05;
pub const PEAK_CURRENT_1_200_mA: c_uint = 0x06;
pub const PEAK_CURRENT_1_400_mA: c_uint = 0x07;
pub const PEAK_CURRENT_1_600_mA: c_uint = 0x08;
pub const PEAK_CURRENT_1_800_mA: c_uint = 0x09;
pub const PEAK_CURRENT_2_000_mA: c_uint = 0x0a;
pub const PEAK_CURRENT_2_200_mA: c_uint = 0x0b;
pub const PEAK_CURRENT_2_400_mA: c_uint = 0x0c;
pub const PEAK_CURRENT_2_600_mA: c_uint = 0x0d;
pub const PEAK_CURRENT_2_800_mA: c_uint = 0x0e;
pub const PEAK_CURRENT_3_000_mA: c_uint = 0x0f;
pub const PEAK_CURRENT_3_200_mA: c_uint = 0x10;
pub const PEAK_CURRENT_3_400_mA: c_uint = 0x11;
pub const PEAK_CURRENT_3_600_mA: c_uint = 0x12;
pub const PEAK_CURRENT_3_800_mA: c_uint = 0x13;
pub const PEAK_CURRENT_4_000_mA: c_uint = 0x14;
pub const PEAK_CURRENT_4_200_mA: c_uint = 0x15;
pub const PEAK_CURRENT_4_400_mA: c_uint = 0x16;
pub const PEAK_CURRENT_4_600_mA: c_uint = 0x17;
pub const PEAK_CURRENT_4_800_mA: c_uint = 0x18;
pub const PEAK_CURRENT_5_000_mA: c_uint = 0x19;
pub const PEAK_CURRENT_5_200_mA: c_uint = 0x1a;
pub const PEAK_CURRENT_5_400_mA: c_uint = 0x1b;
pub const PEAK_CURRENT_5_600_mA: c_uint = 0x1c;
pub const PEAK_CURRENT_5_800_mA: c_uint = 0x1d;
pub const PEAK_CURRENT_6_000_mA: c_uint = 0x1e;
pub const PEAK_CURRENT_6_200_mA: c_uint = 0x1f;
pub const PEAK_CURRENT_6_400_mA: c_uint = 0x20;
pub const PEAK_CURRENT_6_600_mA: c_uint = 0x21;
pub const PEAK_CURRENT_6_800_mA: c_uint = 0x22;
pub const PEAK_CURRENT_7_000_mA: c_uint = 0x23;
pub const PEAK_CURRENT_7_200_mA: c_uint = 0x24;
pub const PEAK_CURRENT_7_400_mA: c_uint = 0x25;
pub const PEAK_CURRENT_7_600_mA: c_uint = 0x26;
pub const PEAK_CURRENT_7_800_mA: c_uint = 0x27;
pub const PEAK_CURRENT_8_000_mA: c_uint = 0x28;
pub const PEAK_CURRENT_8_200_mA: c_uint = 0x29;
pub const PEAK_CURRENT_8_400_mA: c_uint = 0x2a;
pub const PEAK_CURRENT_8_600_mA: c_uint = 0x2b;
pub const PEAK_CURRENT_8_800_mA: c_uint = 0x2c;
pub const PEAK_CURRENT_9_000_mA: c_uint = 0x2d;
pub const PEAK_CURRENT_9_200_mA: c_uint = 0x2e;
pub const PEAK_CURRENT_9_400_mA: c_uint = 0x2f;
pub const HDMI_NV_PDISP_SOR_PAD_CTLS0: c_uint = 0xd2;
