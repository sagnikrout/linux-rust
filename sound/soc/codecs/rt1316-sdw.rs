//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1316-sdw.h
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
// rt1316-sdw.h -- RT1316 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2021 Realtek Semiconductor Corp.
//

// RT1316 SDCA Control - function number
pub const FUNC_NUM_SMART_AMP: c_uint = 0x04;
// RT1316 SDCA entity
pub const RT1316_SDCA_ENT_PDE23: c_uint = 0x31;
pub const RT1316_SDCA_ENT_PDE27: c_uint = 0x32;
pub const RT1316_SDCA_ENT_PDE22: c_uint = 0x33;
pub const RT1316_SDCA_ENT_PDE24: c_uint = 0x34;
pub const RT1316_SDCA_ENT_XU24: c_uint = 0x24;
pub const RT1316_SDCA_ENT_FU21: c_uint = 0x03;
pub const RT1316_SDCA_ENT_UDMPU21: c_uint = 0x02;
// RT1316 SDCA control
pub const RT1316_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT1316_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT1316_SDCA_CTL_BYPASS: c_uint = 0x01;
pub const RT1316_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT1316_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT1316_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0x10;
// RT1316 SDCA channel
pub const CH_L: c_uint = 0x01;
pub const CH_R: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1316_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub bq_params: *mut c_uchar,
    pub bq_params_cnt: c_uint,
}
