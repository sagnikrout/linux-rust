//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ads117x.c
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
// ads117x.c  --  Driver for ads1174/8 ADC chips
//
// Copyright 2009 ShotSpotter Inc.
// Author: Graeme Gregory <gg@slimlogic.co.uk>
//

    static const struct snd_soc_dapm_widget ads117x_dapm_widgets[] = {
    SND_SOC_DAPM_INPUT("Input1"),
    SND_SOC_DAPM_INPUT("Input2"),
    SND_SOC_DAPM_INPUT("Input3"),
    SND_SOC_DAPM_INPUT("Input4"),
    SND_SOC_DAPM_INPUT("Input5"),
    SND_SOC_DAPM_INPUT("Input6"),
    SND_SOC_DAPM_INPUT("Input7"),
    SND_SOC_DAPM_INPUT("Input8"),
    };
    static const struct snd_soc_dapm_route ads117x_dapm_routes[] = {
    { "Capture", core::ptr::null_mut(), "Input1" },
    { "Capture", core::ptr::null_mut(), "Input2" },
    { "Capture", core::ptr::null_mut(), "Input3" },
    { "Capture", core::ptr::null_mut(), "Input4" },
    { "Capture", core::ptr::null_mut(), "Input5" },
    { "Capture", core::ptr::null_mut(), "Input6" },
    { "Capture", core::ptr::null_mut(), "Input7" },
    { "Capture", core::ptr::null_mut(), "Input8" },
    };
    static struct snd_soc_dai_driver ads117x_dai = {
// ADC
    .name = "ads117x-hifi",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 32,
    .rates = ADS117X_RATES,
    .formats = ADS117X_FORMATS,},
    };
    static const struct snd_soc_component_driver soc_component_dev_ads117x = {
    .dapm_widgets		= ads117x_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(ads117x_dapm_widgets),
    .dapm_routes		= ads117x_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(ads117x_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn ads117x_probe(pdev: *mut platform_device) -> c_int {
    static int ads117x_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_ads117x, &ads117x_dai, 1);
    }

    static const struct of_device_id ads117x_dt_ids[] = {
    { .compatible = "ti,ads1174" },
    { .compatible = "ti,ads1178" },
    { },
    };
    MODULE_DEVICE_TABLE(of, ads117x_dt_ids);

    static struct platform_driver ads117x_codec_driver = {
    .driver = {
    .name = "ads117x-codec",
    .of_match_table = of_match_ptr(ads117x_dt_ids),
    },
    .probe = ads117x_probe,
    };
    module_platform_driver(ads117x_codec_driver);
    MODULE_DESCRIPTION("ASoC ads117x driver");
    MODULE_AUTHOR("Graeme Gregory");
    MODULE_LICENSE("GPL");
