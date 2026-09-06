//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/tegra/tegra_asoc_machine.h
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
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_asoc_data {
    pub srate): *mut *mut unsigned int (mclk_rate)(unsigned int,
    pub codec_dev_name: *const c_char,
    pub hp_jack_name: *const c_char,
    pub card: *mut snd_soc_card,
    pub mclk_id: c_uint,
    pub hp_jack_gpio_active_low: bool,
    pub add_common_dapm_widgets: bool,
    pub add_common_controls: bool,
    pub add_common_snd_ops: bool,
    pub add_headset_jack: bool,
    pub add_mic_jack: bool,
    pub add_hp_jack: bool,
    pub set_ac97: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tegra_machine {
    pub clk_pll_a_out0: *mut clk,
    pub clk_pll_a: *mut clk,
    pub clk_cdev1: *mut clk,
    pub set_baseclock: c_uint,
    pub set_mclk: c_uint,
    pub asoc: *const tegra_asoc_data,
    pub gpiod_ext_mic_en: *mut gpio_desc,
    pub gpiod_int_mic_en: *mut gpio_desc,
    pub gpiod_spkr_en: *mut gpio_desc,
    pub gpiod_mic_det: *mut gpio_desc,
    pub gpiod_ear_sel: *mut gpio_desc,
    pub gpiod_hp_mute: *mut gpio_desc,
    pub gpiod_hp_det: *mut gpio_desc,
    pub mic_jack: *mut snd_soc_jack,
    pub hp_jack_gpio: *mut snd_soc_jack_gpio,
}

extern "C" {
    pub fn tegra_asoc_machine_probe(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn tegra_asoc_machine_init(rtd: *mut snd_soc_pcm_runtime) -> c_int;
}
