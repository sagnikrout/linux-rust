//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_cs42l43.c
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
// soc_sdw_cs42l43 - Helpers to handle CS42L43 from generic machine driver
//

    static const struct snd_soc_dapm_route cs42l43_hs_map[] = {
    { "Headphone", core::ptr::null_mut(), "cs42l43 AMP3_OUT" },
    { "Headphone", core::ptr::null_mut(), "cs42l43 AMP4_OUT" },
    { "cs42l43 ADC1_IN1_P", core::ptr::null_mut(), "Headset Mic" },
    { "cs42l43 ADC1_IN1_N", core::ptr::null_mut(), "Headset Mic" },
    };
    static const struct snd_soc_dapm_route cs42l43_spk_map[] = {
    { "Speaker", core::ptr::null_mut(), "cs42l43 AMP1_OUT_P", },
    { "Speaker", core::ptr::null_mut(), "cs42l43 AMP1_OUT_N", },
    { "Speaker", core::ptr::null_mut(), "cs42l43 AMP2_OUT_P", },
    { "Speaker", core::ptr::null_mut(), "cs42l43 AMP2_OUT_N", },
    };
    static const struct snd_soc_dapm_route cs42l43_dmic_map[] = {
    { "cs42l43 PDM1_DIN", core::ptr::null_mut(), "DMIC" },
    { "cs42l43 PDM2_DIN", core::ptr::null_mut(), "DMIC" },
    };
    static struct snd_soc_jack_pin soc_jack_pins[] = {
    {
    .pin    = "Headphone",
    .mask   = SND_JACK_HEADPHONE | SND_JACK_LINEOUT,
    },
    {
    .pin    = "Headset Mic",
    .mask   = SND_JACK_MICROPHONE,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs42l43_hs_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs42l43_hs_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = snd_soc_rtd_to_codec(rtd, 0).component;
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(rtd.card);
    struct snd_soc_jack *jack = &ctx.sdw_headset;
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL, "%s hs:cs42l43",
    card.components);
    if (!card.components)
    return -ENOMEM;
    ret = snd_soc_dapm_add_routes(dapm, cs42l43_hs_map,
    ARRAY_SIZE(cs42l43_hs_map));
    if (ret) {
    dev_err(card.dev, "cs42l43 hs map addition failed: %d\n", ret);
    return ret;
    }
    ret = snd_soc_card_jack_new_pins(card, "Jack",
    SND_JACK_MECHANICAL | SND_JACK_AVOUT |
    SND_JACK_HEADSET | SND_JACK_LINEOUT |
    SND_JACK_BTN_0 | SND_JACK_BTN_1 |
    SND_JACK_BTN_2 | SND_JACK_BTN_3,
    jack, soc_jack_pins,
    ARRAY_SIZE(soc_jack_pins));
    if (ret) {
    dev_err(card.dev, "Failed to create jack: %d\n", ret);
    return ret;
    }
    snd_jack_set_key(jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if (ret) {
    dev_err(card.dev, "Failed to register jack: %d\n", ret);
    return ret;
    }
    ret = snd_soc_component_set_sysclk(component, CS42L43_SYSCLK, CS42L43_SYSCLK_SDW,
    0, SND_SOC_CLOCK_IN);
    if (ret)
    dev_err(card.dev, "Failed to set sysclk: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_hs_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs42l43_spk_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs42l43_spk_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret;
    ret = snd_soc_limit_volume(card, "cs42l43 Speaker Digital Volume",
    CS42L43_SPK_VOLUME_0DB);
    if (ret)
    dev_err(card.dev, "cs42l43 speaker volume limit failed: %d\n", ret);
    else
    dev_info(card.dev, "Setting CS42L43 Speaker volume limit to %d\n",
    CS42L43_SPK_VOLUME_0DB);
    ret = snd_soc_dapm_add_routes(dapm, cs42l43_spk_map,
    ARRAY_SIZE(cs42l43_spk_map));
    if (ret) {
    dev_err(card.dev, "cs42l43 speaker map addition failed: %d\n", ret);
    return ret;
    }
    ret = snd_soc_component_set_sysclk(component, CS42L43_SYSCLK, CS42L43_SYSCLK_SDW,
    0, SND_SOC_CLOCK_IN);
    if (ret)
    dev_err(card.dev, "Failed to set sysclk: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_spk_rtd_init, "SND_SOC_SDW_UTILS");
    int asoc_sdw_cs42l43_spk_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
// Do init on playback link only.
    if (!playback)
    return 0;
    info.amp_num++;
    return asoc_sdw_bridge_cs35l56_spk_init(card, dai_links, info, playback);
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_spk_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_cs42l43_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_cs42l43_dmic_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL, "%s mic:cs42l43-dmic",
    card.components);
    if (!card.components)
    return -ENOMEM;
    ret = snd_soc_dapm_add_routes(dapm, cs42l43_dmic_map,
    ARRAY_SIZE(cs42l43_dmic_map));
    if (ret)
    dev_err(card.dev, "cs42l43 dmic map addition failed: %d\n", ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_cs42l43_dmic_rtd_init, "SND_SOC_SDW_UTILS");
