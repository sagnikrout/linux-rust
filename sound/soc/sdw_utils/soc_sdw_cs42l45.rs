//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_cs42l45.c
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
// Based on sof_sdw_rt5682.c
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_cs42l45 - Helpers to handle CS42L45 from generic machine driver
//

    static struct snd_soc_jack_pin soc_jack_pins[] = {
    {
    .pin    = "cs42l45 OT 43 Headphone",
    .mask   = SND_JACK_HEADPHONE,
    },
    {
    .pin    = "cs42l45 OT 45 Headset",
    .mask   = SND_JACK_HEADPHONE,
    },
    {
    .pin    = "cs42l45 IT 31 Microphone",
    .mask   = SND_JACK_MICROPHONE,
    },
    {
    .pin    = "cs42l45 IT 33 Headset",
    .mask   = SND_JACK_MICROPHONE,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs42l45_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs42l45_hs_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_component *component = snd_soc_rtd_to_codec(rtd, 0).component;
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct snd_soc_jack *jack = &ctx.sdw_headset;
    int ret;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL, "%s hs:cs42l45",
    card.components);
    if (!card.components)
    return -ENOMEM;
    ret = snd_soc_card_jack_new_pins(card, "Jack", SND_JACK_MECHANICAL |
    SND_JACK_HEADSET | SND_JACK_LINEOUT, jack,
    soc_jack_pins, ARRAY_SIZE(soc_jack_pins));
    if (ret) {
    dev_err(card.dev, "Failed to create jack: %d\n", ret);
    return ret;
    }
    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if (ret) {
    dev_err(card.dev, "Failed to register jack: %d\n", ret);
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l45_hs_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs42l45_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs42l45_dmic_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL, "%s mic:cs42l45-dmic",
    card.components);
    if (!card.components)
    return -ENOMEM;
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l45_dmic_rtd_init, "SND_SOC_SDW_UTILS");
