//! Automatically rewritten from C Header to Rust Module
//! Source: include/sound/aw88399.h
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
// linux/sound/aw88399.h --  Platform data for AW88399
//
// Copyright (c) 2023 AWINIC Technology CO., LTD
//
// Author: Weidong Wang <wangweidong.a@awinic.com>
//

// registers list

// EF_VSN_GESLP_H bit 9:0 (EFRH3 0x75)

// EF_VSN_GESLP_L bit 9:0 (EFRL3 0x79)

pub const AW88399_CHIP_ID: c_uint = 0x2183;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88399_DEV_STATUS {
    AW88399_DEV_PW_OFF = 0,
    AW88399_DEV_PW_ON,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88399_DEV_FW_STATUS {
    AW88399_DEV_FW_FAILED = 0,
    AW88399_DEV_FW_OK,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88399_DEV_MEMCLK {
    AW88399_DEV_MEMCLK_OSC = 0,
    AW88399_DEV_MEMCLK_PLL = 1,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AW88399_DEV_DSP_CFG {
    AW88399_DEV_DSP_WORK = 0,
    AW88399_DEV_DSP_BYPASS = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct aw88399 {
    pub aw_pa: *mut aw_device,
    pub lock: mutex,
    pub reset_gpio: *mut gpio_desc,
    pub start_work: delayed_work,
    pub regmap: *mut regmap,
    pub aw_cfg: *mut aw_container,
    pub check_val: c_uint,
    pub crc_init_val: c_uint,
    pub vcalb_init_val: c_uint,
    pub dither_st: c_uint,
    pub bsts_unreliable: bool,
    pub fw_needs_reload: bool,
}

extern "C" {
    pub fn aw_dev_check_syspll(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw_dev_dsp_enable(aw_dev: *mut aw_device, is_enable: bool);
}
extern "C" {
    pub fn aw_dev_get_dsp_status(aw_dev: *mut aw_device) -> c_int;
}
extern "C" {
    pub fn aw_dev_set_volume(aw_dev: *mut aw_device, value: c_uint) -> c_int;
}
extern "C" {
    pub fn aw_dev_update_cali_re(cali_desc: *mut aw_cali_desc) -> c_int;
}
extern "C" {
    pub fn aw88399_dev_get_prof_name(aw_dev: *mut aw_device, index: c_int, prof_name: *mut c_char) -> c_int;
}
extern "C" {
    pub fn aw88399_dev_mute(aw_dev: *mut aw_device, is_mute: bool);
}
extern "C" {
    pub fn aw88399_dev_set_channel(aw88399: *mut aw88399, channel: c_int);
}
extern "C" {
    pub fn aw88399_hw_reset(aw88399: *mut aw88399);
}
extern "C" {
    pub fn aw88399_init(aw88399: *mut aw88399, i2c: *mut i2c_client, regmap: *mut regmap) -> c_int;
}
extern "C" {
    pub fn aw88399_request_firmware_file(aw88399: *mut aw88399) -> c_int;
}
extern "C" {
    pub fn aw88399_start(aw88399: *mut aw88399, sync_start: bool);
}
extern "C" {
    pub fn aw88399_startup_work(work: *mut work_struct);
}
extern "C" {
    pub fn aw88399_stop(aw_dev: *mut aw_device) -> c_int;
}
