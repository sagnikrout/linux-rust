//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/tas2781-dsp.h
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
// ALSA SoC Texas Instruments TAS2781 Audio Smart Amplifier
//
// Copyright (C) 2022 - 2025 Texas Instruments Incorporated
// https://www.ti.com
//
// The TAS2781 driver implements a flexible and configurable
// algo coefficient setting for one, two, or even multiple
// TAS2781 chips.
//
// Author: Shenghao Ding <shenghao-ding@ti.com>
// Author: Kevin Lu <kevin-lu@ti.com>
//
pub const MAIN_ALL_DEVICES: c_uint = 0x0d;
pub const MAIN_DEVICE_A: c_uint = 0x01;
pub const MAIN_DEVICE_B: c_uint = 0x08;
pub const MAIN_DEVICE_C: c_uint = 0x10;
pub const MAIN_DEVICE_D: c_uint = 0x14;
pub const COEFF_DEVICE_A: c_uint = 0x03;
pub const COEFF_DEVICE_B: c_uint = 0x0a;
pub const COEFF_DEVICE_C: c_uint = 0x11;
pub const COEFF_DEVICE_D: c_uint = 0x15;
pub const PRE_DEVICE_A: c_uint = 0x04;
pub const PRE_DEVICE_B: c_uint = 0x0b;
pub const PRE_DEVICE_C: c_uint = 0x12;
pub const PRE_DEVICE_D: c_uint = 0x16;
pub const PPC3_VERSION_BASE: c_uint = 0x4100;
pub const PPC3_VERSION_TAS2781_BASIC_MIN: c_uint = 0x14600;
pub const PPC3_VERSION_TAS2781_ALPHA_MIN: c_uint = 0x4a00;
pub const PPC3_VERSION_TAS2781_BETA_MIN: c_uint = 0x19400;
pub const PPC3_VERSION_TAS5825_BASE: c_uint = 0x114200;
pub const TASDEVICE_DEVICE_SUM: c_int = 8;
pub const TASDEVICE_CONFIG_SUM: c_int = 64;
pub const TASDEVICE_MAX_CHANNELS: c_int = 8;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tasdevice_dsp_dev_idx {
    TASDEVICE_DSP_TAS_2555 = 0,
    TASDEVICE_DSP_TAS_2555_STEREO,
    TASDEVICE_DSP_TAS_2557_MONO,
    TASDEVICE_DSP_TAS_2557_DUAL_MONO,
    TASDEVICE_DSP_TAS_2559,
    TASDEVICE_DSP_TAS_2563,
    TASDEVICE_DSP_TAS_2563_DUAL_MONO = 7,
    TASDEVICE_DSP_TAS_2563_QUAD,
    TASDEVICE_DSP_TAS_2563_21,
    TASDEVICE_DSP_TAS_2781,
    TASDEVICE_DSP_TAS_2781_DUAL_MONO,
    TASDEVICE_DSP_TAS_2781_21,
    TASDEVICE_DSP_TAS_2781_QUAD,
    TASDEVICE_DSP_TAS_5825_MONO,
    TASDEVICE_DSP_TAS_5825_DUAL,
    TASDEVICE_DSP_TAS_MAX_DEVICE
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_fw_fixed_hdr {
    pub fwsize: c_uint,
    pub ppcver: c_uint,
    pub drv_ver: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_dspfw_hdr {
    pub fixed_hdr: tasdevice_fw_fixed_hdr,
    pub device_family: c_ushort,
    pub device: c_ushort,
    pub ndev: c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdev_blk {
    pub nr_retry: c_int,
    pub type: c_uint,
    pub is_pchksum_present: c_uchar,
    pub pchksum: c_uchar,
    pub is_ychksum_present: c_uchar,
    pub ychksum: c_uchar,
    pub nr_cmds: c_uint,
    pub blk_size: c_uint,
    pub nr_subblocks: c_uint,
// fixed m68k compiling issue, storing the dev_idx as a member of block
// can reduce unnecessary timeand system resource comsumption of
// dev_idx mapping every time the block data writing to the dsp.
//
    pub dev_idx: c_uchar,
    pub data: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_data {
    pub name: [c_char; 64],
    pub nr_blk: c_uint,
    pub dev_blks: *mut tasdev_blk,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_prog {
    pub prog_size: c_uint,
    pub dev_data: tasdevice_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_config {
    pub cfg_size: c_uint,
    pub name: [c_char; 64],
    pub dev_data: tasdevice_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_calibration {
    pub dev_data: tasdevice_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fct_param_address {
// Thermal data for PG 1.0 device
    pub thr: [c_uchar; 3],
// Thermal data for PG 2.0 device
    pub thr2: [c_uchar; 3],
// Pilot tone enable flag, usually the sine wave
    pub plt_flg: [c_uchar; 3],
// Pilot tone gain for calibration
    pub sin_gn: [c_uchar; 3],
// Pilot tone gain for calibration
    pub sin_gn2: [c_uchar; 3],
// high 32-bit of real-time spk impedance
    pub r0_reg: [c_uchar; 3],
// check spk connection
    pub tf_reg: [c_uchar; 3],
// check spk resonant frequency
    pub a1_reg: [c_uchar; 3],
// check spk resonant frequency
    pub a2_reg: [c_uchar; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_fw {
    pub fw_hdr: tasdevice_dspfw_hdr,
    pub nr_programs: c_ushort,
    pub programs: *mut tasdevice_prog,
    pub nr_configurations: c_ushort,
    pub configs: *mut tasdevice_config,
    pub nr_calibrations: c_ushort,
    pub calibrations: *mut tasdevice_calibration,
    pub fct_par_addr: fct_param_address,
    pub dev: *mut device,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tasdevice_fw_state {
// Driver in startup mode, not load any firmware.
    TASDEVICE_DSP_FW_PENDING,
// DSP firmware in the system, but parsing error.
    TASDEVICE_DSP_FW_FAIL,
//
// Only RCA (Reconfigurable Architecture) firmware load
// successfully.
//
    TASDEVICE_RCA_FW_OK,
// Both RCA and DSP firmware load successfully.
    TASDEVICE_DSP_FW_ALL_OK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tasdevice_bin_blk_type {
    TASDEVICE_BIN_BLK_COEFF = 1,
    TASDEVICE_BIN_BLK_POST_POWER_UP,
    TASDEVICE_BIN_BLK_PRE_SHUTDOWN,
    TASDEVICE_BIN_BLK_PRE_POWER_UP,
    TASDEVICE_BIN_BLK_POST_SHUTDOWN
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_rca_hdr {
    pub img_sz: c_uint,
    pub checksum: c_uint,
    pub binary_version_num: c_uint,
    pub drv_fw_version: c_uint,
    pub plat_type: c_uchar,
    pub dev_family: c_uchar,
    pub reserve: c_uchar,
    pub ndev: c_uchar,
    pub devs: [c_uchar; TASDEVICE_DEVICE_SUM],
    pub nconfig: c_uint,
    pub config_size: [c_uint; TASDEVICE_CONFIG_SUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdev_blk_data {
    pub dev_idx: c_uchar,
    pub block_type: c_uchar,
    pub yram_checksum: c_ushort,
    pub block_size: c_uint,
    pub n_subblks: c_uint,
    pub regdata: *mut c_uchar,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_config_info {
    pub nblocks: c_uint,
    pub real_nblocks: c_uint,
    pub active_dev: c_uchar,
    pub blk_data: *mut tasdev_blk_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tasdevice_rca {
    pub fw_hdr: tasdevice_rca_hdr,
    pub ncfgs: c_int,
    pub cfg_info: *mut tasdevice_config_info,
    pub profile_cfg_id: c_int,
//
// Used among SmartAMP for PDM microphone recording or IV data
// capture.
//
    pub capture_profile_id: c_int,
//
// Since version 0x105, the keyword 'init' was introduced into the
// profile, which is used for chip initialization, particularly to
// store common settings for other non-initialization profiles.
// if (init_profile_id < 0)
// No init profile inside the RCA firmware.
//
    pub init_profile_id: c_int,
}

extern "C" {
    pub fn tasdevice_config_info_remove(context: *mut c_void);
}
extern "C" {
    pub fn tasdevice_dsp_remove(context: *mut c_void);
}
extern "C" {
    pub fn tasdevice_dsp_parser(context: *mut c_void) -> c_int;
}
extern "C" {
    pub fn tasdevice_rca_parser(context: *mut c_void, fmw: *const firmware) -> c_int;
}
extern "C" {
    pub fn tasdevice_dsp_remove(context: *mut c_void);
}
extern "C" {
    pub fn tasdevice_calbin_remove(context: *mut c_void);
}
extern "C" {
    pub fn tasdevice_prmg_load(context: *mut c_void, prm_no: c_int) -> c_int;
}
extern "C" {
    pub fn tasdevice_tuning_switch(context: *mut c_void, state: c_int, is_cap: bool);
}
