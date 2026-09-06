//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/rt1320-sdw.h
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
// rt1320-sdw.h -- RT1320 SDCA ALSA SoC audio driver header
//
// Copyright(c) 2024 Realtek Semiconductor Corp.
//

pub const RT1320_DEV_ID: c_uint = 0x6981;
pub const RT1321_DEV_ID: c_uint = 0x7045;
pub const RT1321_DEV_HV_VA0_ID: c_uint = 0x6997;
pub const RT1321_DEV_HV_VA1_ID: c_uint = 0x7071;
// imp-defined registers
pub const RT1320_DEV_VERSION_ID_1: c_uint = 0xc404;
pub const RT1320_DEV_ID_1: c_uint = 0xc405;
pub const RT1320_DEV_ID_0: c_uint = 0xc406;
pub const RT1320_HV_DEV_ID_0: c_uint = 0xf622;
pub const RT1320_HV_DEV_ID_1: c_uint = 0xf623;
pub const RT1320_POWER_STATE: c_uint = 0xc560;
pub const RT1321_PATCH_MAIN_VER: c_uint = 0x1000cffe;
pub const RT1321_PATCH_BETA_VER: c_uint = 0x1000cfff;
pub const RT1320_KR0_STATUS_CNT: c_uint = 0x1000f008;
pub const RT1320_KR0_INT_READY: c_uint = 0x1000f021;
pub const RT1320_HIFI_VER_0: c_uint = 0x3fe2e000;
pub const RT1320_HIFI_VER_1: c_uint = 0x3fe2e001;
pub const RT1320_HIFI_VER_2: c_uint = 0x3fe2e002;
pub const RT1320_HIFI_VER_3: c_uint = 0x3fe2e003;
// RT1320 SDCA Control - function number
pub const FUNC_NUM_AMP: c_uint = 0x04;
pub const FUNC_NUM_MIC: c_uint = 0x02;
// RT1320 SDCA entity
pub const RT1320_SDCA_ENT0: c_uint = 0x00;
pub const RT1320_SDCA_ENT_PDE11: c_uint = 0x2a;
pub const RT1320_SDCA_ENT_PDE23: c_uint = 0x33;
pub const RT1320_SDCA_ENT_PDE27: c_uint = 0x27;
pub const RT1320_SDCA_ENT_FU14: c_uint = 0x32;
pub const RT1320_SDCA_ENT_FU21: c_uint = 0x03;
pub const RT1320_SDCA_ENT_FU113: c_uint = 0x30;
pub const RT1320_SDCA_ENT_CS14: c_uint = 0x13;
pub const RT1320_SDCA_ENT_CS21: c_uint = 0x21;
pub const RT1320_SDCA_ENT_CS113: c_uint = 0x12;
pub const RT1320_SDCA_ENT_SAPU: c_uint = 0x29;
pub const RT1320_SDCA_ENT_PPU21: c_uint = 0x04;
// RT1320 SDCA control
pub const RT1320_SDCA_CTL_SAMPLE_FREQ_INDEX: c_uint = 0x10;
pub const RT1320_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT1320_SDCA_CTL_ACTUAL_POWER_STATE: c_uint = 0x10;
pub const RT1320_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT1320_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT1320_SDCA_CTL_SAPU_PROTECTION_MODE: c_uint = 0x10;
pub const RT1320_SDCA_CTL_SAPU_PROTECTION_STATUS: c_uint = 0x11;
pub const RT1320_SDCA_CTL_POSTURE_NUMBER: c_uint = 0x10;
pub const RT1320_SDCA_CTL_FUNC_STATUS: c_uint = 0x10;
// RT1320 SDCA channel
pub const CH_01: c_uint = 0x01;
pub const CH_02: c_uint = 0x02;
// Function_Status

// Sample Frequency Index
pub const RT1320_SDCA_RATE_16000HZ: c_uint = 0x04;
pub const RT1320_SDCA_RATE_32000HZ: c_uint = 0x07;
pub const RT1320_SDCA_RATE_44100HZ: c_uint = 0x08;
pub const RT1320_SDCA_RATE_48000HZ: c_uint = 0x09;
pub const RT1320_SDCA_RATE_96000HZ: c_uint = 0x0b;
pub const RT1320_SDCA_RATE_192000HZ: c_uint = 0x0d;
//
// The version id will be useful to distinguish the capability between the different IC versions.
// Currently, VA and VB have different DSP FW versions.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1320_version_id {
    RT1320_VA,
    RT1320_VB,
    RT1320_VC,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1321_version_id {
    RT1321_VA0,
    RT1321_VA1,
    RT1321_VA2,
}

pub const RT1320_VER_B_ID: c_uint = 0x07392238;

pub const RT1320_FW_PARAM_ADDR: c_uint = 0x3fc2ab80;
pub const RT1320_CMD_ID: c_uint = 0x3fc2ab81;
pub const RT1320_CMD_PARAM_ADDR: c_uint = 0x3fc2ab90;
pub const RT1320_DSPFW_STATUS_ADDR: c_uint = 0x3fc2bfc4;
pub const RT1321_FW_PARAM_ADDR: c_uint = 0x3fc2d300;
pub const RT1321_CMD_ID: c_uint = 0x3fc2d301;
pub const RT1321_CMD_PARAM_ADDR: c_uint = 0x3fc2d310;
pub const RT1321_DSPFW_STATUS_ADDR: c_uint = 0x3fc2dfc4;
// FW parameter id 6, 7
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1320_datafixpoint {
    pub silencedetect: c_int,
    pub r0: c_int,
    pub meanr0: c_int,
    pub advancegain: c_int,
    pub ts: c_int,
    pub re: c_int,
    pub t: c_int,
    pub invrs: c_int,
}

// FW parameter id 1300
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1320_paramcmd {
    pub moudleid: c_uchar,
    pub commandtype: c_uchar,
    pub reserved1: c_ushort,
    pub commandlength: c_uint,
    pub reserved2: c_longlong,
    pub paramid: c_uint,
    pub paramlength: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1320_fw_cmdid {
    RT1320_FW_READY,
    RT1320_SET_PARAM,
    RT1320_GET_PARAM,
    RT1320_GET_POOLSIZE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1320_power_state {
    RT1320_NORMAL_STATE = 0x18,
    RT1320_K_R0_STATE = 0x1b,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rt1320_rw_type {
    RT1320_BRA_WRITE = 0,
    RT1320_BRA_READ = 1,
    RT1320_PARAM_WRITE = 2,
    RT1320_PARAM_READ = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rt1320_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub mbq_regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub version_id: c_int,
    pub brown_out: c_int,
    pub dev_id: c_uint,
    pub fu_dapm_mute: bool,
    pub fu_mixer_mute: [bool; 4],
    pub r0_l_reg: c_ulonglong,
    pub r0_r_reg: c_ulonglong,
    pub r0_l_calib: c_uint,
    pub r0_r_calib: c_uint,
    pub temp_l_calib: c_uint,
    pub temp_r_calib: c_uint,
    pub dspfw_name: *const c_char,
    pub cali_done: bool,
    pub fw_load_done: bool,
    pub rae_update_done: bool,
    pub load_dspfw_work: work_struct,
    pub bra_msg: sdw_bpt_msg,
}
