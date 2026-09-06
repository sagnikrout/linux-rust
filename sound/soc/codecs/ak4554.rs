//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ak4554.c
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
// ak4554.c
//
// Copyright (C) 2013 Renesas Solutions Corp.
// Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>

//
// ak4554 is very simple DA/AD converter which has no setting register.
//
// CAUTION
//
// ak4554 playback format is SND_SOC_DAIFMT_RIGHT_J,
// and,   capture  format is SND_SOC_DAIFMT_LEFT_J
// on same bit clock, LR clock.
// But, this driver doesn't have snd_soc_dai_ops :: set_fmt
//
// CPU/Codec DAI image
//
// CPU-DAI1 (plaback only fmt = RIGHT_J) --+-- ak4554
// |
// CPU-DAI2 (capture only fmt = LEFT_J) ---+
//
    static const struct snd_soc_dapm_widget ak4554_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("AINL"),
    SND_SOC_DAPM_INPUT("AINR"),
    SND_SOC_DAPM_OUTPUT("AOUTL"),
    SND_SOC_DAPM_OUTPUT("AOUTR"),
    };
    static const struct snd_soc_dapm_route ak4554_dapm_routes[] = {
    { "Capture", core::ptr::null_mut(), "AINL" },
    { "Capture", core::ptr::null_mut(), "AINR" },
    { "AOUTL", core::ptr::null_mut(), "Playback" },
    { "AOUTR", core::ptr::null_mut(), "Playback" },
    };
    static struct snd_soc_dai_driver ak4554_dai = {
    .name = "ak4554-hifi",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .symmetric_rate = 1,
    };
    static const struct snd_soc_component_driver soc_component_dev_ak4554 = {
    .dapm_widgets		= ak4554_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ak4554_dapm_widgets),
    .dapm_routes		= ak4554_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(ak4554_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn ak4554_soc_probe(pdev: *mut platform_device) -> c_int {
    static int ak4554_soc_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_ak4554,
    &ak4554_dai, 1);
    }
    static const struct of_device_id ak4554_of_match[] = {
    { .compatible = "asahi-kasei,ak4554" },
    {},
    };
    MODULE_DEVICE_TABLE(of, ak4554_of_match);
    static struct platform_driver ak4554_driver = {
    .driver = {
    .name = "ak4554-adc-dac",
    .of_match_table = ak4554_of_match,
    },
    .probe	= ak4554_soc_probe,
    };
    module_platform_driver(ak4554_driver);
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("SoC AK4554 driver");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
