//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/tac5xx2.h
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


// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC Texas Instruments TAC5XX2 Audio Smart Amplifier
//
// Copyright (C) 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// This the header file for TAC5XX2 family of devices
// which includes TAC5572, TAC5672, TAC5682 and TAS2883
//
// Author: Niranjan H Y <niranjanhy@ti.com>
//
// for soundwire

// page 0 registers

// smartamp function
pub const TAC_FUNCTION_ID_SA: c_uint = 0x1;
pub const TAC_SDCA_ENT_ENT0: c_uint = 0x0;
pub const TAC_SDCA_ENT_PPU21: c_uint = 0x1;
pub const TAC_SDCA_ENT_FU21: c_uint = 0x2;
pub const TAC_SDCA_ENT_FU26: c_uint = 0x3;
pub const TAC_SDCA_ENT_XU22: c_uint = 0x4;
pub const TAC_SDCA_ENT_CS24: c_uint = 0x5;
pub const TAC_SDCA_ENT_CS21: c_uint = 0x6;
pub const TAC_SDCA_ENT_CS25: c_uint = 0x7;
pub const TAC_SDCA_ENT_CS26: c_uint = 0x8;
pub const TAC_SDCA_ENT_CS28: c_uint = 0x9;
pub const TAC_SDCA_ENT_PPU26: c_uint = 0xa;
pub const TAC_SDCA_ENT_FU23: c_uint = 0xb;
pub const TAC_SDCA_ENT_PDE23: c_uint = 0xc;
pub const TAC_SDCA_ENT_TG23: c_uint = 0x12;
pub const TAC_SDCA_ENT_IT21: c_uint = 0x13;
pub const TAC_SDCA_ENT_IT29: c_uint = 0x14;
pub const TAC_SDCA_ENT_IT26: c_uint = 0x15;
pub const TAC_SDCA_ENT_IT28: c_uint = 0x16;
pub const TAC_SDCA_ENT_OT24: c_uint = 0x17;
pub const TAC_SDCA_ENT_OT23: c_uint = 0x18;
pub const TAC_SDCA_ENT_OT25: c_uint = 0x19;
pub const TAC_SDCA_ENT_OT28: c_uint = 0x1a;
pub const TAC_SDCA_ENT_OT27: c_uint = 0x1c;
pub const TAC_SDCA_ENT_SPE199: c_uint = 0x21;
pub const TAC_SDCA_ENT_OT20: c_uint = 0x24;
pub const TAC_SDCA_ENT_FU27: c_uint = 0x26;
pub const TAC_SDCA_ENT_FU20: c_uint = 0x27;
pub const TAC_SDCA_ENT_PDE24: c_uint = 0x2e;
pub const TAC_SDCA_ENT_PDE27: c_uint = 0x2f;
pub const TAC_SDCA_ENT_PDE28: c_uint = 0x30;
pub const TAC_SDCA_ENT_PDE20: c_uint = 0x31;
pub const TAC_SDCA_ENT_SAPU29: c_uint = 0x35;
// Control selector definitions
pub const TAC_SDCA_MASTER_MUTE: c_uint = 0x01;
pub const TAC_SDCA_CHANNEL_MUTE: c_uint = 0x01;
pub const TAC_SDCA_CHANNEL_VOLUME: c_uint = 0x02;
pub const TAC_SDCA_POSTURENUMBER: c_uint = 0x10;
pub const TAC_SDCA_REQUESTED_PS: c_uint = 0x01;
pub const TAC_SDCA_ACTUAL_PS: c_uint = 0x10;
pub const TAC_SDCA_CHANNEL_GAIN: c_uint = 0x0B;
// 2. smart mic function
pub const TAC_FUNCTION_ID_SM: c_uint = 0x2;
pub const TAC_SDCA_ENT_IT11: c_uint = 0x1;
pub const TAC_SDCA_ENT_OT113: c_uint = 0x2;
pub const TAC_SDCA_ENT_CS11: c_uint = 0x3;
pub const TAC_SDCA_ENT_CS18: c_uint = 0x4;
pub const TAC_SDCA_ENT_FU113: c_uint = 0x5;
pub const TAC_SDCA_ENT_FU13: c_uint = 0x6;
pub const TAC_SDCA_ENT_FU11: c_uint = 0x8;
pub const TAC_SDCA_ENT_XU12: c_uint = 0xa;
pub const TAC_SDCA_ENT_CS113: c_uint = 0xc;
pub const TAC_SDCA_ENT_CX11: c_uint = 0xf;
pub const TAC_SDCA_ENT_PDE11: c_uint = 0x12;
pub const TAC_SDCA_ENT_PPU11: c_uint = 0x9;
// controls
pub const TAC_SDCA_CTL_USAGE: c_uint = 0x04;
pub const TAC_SDCA_CTL_IT_CLUSTER: c_uint = 0x10;
pub const TAC_SDCA_CTL_OT_DP_SEL: c_uint = 0x11;
pub const TAC_SDCA_CTL_XU_BYPASS: c_uint = 0x01;
// cx
pub const TAC_SDCA_CTL_CX_CLK_SEL: c_uint = 0x01;
// cs
pub const TAC_SDCA_CTL_CS_CLKVLD: c_uint = 0x02;
pub const TAC_SDCA_CTL_CS_SAMP_RATE_IDX: c_uint = 0x10;
// cs113 end
// ppu
pub const TAC_SDCA_CTL_PPU_POSTURE_NUM: c_uint = 0x10;
// 3. UAJ function
pub const TAC_FUNCTION_ID_UAJ: c_uint = 0x3;
pub const TAC_SDCA_ENT_PDE47: c_uint = 0x35;
pub const TAC_SDCA_ENT_PDE34: c_uint = 0x32;
pub const TAC_SDCA_ENT_FU41: c_uint = 0x26 /* user */;
pub const TAC_SDCA_ENT_IT41: c_uint = 0x07;
pub const TAC_SDCA_ENT_XU42: c_uint = 0x2C;
pub const TAC_SDCA_ENT_CS41: c_uint = 0x30;
pub const TAC_SDCA_ENT_OT45: c_uint = 0x0E;
pub const TAC_SDCA_ENT_IT33: c_uint = 0x03;
pub const TAC_SDCA_ENT_OT36: c_uint = 0x0A;
pub const TAC_SDCA_ENT_FU36: c_uint = 0x28;
pub const TAC_SDCA_ENT_CS36: c_uint = 0x2E;
pub const TAC_SDCA_ENT_GE35: c_uint = 0x3B /* 59 */;
pub const TAC_SDCA_CTL_SEL_MODE: c_uint = 0x1;
pub const TAC_SDCA_CTL_DET_MODE: c_uint = 0x2;
// 4. HID function
pub const TAC_FUNCTION_ID_HID: c_uint = 0x4;
pub const TAC_SDCA_ENT_HID1: c_uint = 0x1;
// HID Control Selectors
pub const TAC_SDCA_CTL_HIDTX_CURRENT_OWNER: c_uint = 0x10;
pub const TAC_SDCA_CTL_HIDTX_MESSAGE_OFFSET: c_uint = 0x12;
pub const TAC_SDCA_CTL_HIDTX_MESSAGE_LENGTH: c_uint = 0x13;
pub const TAC_SDCA_CTL_DETECTED_MODE: c_uint = 0x10;
pub const TAC_SDCA_CTL_SELECTED_MODE: c_uint = 0x11;
pub const TAC_BUF_ADDR_HID1: c_uint = 0x44007F80;
// DAI interfaces
pub const TAC5XX2_SPK: c_int = 0;
pub const TAC5XX2_DMIC: c_int = 2;
pub const TAC5XX2_UAJ: c_int = 3;
// Port numbers for DAIs
pub const TAC_SDW_PORT_NUM_SPK_PLAYBACK: c_int = 1;
pub const TAC_SDW_PORT_NUM_SPK_CAPTURE: c_int = 2;
pub const TAC_SDW_PORT_NUM_DMIC: c_int = 3;
pub const TAC_SDW_PORT_NUM_UAJ_PLAYBACK: c_int = 4;
pub const TAC_SDW_PORT_NUM_UAJ_CAPTURE: c_int = 7;
pub const TAC_SDW_PORT_NUM_IV_SENSE: c_int = 8;
