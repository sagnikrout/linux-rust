//! Automatically rewritten from C to Rust
//! Source: sound/soc/sdw_utils/soc_sdw_rt711.c
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
// soc_sdw_rt711 - Helpers to handle RT711 from generic machine driver
//

//
// Note this MUST be called before snd_soc_register_card(), so that the props
// are in place before the codec component driver's probe function parses them.
//
#[no_mangle]
unsafe extern "C" fn rt711_add_codec_device_props(sdw_dev: *mut device, quirk: c_ulong) -> c_int {
    static int rt711_add_codec_device_props(struct device *sdw_dev, unsigned long quirk)
    {
    struct property_entry props[SOC_SDW_MAX_NO_PROPS] = {};
    struct fwnode_handle *fwnode;
    int ret;
    if (!SOC_SDW_JACK_JDSRC(quirk))
    return 0;
    props[0] = PROPERTY_ENTRY_U32("realtek,jd-src", SOC_SDW_JACK_JDSRC(quirk));
    fwnode = fwnode_create_software_node(props, core::ptr::null_mut());
    if (IS_ERR(fwnode))
    return PTR_ERR(fwnode);
    ret = device_add_software_node(sdw_dev, to_software_node(fwnode));
    fwnode_handle_put(fwnode);
    return ret;
    }
    static const struct snd_soc_dapm_route rt711_map[] = {
// Headphones
    { "Headphone", core::ptr::null_mut(), "rt711 HP" },
    { "rt711 MIC2", core::ptr::null_mut(), "Headset Mic" },
    };
    static struct snd_soc_jack_pin rt711_jack_pins[] = {
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
pub unsafe extern "C" fn asoc_sdw_rt711_rtd_init(rtd: *mut snd_soc_pcm_runtime, dai: *mut snd_soc_dai) -> c_int {
    int asoc_sdw_rt711_rtd_init(struct snd_soc_pcm_runtime *rtd, struct snd_soc_dai *dai)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(card);
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct snd_soc_component *component;
    struct snd_soc_jack *jack;
    int ret;
    component = dai.component;
    card.components = devm_kasprintf(card.dev, GFP_KERNEL,
    "%s hs:rt711",
    card.components);
    if (!card.components)
    return -ENOMEM;
    ret = snd_soc_dapm_add_routes(dapm, rt711_map,
    ARRAY_SIZE(rt711_map));
    if (ret) {
    dev_err(card.dev, "rt711 map addition failed: %d\n", ret);
    return ret;
    }
    ret = snd_soc_card_jack_new_pins(rtd.card, "Headset Jack",
    SND_JACK_HEADSET | SND_JACK_BTN_0 |
    SND_JACK_BTN_1 | SND_JACK_BTN_2 |
    SND_JACK_BTN_3,
    &ctx.sdw_headset,
    rt711_jack_pins,
    ARRAY_SIZE(rt711_jack_pins));
    if (ret) {
    dev_err(rtd.card.dev, "Headset Jack creation failed: %d\n",
    ret);
    return ret;
    }
    jack = &ctx.sdw_headset;
    snd_jack_set_key(jack.jack, SND_JACK_BTN_0, KEY_PLAYPAUSE);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_1, KEY_VOICECOMMAND);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_2, KEY_VOLUMEUP);
    snd_jack_set_key(jack.jack, SND_JACK_BTN_3, KEY_VOLUMEDOWN);
    ret = snd_soc_component_set_jack(component, jack, core::ptr::null_mut());
    if (ret)
    dev_err(rtd.card.dev, "Headset Jack call-back failed: %d\n",
    ret);
    return ret;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt711_rtd_init, "SND_SOC_SDW_UTILS");
#[no_mangle]
pub unsafe extern "C" fn asoc_sdw_rt711_exit(card: *mut snd_soc_card, dai_link: *mut snd_soc_dai_link) -> c_int {
    int asoc_sdw_rt711_exit(struct snd_soc_card *card, struct snd_soc_dai_link *dai_link)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    if (!ctx.headset_codec_dev)
    return 0;
    device_remove_software_node(ctx.headset_codec_dev);
    put_device(ctx.headset_codec_dev);
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt711_exit, "SND_SOC_SDW_UTILS");
    int asoc_sdw_rt711_init(struct snd_soc_card *card,
    struct snd_soc_dai_link *dai_links,
    struct asoc_sdw_codec_info *info,
    bool playback)
    {
    struct asoc_sdw_mc_private *ctx = snd_soc_card_get_drvdata(card);
    struct device *sdw_dev;
    int ret;
//
// headset should be initialized once.
// Do it with dai link for playback.
//
    if (!playback)
    return 0;
    sdw_dev = bus_find_device_by_name(&sdw_bus_type, core::ptr::null_mut(), dai_links.codecs[0].name);
    if (!sdw_dev)
    return -EPROBE_DEFER;
    ret = rt711_add_codec_device_props(sdw_dev, ctx.mc_quirk);
    if (ret < 0) {
    put_device(sdw_dev);
    return ret;
    }
    ctx.headset_codec_dev = sdw_dev;
    return 0;
    }
    EXPORT_SYMBOL_NS(asoc_sdw_rt711_init, "SND_SOC_SDW_UTILS");
