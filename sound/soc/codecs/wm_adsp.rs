//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/wm_adsp.h
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
// wm_adsp.h  --  Wolfson ADSP support
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

// Return values for wm_adsp_compr_handle_irq
pub const WM_ADSP_COMPR_OK: c_int = 0;
pub const WM_ADSP_COMPR_VOICE_TRIGGER: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_adsp {
    pub cs_dsp: cs_dsp,
    pub part: *const c_char,
    pub fwf_name: *const c_char,
    pub system_name: *const c_char,
    pub fwf_suffix: *const c_char,
    pub component: *mut snd_soc_component,
    pub sys_config_size: c_uint,
    pub fw: c_int,
    pub wmfw_optional: bool,
    pub bin_mandatory: bool,
    pub boot_work: work_struct,
    pub cs_ctl): *mut *mut *mut int (control_add)(struct wm_adsp dsp, struct cs_dsp_coeff_ctl,
    pub dsp): *mut *mut int (pre_run)(struct wm_adsp,
    pub preloaded: bool,
    pub fatal_error: bool,
    pub compr_list: list_head,
    pub buffer_list: list_head,
//
// Flag indicating the preloader widget only needs power toggled
// on state change rather than held on for the duration of the
// preload, useful for devices that can retain firmware memory
// across power down.
//
    pub toggle_preload: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_adsp_fw_file {
    pub firmware: *const firmware,
    pub filename: *mut c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wm_adsp_fw_files {
    pub wmfw: wm_adsp_fw_file,
    pub coeff: wm_adsp_fw_file,
}

extern "C" {
    pub fn wm_adsp1_init(dsp: *mut wm_adsp) -> c_int;
}
extern "C" {
    pub fn wm_adsp2_init(dsp: *mut wm_adsp) -> c_int;
}
extern "C" {
    pub fn wm_adsp2_remove(dsp: *mut wm_adsp);
}
extern "C" {
    pub fn wm_adsp2_component_probe(dsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn wm_adsp2_component_remove(dsp: *mut wm_adsp, component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn wm_halo_init(dsp: *mut wm_adsp) -> c_int;
}
extern "C" {
    pub fn wm_adsp_power_up(dsp: *mut wm_adsp, load_firmware: bool) -> c_int;
}
extern "C" {
    pub fn wm_adsp_power_down(dsp: *mut wm_adsp);
}
extern "C" {
    pub fn wm_adsp2_bus_error(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn wm_halo_bus_error(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn wm_halo_wdt_expire(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn wm_adsp_run(dsp: *mut wm_adsp) -> c_int;
}
extern "C" {
    pub fn wm_adsp_stop(dsp: *mut wm_adsp);
}
extern "C" {
    pub fn wm_adsp_hibernate(dsp: *mut wm_adsp, hibernate: bool);
}
extern "C" {
    pub fn wm_adsp2_set_dspclk(w: *mut snd_soc_dapm_widget, freq: c_uint) -> c_int;
}
extern "C" {
    pub fn wm_adsp_compr_open(dsp: *mut wm_adsp, stream: *mut snd_compr_stream) -> c_int;
}
extern "C" {
    pub fn wm_adsp_compr_handle_irq(dsp: *mut wm_adsp) -> c_int;
}
extern "C" {
    pub fn wm_adsp_control_add(cs_ctl: *mut cs_dsp_coeff_ctl) -> c_int;
}

extern "C" {
    pub fn wm_adsp_release_firmware_files(fw: *mut wm_adsp_fw_files);
}
extern "C" {
    pub fn wm_adsp_request_firmware_files(dsp: *mut wm_adsp, fw: *mut wm_adsp_fw_files) -> c_int;
}

