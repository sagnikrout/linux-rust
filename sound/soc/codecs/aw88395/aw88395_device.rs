//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/aw88395/aw88395_device.h
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
// aw88395_device.h --  AW88395 function for ALSA Audio Driver
//
// Copyright (c) 2022-2023 AWINIC Technology CO., LTD
//
// Author: Bruce zhao <zhaolei@awinic.com>
//

// Macro flag: #define AW88395_DSP_I2C_WRITES

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88395_DEV_STATUS {
    AW88395_DEV_PW_OFF = 0,
    AW88395_DEV_PW_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88395_DEV_FW_STATUS {
    AW88395_DEV_FW_FAILED = 0,
    AW88395_DEV_FW_OK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88395_DEV_MEMCLK {
    AW88395_DEV_MEMCLK_OSC = 0,
    AW88395_DEV_MEMCLK_PLL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88395_DEV_DSP_CFG {
    AW88395_DEV_DSP_WORK = 0,
    AW88395_DEV_DSP_BYPASS = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_profctrl_desc {
    pub cur_mode: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_volume_desc {
    pub init_volume: c_uint,
    pub mute_volume: c_uint,
    pub ctl_volume: c_uint,
    pub max_volume: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_dsp_mem_desc {
    pub dsp_madd_reg: c_uint,
    pub dsp_mdat_reg: c_uint,
    pub dsp_fw_base_addr: c_uint,
    pub dsp_cfg_base_addr: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_vmax_desc {
    pub init_vmax: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cali_delay_desc {
    pub delay: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cali_cfg {
    pub data: [u32; AW_CALI_CFG_NUM],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cali_backup_desc {
    pub dsp_ng_cfg: c_uint,
    pub dsp_lp_cfg: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_cali_desc {
    pub cali_re: u32,
    pub ra: u32,
    pub cali_switch: bool,
    pub cali_running: bool,
    pub cali_result: u16,
    pub store_vol: u16,
    pub cali_cfg: cali_cfg,
    pub backup_info: aw_cali_backup_desc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_container {
    pub len: c_int,
    pub __counted_by(len): u8 data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw_device {
    pub status: c_int,
    pub dsp_lock: mutex,
    pub prof_cur: c_uchar,
    pub prof_index: c_uchar,
    pub dsp_crc_st: c_uchar,
    pub dsp_cfg: c_uchar,
    pub chip_id: u16,
    pub channel: c_uint,
    pub fade_step: c_uint,
    pub prof_data_type: c_uint,
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub acf: *mut c_char,
    pub dsp_fw_len: u32,
    pub dsp_cfg_len: u32,
    pub platform: u8,
    pub fw_status: u8,
    pub fade_in_time: c_uint,
    pub fade_out_time: c_uint,
    pub prof_info: aw_prof_info,
    pub crc_dsp_cfg: aw_sec_data_desc,
    pub profctrl_desc: aw_profctrl_desc,
    pub volume_desc: aw_volume_desc,
    pub dsp_mem_desc: aw_dsp_mem_desc,
    pub vmax_desc: aw_vmax_desc,
    pub cali_delay_desc: aw_cali_delay_desc,
    pub cali_desc: aw_cali_desc,
}

extern "C" {
    pub fn aw88395_init(aw_dev: *mut aw_device, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_init(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_start(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_stop(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_fw_update(aw_dev: *mut aw_device, up_dsp_fw_en: bool, force_up_en: bool) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_set_volume(aw_dev: *mut aw_device, set_vol: c_ushort);
}
extern "C" {
    pub fn aw88395_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_set_profile_index(aw_dev: *mut aw_device, index: c_int) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_get_profile_index(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_get_profile_count(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_load_acf_check(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_cfg_load(aw_dev: *mut aw_device, aw_cfg: *mut aw_container) -> c_int;
}
extern "C" {
    pub fn aw88395_dev_mute(aw_dev: *mut aw_device, is_mute: bool);
}
