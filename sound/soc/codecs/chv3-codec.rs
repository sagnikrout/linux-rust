//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/chv3-codec.c
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

    static struct snd_soc_dai_driver chv3_codec_dai = {
    .name = "chv3-codec-hifi",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 8,
    .channels_max = 8,
    .rates = SNDRV_PCM_RATE_CONTINUOUS,
    .formats = SNDRV_PCM_FMTBIT_S32_LE,
    },
    };
    static const struct snd_soc_component_driver soc_component_dev_chv3_codec = {
    };
#[no_mangle]
unsafe extern "C" fn chv3_codec_probe(pdev: *mut platform_device) -> c_int {
    static int chv3_codec_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &soc_component_dev_chv3_codec, &chv3_codec_dai, 1);
    }
    static const struct of_device_id chv3_codec_of_match[] = {
    { .compatible = "google,chv3-codec", },
    { }
    };
    MODULE_DEVICE_TABLE(of, chv3_codec_of_match);
    static struct platform_driver chv3_codec_platform_driver = {
    .driver = {
    .name = "chv3-codec",
    .of_match_table = chv3_codec_of_match,
    },
    .probe = chv3_codec_probe,
    };
    module_platform_driver(chv3_codec_platform_driver);
    MODULE_DESCRIPTION("ASoC Chameleon v3 codec driver");
    MODULE_AUTHOR("Pawel Anikiel <pan@semihalf.com>");
    MODULE_LICENSE("GPL");
