//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/pcm5102a.c
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
// Driver for the PCM5102A codec
//
// Author:	Florian Meier <florian.meier@koalo.de>
// Copyright 2013
//

    static struct snd_soc_dai_driver pcm5102a_dai = {
    .name = "pcm5102a-hifi",
    .playback = {
    .channels_min = 2,
    .channels_max = 2,
    .rates = SNDRV_PCM_RATE_8000_384000,
    .formats = SNDRV_PCM_FMTBIT_S16_LE |
    SNDRV_PCM_FMTBIT_S24_LE |
    SNDRV_PCM_FMTBIT_S32_LE
    },
    };
    static const struct snd_soc_component_driver soc_component_dev_pcm5102a = {
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn pcm5102a_probe(pdev: *mut platform_device) -> c_int {
    static int pcm5102a_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev, &soc_component_dev_pcm5102a,
    &pcm5102a_dai, 1);
    }
    static const struct of_device_id pcm5102a_of_match[] = {
    { .compatible = "ti,pcm5102a", },
    { }
    };
    MODULE_DEVICE_TABLE(of, pcm5102a_of_match);
    static struct platform_driver pcm5102a_codec_driver = {
    .probe		= pcm5102a_probe,
    .driver		= {
    .name	= "pcm5102a-codec",
    .of_match_table = pcm5102a_of_match,
    },
    };
    module_platform_driver(pcm5102a_codec_driver);
    MODULE_DESCRIPTION("ASoC PCM5102A codec driver");
    MODULE_AUTHOR("Florian Meier <florian.meier@koalo.de>");
    MODULE_LICENSE("GPL v2");
