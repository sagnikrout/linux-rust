//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l42.h
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
// cs42l42.h -- CS42L42 ALSA SoC audio driver header
//
// Copyright 2016-2022 Cirrus Logic, Inc.
//
// Author: James Schulman <james.schulman@cirrus.com>
// Author: Brian Austin <brian.austin@cirrus.com>
// Author: Michael White <michael.white@cirrus.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs42l42_private {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub supplies: [regulator_bulk_data; CS42L42_NUM_SUPPLIES],
    pub reset_gpio: *mut gpio_desc,
    pub pdn_done: completion,
    pub jack: *mut snd_soc_jack,
    pub sdw_peripheral: *mut sdw_slave,
    pub irq_lock: mutex,
    pub devid: c_int,
    pub irq: c_int,
    pub pll_config: c_int,
    pub sclk: u32,
    pub sample_rate: u32,
    pub bclk_ratio: u32,
    pub plug_state: u8,
    pub hs_type: u8,
    pub ts_inv: u8,
    pub ts_dbnc_rise: u8,
    pub ts_dbnc_fall: u8,
    pub btn_det_init_dbnce: u8,
    pub btn_det_event_dbnce: u8,
    pub bias_thresholds: [u8; CS42L42_NUM_BIASES],
    pub hs_bias_ramp_rate: u8,
    pub hs_bias_ramp_time: u8,
    pub hs_bias_sense_en: u8,
    pub stream_use: u8,
    pub hp_adc_up_pending: bool,
    pub suspended: bool,
    pub sdw_waiting_first_unattach: bool,
    pub init_done: bool,
}

extern "C" {
    pub fn cs42l42_readable_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn cs42l42_volatile_register(dev: *mut device, reg: c_uint) -> bool;
}
extern "C" {
    pub fn cs42l42_src_config(component: *mut snd_soc_component, sample_rate: c_uint);
}
extern "C" {
    pub fn cs42l42_mute_stream(dai: *mut snd_soc_dai, mute: c_int, stream: c_int) -> c_int;
}
extern "C" {
    pub fn cs42l42_irq_thread(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cs42l42_suspend(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs42l42_resume(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn cs42l42_resume_restore(dev: *mut device);
}
extern "C" {
    pub fn cs42l42_init(cs42l42: *mut cs42l42_private) -> c_int;
}
extern "C" {
    pub fn cs42l42_common_remove(cs42l42: *mut cs42l42_private);
}
