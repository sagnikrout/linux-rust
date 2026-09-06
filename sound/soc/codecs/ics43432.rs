//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/ics43432.c
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
// I2S MEMS microphone driver for InvenSense ICS-43432 and similar
// MEMS-based microphones.
//
// - Non configurable.
// - I2S interface, 64 BCLs per frame, 32 bits per channel, 24 bit data
//
// Copyright (c) 2015 Axis Communications AB
//

    static struct snd_soc_dai_driver ics43432_dai = {
    .name = "ics43432-hifi",
    .capture = {
    .stream_name = "Capture",
    .channels_min = 1,
    .channels_max = 2,
    .rate_min = ICS43432_RATE_MIN,
    .rate_max = ICS43432_RATE_MAX,
    .rates = SNDRV_PCM_RATE_CONTINUOUS,
    .formats = ICS43432_FORMATS,
    },
    };
    static const struct snd_soc_component_driver ics43432_component_driver = {
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
#[no_mangle]
unsafe extern "C" fn ics43432_probe(pdev: *mut platform_device) -> c_int {
    static int ics43432_probe(struct platform_device *pdev)
    {
    return devm_snd_soc_register_component(&pdev.dev,
    &ics43432_component_driver,
    &ics43432_dai, 1);
    }

    static const struct of_device_id ics43432_ids[] = {
    { .compatible = "invensense,ics43432", },
    { .compatible = "cui,cmm-4030d-261", },
    { }
    };
    MODULE_DEVICE_TABLE(of, ics43432_ids);

    static struct platform_driver ics43432_driver = {
    .driver = {
    .name = "ics43432",
    .of_match_table = of_match_ptr(ics43432_ids),
    },
    .probe = ics43432_probe,
    };
    module_platform_driver(ics43432_driver);
    MODULE_DESCRIPTION("ASoC ICS43432 driver");
    MODULE_AUTHOR("Ricard Wanderlof <ricardw@axis.com>");
    MODULE_LICENSE("GPL v2");
