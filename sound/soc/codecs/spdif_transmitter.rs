//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/spdif_transmitter.c
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
// ALSA SoC SPDIF DIT driver
//
// This driver is used by controllers which can operate in DIT (SPDI/F) where
// no codec is needed.  This file provides stub codec that can be used
// in these configurations. TI DaVinci Audio controller uses this driver.
//
// Author:      Steve Chen,  <schen@mvista.com>
// Copyright:   (C) 2009 MontaVista Software, Inc., <source@mvista.com>
// Copyright:   (C) 2009  Texas Instruments, India
//

    SNDRV_PCM_RATE_128000)

    SNDRV_PCM_FMTBIT_S20_3LE | \
    SNDRV_PCM_FMTBIT_S24_LE  | \
    SNDRV_PCM_FMTBIT_S32_LE)
    static const struct snd_soc_dapm_widget dit_widgets[] = {
    SND_SOC_DAPM_OUTPUT("spdif-out"),
    };
    static const struct snd_soc_dapm_route dit_routes[] = {
    { "spdif-out", core::ptr::null_mut(), "Playback" },
    };
    static const struct snd_soc_component_driver soc_codec_spdif_dit = {
    .dapm_widgets		= dit_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(dit_widgets),
    .dapm_routes		= dit_routes,
    .num_dapm_routes	= ARRAY_SIZE(dit_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static struct snd_soc_dai_driver dit_stub_dai = {
    .name		= "dit-hifi",
    .playback 	= {
    .stream_name	= "Playback",
    .channels_min	= 1,
    .channels_max	= 384,
    .rates		= STUB_RATES,
    .formats	= STUB_FORMATS,
    },
    };
#[no_mangle]
unsafe extern "C" fn spdif_dit_probe(pdev: *mut platform_device) -> c_int {
    static int spdif_dit_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_codec_spdif_dit,
    &dit_stub_dai, 1);
    }

    static const struct of_device_id spdif_dit_dt_ids[] = {
    { .compatible = "linux,spdif-dit", },
    { }
    };
    MODULE_DEVICE_TABLE(of, spdif_dit_dt_ids);

    static struct platform_driver spdif_dit_driver = {
    .probe		= spdif_dit_probe,
    .driver		= {
    .name	= DRV_NAME,
    .of_match_table = of_match_ptr(spdif_dit_dt_ids),
    },
    };
    module_platform_driver(spdif_dit_driver);
    MODULE_AUTHOR("Steve Chen <schen@mvista.com>");
    MODULE_DESCRIPTION("SPDIF dummy codec driver");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:" DRV_NAME);
