//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm_hubs.h
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
// wm_hubs.h  --  WM899x common code
//
// Copyright 2009 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

// This *must* be the first element of the codec->private_data struct
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_hubs_data {
    pub dcs_codes_l: c_int,
    pub dcs_codes_r: c_int,
    pub dcs_readback_mode: c_int,
    pub hp_startup_mode: c_int,
    pub series_startup: c_int,
    pub no_series_update: c_int,
    pub micd_scthr: bool,
    pub no_cache_dac_hp_direct: bool,
    pub dcs_cache: list_head,
    pub ): *mut *mut bool (check_class_w_digital)(struct snd_soc_component,
    pub micb1_delay: c_int,
    pub micb2_delay: c_int,
    pub lineout1_se: bool,
    pub lineout1n_ena: bool,
    pub lineout1p_ena: bool,
    pub lineout2_se: bool,
    pub lineout2n_ena: bool,
    pub lineout2p_ena: bool,
    pub dcs_done_irq: bool,
    pub dcs_done: completion,
    pub component: *mut snd_soc_component,
}

extern "C" {
    pub fn wm_hubs_add_analogue_controls(: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn wm_hubs_add_analogue_routes(: *mut snd_soc_component, _arg: c_int, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn wm_hubs_dcs_done(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn wm_hubs_vmid_ena(component: *mut snd_soc_component);
}
extern "C" {
    pub fn wm_hubs_update_class_w(component: *mut snd_soc_component);
}
