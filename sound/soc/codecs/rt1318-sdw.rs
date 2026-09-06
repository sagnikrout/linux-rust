//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1318-sdw.h
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
// rt1318-sdw.h -- RT1318 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2022 Realtek Semiconductor Corp.
//

// imp-defined registers
pub const RT1318_SAPU_SM: c_uint = 0x3203;
pub const R1318_TCON: c_uint = 0xc203;
pub const R1318_TCON_RELATED_1: c_uint = 0xc206;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_0: c_uint = 0xdb00;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_L_4: c_uint = 0xdb08;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_R_4: c_uint = 0xdd08;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_L_6: c_uint = 0xdb12;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_R_6: c_uint = 0xdd12;
pub const RT1318_INIT_RECIPROCAL_REG_L_24: c_uint = 0xdbb5;
pub const RT1318_INIT_RECIPROCAL_REG_L_23_16: c_uint = 0xdbb6;
pub const RT1318_INIT_RECIPROCAL_REG_L_15_8: c_uint = 0xdbb7;
pub const RT1318_INIT_RECIPROCAL_REG_L_7_0: c_uint = 0xdbb8;
pub const RT1318_INIT_RECIPROCAL_REG_R_24: c_uint = 0xddb5;
pub const RT1318_INIT_RECIPROCAL_REG_R_23_16: c_uint = 0xddb6;
pub const RT1318_INIT_RECIPROCAL_REG_R_15_8: c_uint = 0xddb7;
pub const RT1318_INIT_RECIPROCAL_REG_R_7_0: c_uint = 0xddb8;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_24: c_uint = 0xdbc5;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_23_16: c_uint = 0xdbc6;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_15_8: c_uint = 0xdbc7;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_7_0: c_uint = 0xdbc8;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_24: c_uint = 0xddc5;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_23_16: c_uint = 0xddc6;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_15_8: c_uint = 0xddc7;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_7_0: c_uint = 0xddc8;
pub const RT1318_R0_COMPARE_FLAG_L: c_uint = 0xdb35;
pub const RT1318_R0_COMPARE_FLAG_R: c_uint = 0xdd35;
pub const RT1318_STP_INITIAL_RS_TEMP_H: c_uint = 0xdd93;
pub const RT1318_STP_INITIAL_RS_TEMP_L: c_uint = 0xdd94;
// RT1318 SDCA Control - function number
pub const FUNC_NUM_SMART_AMP: c_uint = 0x04;
// RT1318 SDCA entity
pub const RT1318_SDCA_ENT_PDE23: c_uint = 0x31;
pub const RT1318_SDCA_ENT_XU24: c_uint = 0x24;
pub const RT1318_SDCA_ENT_FU21: c_uint = 0x03;
pub const RT1318_SDCA_ENT_UDMPU21: c_uint = 0x02;
pub const RT1318_SDCA_ENT_CS21: c_uint = 0x21;
pub const RT1318_SDCA_ENT_SAPU: c_uint = 0x29;
// RT1318 SDCA control
pub const RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT1318_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT1318_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT1318_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT1318_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0x10;
pub const RT1318_SDCA_CTL_SAPU_PROTECTION_MODE: c_uint = 0x10;
pub const RT1318_SDCA_CTL_SAPU_PROTECTION_STATUS: c_uint = 0x11;
// RT1318 SDCA channel
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;
// sample frequency index
pub const RT1318_SDCA_RATE_16000HZ: c_uint = 0x04;
pub const RT1318_SDCA_RATE_32000HZ: c_uint = 0x07;
pub const RT1318_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT1318_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT1318_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT1318_SDCA_RATE_192000HZ: c_uint = 0x0d;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1318_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
}
