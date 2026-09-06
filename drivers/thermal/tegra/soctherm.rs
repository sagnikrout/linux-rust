//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/thermal/tegra/soctherm.h
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
// Copyright (c) 2014-2016, NVIDIA CORPORATION.  All rights reserved.
//
// This software is licensed under the terms of the GNU General Public
// License version 2, as published by the Free Software Foundation, and
// may be copied, distributed, and modified under those terms.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
pub const THERMCTL_LEVEL0_GROUP_CPU: c_uint = 0x0;
pub const THERMCTL_LEVEL0_GROUP_GPU: c_uint = 0x4;
pub const THERMCTL_LEVEL0_GROUP_MEM: c_uint = 0x8;
pub const THERMCTL_LEVEL0_GROUP_TSENSE: c_uint = 0xc;
pub const SENSOR_CONFIG2: c_int = 8;

pub const SENSOR_CONFIG2_THERMA_SHIFT: c_int = 16;
pub const SENSOR_CONFIG2_THERMB_MASK: c_uint = 0xffff;
pub const SENSOR_CONFIG2_THERMB_SHIFT: c_int = 0;
pub const THERMCTL_THERMTRIP_CTL: c_uint = 0x80;
// BITs are defined in device file
pub const THERMCTL_INTR_ENABLE: c_uint = 0x88;
pub const THERMCTL_INTR_DISABLE: c_uint = 0x8c;
pub const TH_INTR_UP_DN_EN: c_uint = 0x3;

pub const SENSOR_PDIV: c_uint = 0x1c0;

pub const SENSOR_HOTSPOT_OFF: c_uint = 0x1c4;

pub const SENSOR_TEMP1: c_uint = 0x1c8;

pub const SENSOR_TEMP1_GPU_TEMP_MASK: c_uint = 0xffff;
pub const SENSOR_TEMP2: c_uint = 0x1cc;

pub const SENSOR_TEMP2_PLLX_TEMP_MASK: c_uint = 0xffff;
pub const FUSE_VSENSOR_CALIB: c_uint = 0x08c;
pub const FUSE_TSENSOR_COMMON: c_uint = 0x180;
//
// struct tegra_tsensor_group - SOC_THERM sensor group data
// @name: short name of the temperature sensor group
// @id: numeric ID of the temperature sensor group
// @sensor_temp_offset: offset of the SENSOR_TEMP* register
// @sensor_temp_mask: bit mask for this sensor group in SENSOR_TEMP* register
// @pdiv: the sensor count post-divider to use during runtime
// @pdiv_ate: the sensor count post-divider used during automated test
// @pdiv_mask: register bitfield mask for the PDIV field for this sensor
// @pllx_hotspot_diff: hotspot offset from the PLLX sensor, must be 0 for
// @pllx_hotspot_mask: register bitfield mask for the HOTSPOT field
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_tsensor_group {
    pub name: *const c_char,
    pub id: u8,
    pub sensor_temp_offset: u16,
    pub sensor_temp_mask: u32,
    pub pdiv_mask: u32 pdiv, pdiv_ate,,
    pub pllx_hotspot_mask: u32 pllx_hotspot_diff,,
    pub thermtrip_enable_mask: u32,
    pub thermtrip_any_en_mask: u32,
    pub thermtrip_threshold_mask: u32,
    pub thermctl_isr_mask: u32,
    pub thermctl_lvl0_offset: u16,
    pub thermctl_lvl0_up_thresh_mask: u32,
    pub thermctl_lvl0_dn_thresh_mask: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_tsensor_configuration {
    pub tsample_ate: u32 tall, tiddq_en, ten_count, pdiv, pdiv_ate, tsample,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_tsensor {
    pub name: *const c_char,
    pub base: u32,
    pub config: *const tegra_tsensor_configuration,
    pub calib_fuse_offset: u32,
//
// Correction values used to modify values read from
// calibration fuses
//
    pub fuse_corr_beta: s32 fuse_corr_alpha,,
    pub group: *const tegra_tsensor_group,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsensor_group_thermtrips {
    pub id: u8,
    pub temp: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_soctherm_fuse {
    pub fuse_base_cp_shift: u32 fuse_base_cp_mask,,
    pub fuse_shift_cp_shift: u32 fuse_shift_cp_mask,,
    pub fuse_base_ft_shift: u32 fuse_base_ft_mask,,
    pub fuse_shift_ft_shift: u32 fuse_shift_ft_mask,,
    pub fuse_spare_realignment: u32 fuse_common_reg,,
    pub nominal_calib_ft: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tsensor_shared_calib {
    pub base_ft: u32 base_cp,,
    pub actual_temp_ft: u32 actual_temp_cp,,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_soctherm_soc {
    pub tsensors: *const tegra_tsensor,
    pub num_tsensors: c_uint,
    pub ttgs: *const tegra_tsensor_group,
    pub num_ttgs: c_uint,
    pub tfuse: *const tegra_soctherm_fuse,
    pub thresh_grain: c_int,
    pub bptt: c_uint,
    pub use_ccroc: bool,
    pub thermtrips: *mut tsensor_group_thermtrips,
}

