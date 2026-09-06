//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_ti_amp.c
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
// Copyright (c) 2025 Texas Instruments Inc.
//
// soc_sdw_ti_amp - Helpers to handle TI's soundwire based codecs
//

pub const TIAMP_SPK_VOLUME_0DB: c_int = 200;
pub const TAC5XX2_WIDGET_NAME_MAX: c_int = 32;
    int asoc_sdw_ti_amp_initial_settings(struct snd_soc_card *card,
    const char *name_prefix)
    {
    char *volume_ctl_name;
    int ret;
    volume_ctl_name = kasprintf(GFP_KERNEL, "%s Speaker Volume",
    name_prefix);
    if (!volume_ctl_name)
    return -ENOMEM;
    ret = snd_soc_limit_volume(card, volume_ctl_name,
    TIAMP_SPK_VOLUME_0DB);
    if (ret)
    dev_err(card.dev,
    "%s update failed %d\n",
    volume_ctl_name, ret);
    kfree(volume_ctl_name);
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_amp_initial_settings, "SND_SOC_SDW_UTILS");
    int asoc_sdw_ti_spk_rtd_init(struct snd_soc_pcm_runtime *rtd,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    char widget_name[16];
    char speaker[16];
    let mut route: snd_soc_dapm_route = {speaker, core::ptr::null_mut(), widget_name};
    struct snd_soc_dai *codec_dai;
    const char *prefix;
    int i, ret = 0;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    if (!strstr(codec_dai.name, "tas2783"))
    continue;
    prefix = codec_dai.component.name_prefix;
    if (!strncmp(prefix, "tas2783-1", strlen("tas2783-1"))) {
    strscpy(speaker, "Left Spk", sizeof(speaker));
    } else if (!strncmp(prefix, "tas2783-2", strlen("tas2783-2"))) {
    strscpy(speaker, "Right Spk", sizeof(speaker));
    } else if (!strncmp(prefix, "tas2783-3", strlen("tas2783-3"))) {
    strscpy(speaker, "Left Spk2", sizeof(speaker));
    } else if (!strncmp(prefix, "tas2783-4", strlen("tas2783-4"))) {
    strscpy(speaker, "Right Spk2", sizeof(speaker));
    } else {
    ret = -EINVAL;
    dev_err(card.dev, "unhandled prefix %s", prefix);
    break;
    }
    snprintf(widget_name, sizeof(widget_name), "%s SPK", prefix);
    ret = asoc_sdw_ti_amp_initial_settings(card, prefix);
    if (ret)
    return ret;
    ret = snd_soc_dapm_add_routes(dapm, &route, 1);
    if (ret)
    return ret;
    }
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_spk_rtd_init, "SND_SOC_SDW_UTILS");
    int asoc_sdw_ti_amp_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
    if (!playback)
    return 0;
    info.amp_num++;
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_amp_init, "SND_SOC_SDW_UTILS");
    static int asoc_sdw_ti_add_tac5xx2_routes(struct snd_soc_dapm_context *dapm,
    const char *name_prefix)
    {
    struct snd_soc_dapm_route routes[2];
    char left_widget[TAC5XX2_WIDGET_NAME_MAX];
    char right_widget[TAC5XX2_WIDGET_NAME_MAX];
    if (strlen(name_prefix) > (TAC5XX2_WIDGET_NAME_MAX - 7))
    return -ENAMETOOLONG;
    scnprintf(left_widget, sizeof(left_widget), "%s SPK_L", name_prefix);
    scnprintf(right_widget, sizeof(right_widget), "%s SPK_R", name_prefix);
    routes[0] = (struct snd_soc_dapm_route){"Left Spk", core::ptr::null_mut(), left_widget};
    routes[1] = (struct snd_soc_dapm_route){"Right Spk", core::ptr::null_mut(), right_widget};
    return snd_soc_dapm_add_routes(dapm, routes, ARRAY_SIZE(routes));
    }
    int asoc_sdw_ti_tac5xx2_spk_rtd_init(struct snd_soc_pcm_runtime *rtd,
    struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    int ret, i;
    struct snd_soc_dai *codec_dai;
    const char *prefix;
    for_each_rtd_codec_dais(rtd, i, codec_dai) {
    if (!strstr(codec_dai.name, "tac5") &&
    !strstr(codec_dai.name, "tas2883"))
    continue;
    prefix = codec_dai.component.name_prefix;
    if (!prefix) {
    dev_warn(card.dev,
    "No name prefix found for codec DAI: %s\n",
    codec_dai.name);
    continue;
    }
    ret = asoc_sdw_ti_add_tac5xx2_routes(dapm, prefix);
    if (ret) {
    dev_err(card.dev, "Failed to add routes for %s: %d\n",
    prefix, ret);
    return ret;
    }
    }
    dev_dbg(card.dev, "Added TAC5XX2 speaker routes\n");
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_tac5xx2_spk_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_ti_dmic_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_ti_dmic_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_component *component;
    component = dai.component;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "%s mic:%s", card.components,
    component.name_prefix);
    if (!card.components)
    return -ENOMEM;
    dev_dbg(card.dev, "card.components: %s\n", card.components);
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_dmic_rtd_init, "SND_SOC_SDW_UTILS");
    static struct snd_soc_jack_pin ti_sdca_jack_pins[] = {
    {
    .pin    = "Headphone",
    .mask   = SND_JACK_HEADPHONE,
    },
    {
    .pin    = "Headset Mic",
    .mask   = SND_JACK_MICROPHONE,
    },
    };
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_ti_sdca_jack_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_ti_sdca_jack_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct snd_soc_component *component;
    struct snd_soc_jack *jack;
    int ret;
    component = dai.component;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "%s hs:%s", card.components,
    component.name_prefix);
    if (!card.components)
    return -ENOMEM;
    ret = snd_soc_card_jack_new_pins(rtd.card, "Headset Jack",
    SND_JACK_HEADSET | SND_JACK_BTN_0 |
    SND_JACK_BTN_1 | SND_JACK_BTN_2 |
    SND_JACK_BTN_3 | SND_JACK_BTN_4,
    &ctx.sdw_headset,
    ti_sdca_jack_pins,
    ARRAY_SIZE(ti_sdca_jack_pins));
    if (ret) {
    dev_err(rtd.card.dev, "Jack create failed%d\n", ret);
    return ret;
    }
    jack = &ctx.sdw_headset;
    snd_jack_set_key(jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_4, KEY_NEXTSONG);
    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if (ret)
    dev_err(rtd.card.dev, "Headset Jack call-back failed: %d\n",
    ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_ti_sdca_jack_rtd_init, "SND_SOC_SDW_UTILS");
