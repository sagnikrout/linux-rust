//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/sta350.h
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
//
// Platform data for ST STA350 ASoC codec driver.
//
// Copyright: 2014 Raumfeld GmbH
// Author: Sven Brandau <info@brandau.biz>
//
pub const STA350_OCFG_2CH: c_int = 0;
pub const STA350_OCFG_2_1CH: c_int = 1;
pub const STA350_OCFG_1CH: c_int = 3;
pub const STA350_OM_CH1: c_int = 0;
pub const STA350_OM_CH2: c_int = 1;
pub const STA350_OM_CH3: c_int = 2;
pub const STA350_THERMAL_ADJUSTMENT_ENABLE: c_int = 1;
pub const STA350_THERMAL_RECOVERY_ENABLE: c_int = 2;
pub const STA350_FAULT_DETECT_RECOVERY_BYPASS: c_int = 1;
pub const STA350_FFX_PM_DROP_COMP: c_int = 0;
pub const STA350_FFX_PM_TAPERED_COMP: c_int = 1;
pub const STA350_FFX_PM_FULL_POWER: c_int = 2;
pub const STA350_FFX_PM_VARIABLE_DROP_COMP: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sta350_platform_data {
    pub output_conf: u8,
    pub ch1_output_mapping: u8,
    pub ch2_output_mapping: u8,
    pub ch3_output_mapping: u8,
    pub ffx_power_output_mode: u8,
    pub drop_compensation_ns: u8,
    pub powerdown_delay_divider: u8,
    pub thermal_warning_recovery:1: c_uint,
    pub thermal_warning_adjustment:1: c_uint,
    pub fault_detect_recovery:1: c_uint,
    pub oc_warning_adjustment:1: c_uint,
    pub max_power_use_mpcc:1: c_uint,
    pub max_power_correction:1: c_uint,
    pub am_reduction_mode:1: c_uint,
    pub odd_pwm_speed_mode:1: c_uint,
    pub distortion_compensation:1: c_uint,
    pub invalid_input_detect_mute:1: c_uint,
    pub activate_mute_output:1: c_uint,
    pub bridge_immediate_off:1: c_uint,
    pub noise_shape_dc_cut:1: c_uint,
    pub powerdown_master_vol:1: c_uint,
}
