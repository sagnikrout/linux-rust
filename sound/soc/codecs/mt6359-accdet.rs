//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/mt6359-accdet.h
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
// Copyright (C) 2021 MediaTek Inc.
// Author: Argus Lin <argus.lin@mediatek.com>
//

pub const MT6359_ACCDET_NUM_BUTTONS: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum eint_moisture_status {
    M_PLUG_IN =		0,
    M_WATER_IN =		1,
    M_HP_PLUG_IN =		2,
    M_PLUG_OUT =		3,
    M_NO_ACT =		4,
    M_UNKNOWN =		5,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct three_key_threshold {
    pub mid: c_uint,
    pub up: c_uint,
    pub down: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct four_key_threshold {
    pub mid: c_uint,
    pub voice: c_uint,
    pub up: c_uint,
    pub down: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pwm_deb_settings {
    pub pwm_width: c_uint,
    pub pwm_thresh: c_uint,
    pub fall_delay: c_uint,
    pub rise_delay: c_uint,
    pub debounce0: c_uint,
    pub debounce1: c_uint,
    pub debounce3: c_uint,
    pub debounce4: c_uint,
    pub eint_pwm_width: c_uint,
    pub eint_pwm_thresh: c_uint,
    pub eint_debounce0: c_uint,
    pub eint_debounce1: c_uint,
    pub eint_debounce2: c_uint,
    pub eint_debounce3: c_uint,
    pub eint_inverter_debounce: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dts_data {
    pub mic_vol: c_uint,
    pub mic_mode: c_uint,
    pub plugout_deb: c_uint,
    pub eint_pol: c_uint,
    pub pwm_deb: *mut pwm_deb_settings,
    pub three_key: three_key_threshold,
    pub four_key: four_key_threshold,
    pub moisture_detect_enable: c_uint,
    pub eint_detect_mode: c_uint,
    pub eint_use_ext_res: c_uint,
    pub eint_comp_vth: c_uint,
    pub moisture_detect_mode: c_uint,
    pub moisture_comp_vth: c_uint,
    pub moisture_comp_vref2: c_uint,
    pub moisture_use_ext_res: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mt6359_accdet {
    pub jack: *mut snd_soc_jack,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub data: *mut dts_data,
    pub caps: c_uint,
    pub accdet_irq: c_int,
    pub accdet_eint0: c_int,
    pub accdet_eint1: c_int,
    pub /: *mut *mut mutex res_lock; / lock protection,
    pub jack_plugged: bool,
    pub jack_type: c_uint,
    pub btn_type: c_uint,
    pub accdet_status: c_uint,
    pub pre_accdet_status: c_uint,
    pub cali_voltage: c_uint,
    pub jd_sts: c_uint,
    pub accdet_work: work_struct,
    pub accdet_workqueue: *mut workqueue_struct,
    pub jd_work: work_struct,
    pub jd_workqueue: *mut workqueue_struct,
}

