//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/gtm601.c
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
// This is a simple driver for the GTM601 Voice PCM interface
//
// Copyright (C) 2015 Goldelico GmbH
//
// Author: Marek Belisko <marek@goldelico.com>
//
// Based on wm8727.c driver
//

    static const struct snd_soc_dapm_widget gtm601_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("AOUT"),
    SND_SOC_DAPM_INPUT("AIN"),
    };
    static const struct snd_soc_dapm_route gtm601_dapm_routes[] = {
    { "AOUT", core::ptr::null_mut(), "Playback" },
    { "Capture", core::ptr::null_mut(), "AIN" },
    };
    static struct snd_soc_dai_driver gtm601_dai = {
    .name = "gtm601",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 1,
    .rates = SNDRV_PCM_RATE_8000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    };
    static struct snd_soc_dai_driver bm818_dai = {
    .name = "bm818",
    .playback = {
    .stream_name = "Playback",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    .capture = {
    .stream_name = "Capture",
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_48000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE,
    },
    };
    static const struct snd_soc_component_driver soc_component_dev_gtm601 = {
    .dapm_widgets		= gtm601_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(gtm601_dapm_widgets),
    .dapm_routes		= gtm601_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(gtm601_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn gtm601_platform_probe(pdev: *mut platform_device) -> c_int {
    static int gtm601_platform_probe(struct platform_device *pdev)
    {
    const struct snd_soc_dai_driver *dai_driver;
    dai_driver = of_device_get_match_data(&pdev.dev);
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_gtm601,
    (struct snd_soc_dai_driver *)dai_driver, 1);
    }
    static const struct of_device_id gtm601_codec_of_match[] __maybe_unused = {
    { .compatible = "option,gtm601", .data = (void *)&gtm601_dai },
    { .compatible = "broadmobi,bm818", .data = (void *)&bm818_dai },
    {},
    };
    MODULE_DEVICE_TABLE(of, gtm601_codec_of_match);
    static struct platform_driver gtm601_codec_driver = {
    .driver = {
    .name = "gtm601",
    .of_match_table = of_match_ptr(gtm601_codec_of_match),
    },
    .probe = gtm601_platform_probe,
    };
    module_platform_driver(gtm601_codec_driver);
    MODULE_DESCRIPTION("ASoC gtm601 driver");
    MODULE_AUTHOR("Marek Belisko <marek@goldelico.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:gtm601");
