//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/codecs/arizona.h
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
// arizona.h - Wolfson Arizona class device shared support
//
// Copyright 2012 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
//

pub const ARIZONA_CLK_SYSCLK: c_int = 1;
pub const ARIZONA_CLK_ASYNCCLK: c_int = 2;
pub const ARIZONA_CLK_OPCLK: c_int = 3;
pub const ARIZONA_CLK_ASYNC_OPCLK: c_int = 4;
pub const ARIZONA_CLK_SRC_MCLK1: c_uint = 0x0;
pub const ARIZONA_CLK_SRC_MCLK2: c_uint = 0x1;
pub const ARIZONA_CLK_SRC_FLL1: c_uint = 0x4;
pub const ARIZONA_CLK_SRC_FLL2: c_uint = 0x5;
pub const ARIZONA_CLK_SRC_AIF1BCLK: c_uint = 0x8;
pub const ARIZONA_CLK_SRC_AIF2BCLK: c_uint = 0x9;
pub const ARIZONA_CLK_SRC_AIF3BCLK: c_uint = 0xa;

pub const ARIZONA_FLL_SRC_MCLK1: c_int = 0;
pub const ARIZONA_FLL_SRC_MCLK2: c_int = 1;
pub const ARIZONA_FLL_SRC_SLIMCLK: c_int = 3;
pub const ARIZONA_FLL_SRC_FLL1: c_int = 4;
pub const ARIZONA_FLL_SRC_FLL2: c_int = 5;
pub const ARIZONA_FLL_SRC_AIF1BCLK: c_int = 8;
pub const ARIZONA_FLL_SRC_AIF2BCLK: c_int = 9;
pub const ARIZONA_FLL_SRC_AIF3BCLK: c_int = 10;
pub const ARIZONA_FLL_SRC_AIF1LRCLK: c_int = 12;
pub const ARIZONA_FLL_SRC_AIF2LRCLK: c_int = 13;
pub const ARIZONA_FLL_SRC_AIF3LRCLK: c_int = 14;
pub const ARIZONA_MIXER_VOL_MASK: c_uint = 0x00FE;
pub const ARIZONA_MIXER_VOL_SHIFT: c_int = 1;
pub const ARIZONA_MIXER_VOL_WIDTH: c_int = 7;
pub const ARIZONA_CLK_6MHZ: c_int = 0;
pub const ARIZONA_CLK_12MHZ: c_int = 1;
pub const ARIZONA_CLK_24MHZ: c_int = 2;
pub const ARIZONA_CLK_49MHZ: c_int = 3;
pub const ARIZONA_CLK_73MHZ: c_int = 4;
pub const ARIZONA_CLK_98MHZ: c_int = 5;
pub const ARIZONA_CLK_147MHZ: c_int = 6;
pub const ARIZONA_MAX_DAI: c_int = 10;
pub const ARIZONA_MAX_ADSP: c_int = 4;
pub const ARIZONA_DVFS_SR1_RQ: c_uint = 0x001;
pub const ARIZONA_DVFS_ADSP1_RQ: c_uint = 0x100;
// Notifier events
pub const ARIZONA_NOTIFY_VOICE_TRIGGER: c_uint = 0x1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_dai_priv {
    pub clk: c_int,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_priv {
    pub adsp: [wm_adsp; ARIZONA_MAX_ADSP],
    pub arizona: *mut arizona,
    pub sysclk: c_int,
    pub asyncclk: c_int,
    pub dai: [arizona_dai_priv; ARIZONA_MAX_DAI],
    pub num_inputs: c_int,
    pub in_pending: c_uint,
    pub out_up_pending: c_uint,
    pub out_up_delay: c_uint,
    pub out_down_pending: c_uint,
    pub out_down_delay: c_uint,
    pub dvfs_reqs: c_uint,
    pub dvfs_lock: mutex,
    pub dvfs_cached: bool,
// Variables used by arizona-jack.c code
    pub lock: mutex,
    pub hpdet_work: delayed_work,
    pub micd_detect_work: delayed_work,
    pub micd_timeout_work: delayed_work,
    pub jack: *mut snd_soc_jack,
    pub micvdd: *mut regulator,
    pub micd_pol_gpio: *mut gpio_desc,
    pub hpdet_id_gpio: *mut gpio_desc,
    pub last_jackdet: u16,
    pub micd_mode: c_int,
    pub micd_modes: *const arizona_micd_config,
    pub micd_num_modes: c_int,
    pub micd_button_mask: c_int,
    pub micd_ranges: *const arizona_micd_range,
    pub num_micd_ranges: c_int,
    pub micd_reva: bool,
    pub micd_clamp: bool,
    pub hpdet_active: bool,
    pub hpdet_done: bool,
    pub hpdet_retried: bool,
    pub mic: bool,
    pub detecting: bool,
    pub num_hpdet_res: c_int,
    pub hpdet_res: [c_uint; 3],
    pub jack_flips: c_int,
    pub hpdet_ip_version: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_voice_trigger_info {
    pub core: c_int,
}

pub const ARIZONA_NUM_MIXER_INPUTS: c_int = 104;

pub const ARIZONA_RATE_ENUM_SIZE: c_int = 4;
pub const ARIZONA_SAMPLE_RATE_ENUM_SIZE: c_int = 14;
// SND_JACK_* mask for supported cable/switch types

pub const ARIZONA_FLL_NAME_LEN: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct arizona_fll {
    pub arizona: *mut arizona,
    pub id: c_int,
    pub base: c_uint,
    pub vco_mult: c_uint,
    pub fout: c_uint,
    pub sync_src: c_int,
    pub sync_freq: c_uint,
    pub ref_src: c_int,
    pub ref_freq: c_uint,
    pub lock_name: [c_char; ARIZONA_FLL_NAME_LEN],
    pub clock_ok_name: [c_char; ARIZONA_FLL_NAME_LEN],
}

extern "C" {
    pub fn arizona_dvfs_up(component: *mut snd_soc_component, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn arizona_dvfs_down(component: *mut snd_soc_component, flags: c_uint) -> c_int;
}
extern "C" {
    pub fn arizona_init_dvfs(priv: *mut arizona_priv);
}
extern "C" {
    pub fn arizona_init_spk(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn arizona_init_gpio(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn arizona_init_mono(component: *mut snd_soc_component) -> c_int;
}
extern "C" {
    pub fn arizona_init_common(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_init_vol_limit(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_init_spk_irqs(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_free_spk_irqs(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_init_dai(priv: *mut arizona_priv, id: c_int) -> c_int;
}
extern "C" {
    pub fn arizona_input_analog(component: *mut snd_soc_component, shift: c_int) -> bool;
}
extern "C" {
    pub fn blocking_notifier_chain_register(_arg: &arizona->notifier, _arg: nb) -> return;
}
extern "C" {
    pub fn blocking_notifier_chain_unregister(_arg: &arizona->notifier, _arg: nb) -> return;
}
extern "C" {
    pub fn arizona_of_get_audio_pdata(arizona: *mut arizona) -> c_int;
}
extern "C" {
    pub fn arizona_jack_codec_dev_probe(info: *mut arizona_priv, dev: *mut device) -> c_int;
}
extern "C" {
    pub fn arizona_jack_codec_dev_remove(info: *mut arizona_priv) -> c_int;
}
