//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_maxim.c
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
// soc_sdw_maxim - Helpers to handle maxim codecs
// codec devices from generic machine driver

    static int maxim_part_id;
pub const SOC_SDW_PART_ID_MAX98363: c_uint = 0x8363;
pub const SOC_SDW_PART_ID_MAX98373: c_uint = 0x8373;
    static const struct snd_soc_dapm_route max_98373_dapm_routes[] = {
    { "Left Spk", core::ptr::null_mut(), "Left BE_OUT" },
    { "Right Spk", core::ptr::null_mut(), "Right BE_OUT" },
    };
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_maxim_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_maxim_spk_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret;
    ret = snd_soc_dapm_add_routes(dapm, max_98373_dapm_routes, 2);
    if (ret)
    dev_err(rtd.dev, "failed to add first SPK map: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_maxim_spk_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
unsafe extern "C" fn asoc_sdw_mx8373_enable_spk_pin(substream: *mut snd_pcm_substream, enable: bool) -> c_int {
    static int asoc_sdw_mx8373_enable_spk_pin(struct snd_pcm_substream *substream, bool enable)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai;
    struct snd_soc_dai *cpu_dai;
    int ret;
    int j;
// set spk pin by playback only
    if (substream.stream == SNDRV_PCM_STREAM_CAPTURE)
    return 0;
    cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    for_each_rtd_codec_dais(rtd, j, codec_dai) {
    struct snd_soc_dapm_context *dapm = snd_soc_component_to_dapm(cpu_dai.component);
    char pin_name[16];
    snprintf(pin_name, ARRAY_SIZE(pin_name), "%s Spk",
    codec_dai.component.name_prefix);
    if (enable)
    ret = snd_soc_dapm_enable_pin(dapm, pin_name);
    else
    ret = snd_soc_dapm_disable_pin(dapm, pin_name);
    if (!ret)
    snd_soc_dapm_sync(dapm);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn asoc_sdw_mx8373_prepare(substream: *mut snd_pcm_substream) -> c_int {
    static int asoc_sdw_mx8373_prepare(struct snd_pcm_substream *substream)
    {
    int ret;
// according to soc_pcm_prepare dai link prepare is called first
    ret = asoc_sdw_prepare(substream);
    if (ret < 0)
    return ret;
    return asoc_sdw_mx8373_enable_spk_pin(substream, true);
    }
#[no_mangle]
unsafe extern "C" fn asoc_sdw_mx8373_hw_free(substream: *mut snd_pcm_substream) -> c_int {
    static int asoc_sdw_mx8373_hw_free(struct snd_pcm_substream *substream)
    {
    int ret;
// according to soc_pcm_hw_free dai link free is called first
    ret = asoc_sdw_hw_free(substream);
    if (ret < 0)
    return ret;
    return asoc_sdw_mx8373_enable_spk_pin(substream, false);
    }
    static const struct snd_soc_ops max_98373_sdw_ops = {
    .startup = asoc_sdw_startup,
    .prepare = asoc_sdw_mx8373_prepare,
    .trigger = asoc_sdw_trigger,
    .hw_params = asoc_sdw_hw_params,
    .hw_free = asoc_sdw_mx8373_hw_free,
    .shutdown = asoc_sdw_shutdown,
    };
#[no_mangle]
unsafe extern "C" fn asoc_sdw_mx8373_sdw_late_probe(card: *mut snd_soc_card) -> c_int {
    static int asoc_sdw_mx8373_sdw_late_probe(struct snd_soc_card *card)
    {
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
// Disable Left and Right Spk pin after boot
    snd_soc_dapm_disable_pin(dapm, "Left Spk");
    snd_soc_dapm_disable_pin(dapm, "Right Spk");
    return snd_soc_dapm_sync(dapm);
    }
    int asoc_sdw_maxim_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
    info.amp_num++;
    maxim_part_id = info.part_id;
    switch (maxim_part_id) {
    case SOC_SDW_PART_ID_MAX98363:
// Default ops are set in function init_dai_link.
// called as part of function create_sdw_dailink
//
    break;
    case SOC_SDW_PART_ID_MAX98373:
    info.codec_card_late_probe = asoc_sdw_mx8373_sdw_late_probe;
    dai_links.ops = &max_98373_sdw_ops;
    break;
    default:
    dev_err(card.dev, "Invalid maxim_part_id %#x\n", maxim_part_id);
    return -EINVAL;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_maxim_init, "SND_SOC_SDW_UTILS");
