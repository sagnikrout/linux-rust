//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_dmic.c
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
// This file incorporates work covered by the following copyright notice:
// Copyright (c) 2020 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_dmic - Helpers to handle dmic from generic machine driver
//

    static const struct snd_soc_dapm_widget dmic_widgets[] = {
    SND_SOC_DAPM_MIC("SoC DMIC", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route dmic_map[] = {
// digital mics
    {"DMic", core::ptr::null_mut(), "SoC DMIC"},
    };
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_dmic_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    int asoc_sdw_dmic_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret;
    ret = snd_soc_dapm_new_controls(dapm, dmic_widgets,
    ARRAY_SIZE(dmic_widgets));
    if (ret) {
    dev_err(card.dev, "DMic widget addition failed: %d\n", ret);
// Don't need to add routes if widget addition failed
    return ret;
    }
    ret = snd_soc_dapm_add_routes(dapm, dmic_map,
    ARRAY_SIZE(dmic_map));
    if (ret)
    dev_err(card.dev, "DMic map addition failed: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_dmic_init, "SND_SOC_SDW_UTILS");
