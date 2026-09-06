//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/cs42l43.h
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
// CS42L43 CODEC driver internal data
//
// Copyright (C) 2022-2023 Cirrus Logic, Inc. and
// Cirrus Logic International Semiconductor Ltd.
//

pub const CS42L43_INTERNAL_SYSCLK: c_int = 24576000;
pub const CS42L43_DEFAULT_SLOTS: c_uint = 0x3F;
pub const CS42L43_PLL_TIMEOUT_MS: c_int = 200;
pub const CS42L43_SPK_TIMEOUT_MS: c_int = 100;
pub const CS42L43_HP_TIMEOUT_MS: c_int = 2000;
pub const CS42L43_LOAD_TIMEOUT_MS: c_int = 1000;
pub const CS42L43_HP_ILIMIT_BACKOFF_MS: c_int = 1000;
pub const CS42L43_HP_ILIMIT_DECAY_MS: c_int = 300;
pub const CS42L43_HP_ILIMIT_MAX_COUNT: c_int = 4;
pub const CS42L43_ASP_MAX_CHANNELS: c_int = 6;
pub const CS42L43_N_EQ_COEFFS: c_int = 15;
pub const CS42L43_N_BUTTONS: c_int = 6;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cs42l43_codec {
    pub dev: *mut device,
    pub core: *mut cs42l43,
    pub component: *mut snd_soc_component,
    pub dom: *mut irq_domain,
    pub shutter_irqs: [c_uint; 4],
    pub mclk: *mut clk,
    pub n_slots: c_int,
    pub slot_width: c_int,
    pub tx_slots: [c_int; CS42L43_ASP_MAX_CHANNELS],
    pub rx_slots: [c_int; CS42L43_ASP_MAX_CHANNELS],
    pub constraint: snd_pcm_hw_constraint_list,
    pub eq_coeffs: [u32; CS42L43_N_EQ_COEFFS],
    pub refclk_src: c_uint,
    pub refclk_freq: c_uint,
    pub pll_ready: completion,
    pub decim_cache: [c_uint; 6],
    pub adc_ena: c_uint,
    pub hp_ena: c_uint,
    pub hp_startup: completion,
    pub hp_shutdown: completion,
    pub spkr_shutdown: completion,
    pub spkl_shutdown: completion,
    pub spkr_startup: completion,
    pub spkl_startup: completion,
// Lock to ensure speaker VU updates don't clash
    pub spk_vu_lock: mutex,
// Lock for all jack detect operations
    pub jack_lock: mutex,
    pub jack_hp: *mut snd_soc_jack,
    pub use_ring_sense: bool,
    pub tip_debounce_ms: c_uint,
    pub tip_fall_db_ms: c_uint,
    pub tip_rise_db_ms: c_uint,
    pub bias_low: c_uint,
    pub bias_sense_ua: c_uint,
    pub bias_ramp_ms: c_uint,
    pub detect_us: c_uint,
    pub buttons: [c_uint; CS42L43_N_BUTTONS],
    pub tip_sense_work: delayed_work,
    pub bias_sense_timeout: delayed_work,
    pub type_detect: completion,
    pub load_detect: completion,
    pub load_detect_running: bool,
    pub button_detect_running: bool,
    pub jack_present: bool,
    pub jack_override: c_int,
    pub suspend_jack_debounce: bool,
    pub hp_ilimit_clear_work: delayed_work,
    pub hp_ilimited: bool,
    pub hp_ilimit_count: c_int,
    pub kctl: [*mut snd_kcontrol; 7],
}

extern "C" {
    pub fn cs42l43_sdw_set_stream(dai: *mut snd_soc_dai, sdw_stream: *mut c_void, direction: c_int) -> c_int;
}

extern "C" {
    pub fn cs42l43_bias_sense_timeout(work: *mut work_struct);
}
extern "C" {
    pub fn cs42l43_clear_jack(priv: *mut cs42l43_codec);
}
extern "C" {
    pub fn cs42l43_tip_sense_work(work: *mut work_struct);
}
extern "C" {
    pub fn cs42l43_bias_detect_clamp(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cs42l43_button_press(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cs42l43_button_release(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cs42l43_tip_sense(irq: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cs42l43_jack_get(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
extern "C" {
    pub fn cs42l43_jack_put(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
}
