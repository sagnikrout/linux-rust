//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_cs_amp.c
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
// Copyright (c) 2023 Intel Corporation
// Copyright (c) 2024 Advanced Micro Devices, Inc.
//
// soc_sdw_cs_amp - Helpers to handle CS35L56 from generic machine driver
//

pub const CS_AMP_CHANNELS_PER_AMP: c_int = 4;

#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs35l56_volume_limit(card: *mut snd_soc_card, name_prefix: *const c_char) -> c_int {
    int asoc_sdw_cs35l56_volume_limit(struct snd_soc_card *card, const char *name_prefix)
    {
    char *volume_ctl_name;
    int ret;
    volume_ctl_name = kasprintf(GFP_KERNEL, "%s Speaker Volume", name_prefix);
    if (!volume_ctl_name)
    return -ENOMEM;
    ret = snd_soc_limit_volume(card, volume_ctl_name, CS35L56_SPK_VOLUME_0DB);
    if (ret)
    dev_err(card.dev, "%s limit set failed: %d\n", volume_ctl_name, ret);
    kfree(volume_ctl_name);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs35l56_volume_limit, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs_spk_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    char widget_name[16];
    let mut route: snd_soc_dapm_route = { "Speaker", core::ptr::null_mut(), widget_name };
    struct snd_soc_dai *codec_dai;
    int i, ret;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    if (!strstr(codec_dai.name, "cs35l56"))
    continue;
    snprintf(widget_name, sizeof(widget_name), "%s SPK",
    codec_dai.component.name_prefix);
    ret = asoc_sdw_cs35l56_volume_limit(card, codec_dai.component.name_prefix);
    if (ret)
    return ret;
    ret = snd_soc_dapm_add_routes(dapm, &route, 1);
    if (ret)
    return ret;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs_spk_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs_spk_feedback_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs_spk_feedback_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    const struct snd_soc_dai_link *dai_link = rtd.dai_link;
    const struct snd_soc_dai_link_ch_map *ch_map;
    const struct snd_soc_dai_link_component *codec_dlc;
    struct snd_soc_dai *codec_dai;
    u8 ch_slot[8] = {};
    unsigned int amps_per_bus, ch_per_amp, mask;
    int i, ret;
    WARN_ON(dai_link.num_cpus > ARRAY_SIZE(ch_slot));
//
// CS35L56 has 4 TX channels. When the capture is aggregated the
// same bus slots will be allocated to all the amps on a bus. Only
// one amp on that bus can be transmitting in each slot so divide
// the available 4 slots between all the amps on a bus.
//
    amps_per_bus = dai_link.num_codecs / dai_link.num_cpus;
    if ((amps_per_bus == 0) || (amps_per_bus > CS_AMP_CHANNELS_PER_AMP)) {
    dev_err(rtd.card.dev, "Illegal num_codecs:%u / num_cpus:%u\n",
    dai_link.num_codecs, dai_link.num_cpus);
    return -EINVAL;
    }
    ch_per_amp = CS_AMP_CHANNELS_PER_AMP / amps_per_bus;
    for_each_rtd_ch_maps(rtd, i, ch_map) {
    codec_dlc = snd_soc_link_to_codec(rtd.dai_link, i);
    codec_dai = snd_soc_find_dai(codec_dlc);
    mask = GENMASK(ch_per_amp - 1, 0) << ch_slot[ch_map.cpu];
    ret = snd_soc_dai_set_tdm_slot(codec_dai, 0, mask, 4, 32);
    if (ret < 0) {
    dev_err(rtd.card.dev, "Failed to set TDM slot:%d\n", ret);
    return ret;
    }
    ch_slot[ch_map.cpu] += ch_per_amp;
    }
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs_spk_feedback_rtd_init, "SND_SOC_SDW_UTILS");
    int asoc_sdw_cs_amp_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
// Do init on playback link only.
    if (!playback)
    return 0;
    info.amp_num++;
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs_amp_init, "SND_SOC_SDW_UTILS");
