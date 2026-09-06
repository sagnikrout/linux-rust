//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/wm8727.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// wm8727.c
//
// Created on: 15-Oct-2009
// Author: neil.jones@imgtec.com
//
// Copyright (C) 2009 Imagination Technologies Ltd.
//

    static const struct snd_soc_dapm_widget wm8727_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("VOUTL"),
    SND_SOC_DAPM_OUTPUT("VOUTR"),
    };
    static const struct snd_soc_dapm_route wm8727_dapm_routes[] = {
    { "VOUTL", core::ptr::null_mut(), "Playback" },
    { "VOUTR", core::ptr::null_mut(), "Playback" },
    };
//
// Note this is a simple chip with no configuration interface, sample rate is
// determined automatically by examining the Master clock and Bit clock ratios
//

    SNDRV_PCM_RATE_48000 | SNDRV_PCM_RATE_96000 |\
    SNDRV_PCM_RATE_192000)
    static struct snd_soc_dai_driver wm8727_dai = {
    .name = "wm8727-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = WM8727_RATES,
    .formats = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE,
    },
    };
    static const struct snd_soc_component_driver soc_component_dev_wm8727 = {
    .dapm_widgets		= wm8727_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(wm8727_dapm_widgets),
    .dapm_routes		= wm8727_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(wm8727_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn wm8727_probe(pdev: *mut platform_device) -> c_int {
    static int wm8727_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_wm8727, &wm8727_dai, 1);
    }
    static struct platform_driver wm8727_codec_driver = {
    .driver = {
    .name = "wm8727",
    },
    .probe = wm8727_probe,
    };
    module_platform_driver(wm8727_codec_driver);
    MODULE_DESCRIPTION("ASoC wm8727 driver");
    MODULE_AUTHOR("Neil Jones");
    MODULE_LICENSE("GPL");
