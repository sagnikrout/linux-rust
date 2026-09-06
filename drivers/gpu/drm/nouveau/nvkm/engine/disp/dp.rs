//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/dp.h
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


// SPDX-License-Identifier: MIT

extern "C" {
    pub fn nvkm_dp_disable(: *mut nvkm_outp, : *mut nvkm_ior);
}
extern "C" {
    pub fn nvkm_dp_enable(: *mut nvkm_outp, auxpwr: bool);
}
// DPCD Receiver Capabilities
pub const DPCD_RC00_DPCD_REV: c_uint = 0x00000;
pub const DPCD_RC01_MAX_LINK_RATE: c_uint = 0x00001;
pub const DPCD_RC02: c_uint = 0x00002;
pub const DPCD_RC02_ENHANCED_FRAME_CAP: c_uint = 0x80;
pub const DPCD_RC02_TPS3_SUPPORTED: c_uint = 0x40;
pub const DPCD_RC02_MAX_LANE_COUNT: c_uint = 0x1f;
pub const DPCD_RC03: c_uint = 0x00003;
pub const DPCD_RC03_TPS4_SUPPORTED: c_uint = 0x80;
pub const DPCD_RC03_MAX_DOWNSPREAD: c_uint = 0x01;
pub const DPCD_RC0E: c_uint = 0x0000e;
pub const DPCD_RC0E_AUX_RD_INTERVAL: c_uint = 0x7f;
pub const DPCD_RC10_SUPPORTED_LINK_RATES(i): c_uint = 0x00010;
pub const DPCD_RC10_SUPPORTED_LINK_RATES__SIZE: c_int = 16;
// DPCD Link Configuration
pub const DPCD_LC00_LINK_BW_SET: c_uint = 0x00100;
pub const DPCD_LC01: c_uint = 0x00101;
pub const DPCD_LC01_ENHANCED_FRAME_EN: c_uint = 0x80;
pub const DPCD_LC01_LANE_COUNT_SET: c_uint = 0x1f;
pub const DPCD_LC02: c_uint = 0x00102;
pub const DPCD_LC02_TRAINING_PATTERN_SET: c_uint = 0x0f;
pub const DPCD_LC02_SCRAMBLING_DISABLE: c_uint = 0x20;

pub const DPCD_LC03_MAX_PRE_EMPHASIS_REACHED: c_uint = 0x20;
pub const DPCD_LC03_PRE_EMPHASIS_SET: c_uint = 0x18;
pub const DPCD_LC03_MAX_SWING_REACHED: c_uint = 0x04;
pub const DPCD_LC03_VOLTAGE_SWING_SET: c_uint = 0x03;
pub const DPCD_LC0F: c_uint = 0x0010f;
pub const DPCD_LC0F_LANE1_MAX_POST_CURSOR2_REACHED: c_uint = 0x40;
pub const DPCD_LC0F_LANE1_POST_CURSOR2_SET: c_uint = 0x30;
pub const DPCD_LC0F_LANE0_MAX_POST_CURSOR2_REACHED: c_uint = 0x04;
pub const DPCD_LC0F_LANE0_POST_CURSOR2_SET: c_uint = 0x03;
pub const DPCD_LC10: c_uint = 0x00110;
pub const DPCD_LC10_LANE3_MAX_POST_CURSOR2_REACHED: c_uint = 0x40;
pub const DPCD_LC10_LANE3_POST_CURSOR2_SET: c_uint = 0x30;
pub const DPCD_LC10_LANE2_MAX_POST_CURSOR2_REACHED: c_uint = 0x04;
pub const DPCD_LC10_LANE2_POST_CURSOR2_SET: c_uint = 0x03;
pub const DPCD_LC15_LINK_RATE_SET: c_uint = 0x00115;
pub const DPCD_LC15_LINK_RATE_SET_MASK: c_uint = 0x07;
// DPCD Link/Sink Status
pub const DPCD_LS02: c_uint = 0x00202;
pub const DPCD_LS02_LANE1_SYMBOL_LOCKED: c_uint = 0x40;
pub const DPCD_LS02_LANE1_CHANNEL_EQ_DONE: c_uint = 0x20;
pub const DPCD_LS02_LANE1_CR_DONE: c_uint = 0x10;
pub const DPCD_LS02_LANE0_SYMBOL_LOCKED: c_uint = 0x04;
pub const DPCD_LS02_LANE0_CHANNEL_EQ_DONE: c_uint = 0x02;
pub const DPCD_LS02_LANE0_CR_DONE: c_uint = 0x01;
pub const DPCD_LS03: c_uint = 0x00203;
pub const DPCD_LS03_LANE3_SYMBOL_LOCKED: c_uint = 0x40;
pub const DPCD_LS03_LANE3_CHANNEL_EQ_DONE: c_uint = 0x20;
pub const DPCD_LS03_LANE3_CR_DONE: c_uint = 0x10;
pub const DPCD_LS03_LANE2_SYMBOL_LOCKED: c_uint = 0x04;
pub const DPCD_LS03_LANE2_CHANNEL_EQ_DONE: c_uint = 0x02;
pub const DPCD_LS03_LANE2_CR_DONE: c_uint = 0x01;
pub const DPCD_LS04: c_uint = 0x00204;
pub const DPCD_LS04_LINK_STATUS_UPDATED: c_uint = 0x80;
pub const DPCD_LS04_DOWNSTREAM_PORT_STATUS_CHANGED: c_uint = 0x40;
pub const DPCD_LS04_INTERLANE_ALIGN_DONE: c_uint = 0x01;
pub const DPCD_LS06: c_uint = 0x00206;
pub const DPCD_LS06_LANE1_PRE_EMPHASIS: c_uint = 0xc0;
pub const DPCD_LS06_LANE1_VOLTAGE_SWING: c_uint = 0x30;
pub const DPCD_LS06_LANE0_PRE_EMPHASIS: c_uint = 0x0c;
pub const DPCD_LS06_LANE0_VOLTAGE_SWING: c_uint = 0x03;
pub const DPCD_LS07: c_uint = 0x00207;
pub const DPCD_LS07_LANE3_PRE_EMPHASIS: c_uint = 0xc0;
pub const DPCD_LS07_LANE3_VOLTAGE_SWING: c_uint = 0x30;
pub const DPCD_LS07_LANE2_PRE_EMPHASIS: c_uint = 0x0c;
pub const DPCD_LS07_LANE2_VOLTAGE_SWING: c_uint = 0x03;
pub const DPCD_LS0C: c_uint = 0x0020c;
pub const DPCD_LS0C_LANE3_POST_CURSOR2: c_uint = 0xc0;
pub const DPCD_LS0C_LANE2_POST_CURSOR2: c_uint = 0x30;
pub const DPCD_LS0C_LANE1_POST_CURSOR2: c_uint = 0x0c;
pub const DPCD_LS0C_LANE0_POST_CURSOR2: c_uint = 0x03;
// DPCD Sink Control
pub const DPCD_SC00: c_uint = 0x00600;
pub const DPCD_SC00_SET_POWER: c_uint = 0x03;
pub const DPCD_SC00_SET_POWER_D0: c_uint = 0x01;
pub const DPCD_SC00_SET_POWER_D3: c_uint = 0x03;
pub const DPCD_LTTPR_REV: c_uint = 0xf0000;
pub const DPCD_LTTPR_MODE: c_uint = 0xf0003;
pub const DPCD_LTTPR_MODE_TRANSPARENT: c_uint = 0x55;
pub const DPCD_LTTPR_MODE_NON_TRANSPARENT: c_uint = 0xaa;

