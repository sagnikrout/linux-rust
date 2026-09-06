//! Automatically rewritten from C to Rust
//! Source: sound/soc/generic/audio-graph-card2-custom-sample.c
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


// SPDX-License-Identifier: GPL-2.0
//
// audio-graph-card2-custom-sample.c
//
// Copyright (C) 2020 Renesas Electronics Corp.
// Copyright (C) 2020 Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>
//

//
// Custom driver can have own priv
// which includes simple_util_priv.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct custom_priv {
    pub simple_priv: simple_util_priv,
// custom driver's own params
    pub custom_params: c_int,
}

// You can get custom_priv from simple_priv

#[no_mangle]
unsafe extern "C" fn custom_card_probe(card: *mut snd_soc_card) -> c_int {
    static int custom_card_probe(struct snd_soc_card *card)
    {
    struct simple_util_priv *simple_priv = snd_soc_card_get_drvdata(card);
    struct custom_priv *custom_priv = simple_to_custom(simple_priv);
    struct device *dev = simple_priv_to_dev(simple_priv);
    dev_info(dev, "custom probe\n");
    custom_priv.custom_params = 1;
// you can use generic probe function
    return graph_util_card_probe(card);
    }
#[no_mangle]
unsafe extern "C" fn custom_hook_pre(priv: *mut simple_util_priv) -> c_int {
    static int custom_hook_pre(struct simple_util_priv *priv)
    {
    struct device *dev = simple_priv_to_dev(priv);
// You can custom before parsing
    dev_info(dev, "hook : %s\n", __func__);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn custom_hook_post(priv: *mut simple_util_priv) -> c_int {
    static int custom_hook_post(struct simple_util_priv *priv)
    {
    struct device *dev = simple_priv_to_dev(priv);
    struct snd_soc_card *card;
// You can custom after parsing
    dev_info(dev, "hook : %s\n", __func__);
// overwrite .probe sample
    card = simple_priv_to_card(priv);
    card.probe = custom_card_probe;
    return 0;
    }
    static int custom_normal(struct simple_util_priv *priv,
    struct device_node *lnk,
    struct link_info *li)
    {
    struct device *dev = simple_priv_to_dev(priv);
//
// You can custom Normal parsing
// before/affter audio_graph2_link_normal()
//
    dev_info(dev, "hook : %s\n", __func__);
    return audio_graph2_link_normal(priv, lnk, li);
    }
    static int custom_dpcm(struct simple_util_priv *priv,
    struct device_node *lnk,
    struct link_info *li)
    {
    struct device *dev = simple_priv_to_dev(priv);
//
// You can custom DPCM parsing
// before/affter audio_graph2_link_dpcm()
//
    dev_info(dev, "hook : %s\n", __func__);
    return audio_graph2_link_dpcm(priv, lnk, li);
    }
    static int custom_c2c(struct simple_util_priv *priv,
    struct device_node *lnk,
    struct link_info *li)
    {
    struct device *dev = simple_priv_to_dev(priv);
//
// You can custom Codec2Codec parsing
// before/affter audio_graph2_link_c2c()
//
    dev_info(dev, "hook : %s\n", __func__);
    return audio_graph2_link_c2c(priv, lnk, li);
    }
//
// audio-graph-card2 has many hooks for your customizing.
//
    static struct graph2_custom_hooks custom_hooks = {
    .hook_pre	= custom_hook_pre,
    .hook_post	= custom_hook_post,
    .custom_normal	= custom_normal,
    .custom_dpcm	= custom_dpcm,
    .custom_c2c	= custom_c2c,
    };
#[no_mangle]
unsafe extern "C" fn custom_startup(substream: *mut snd_pcm_substream) -> c_int {
    static int custom_startup(struct snd_pcm_substream *substream)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct simple_util_priv *priv = snd_soc_card_get_drvdata(rtd.card);
    struct device *dev = simple_priv_to_dev(priv);
    dev_info(dev, "custom startup\n");
    return simple_util_startup(substream);
    }
// You can use custom ops
    static const struct snd_soc_ops custom_ops = {
    .startup	= custom_startup,
    .shutdown	= simple_util_shutdown,
    .hw_params	= simple_util_hw_params,
    };
#[no_mangle]
unsafe extern "C" fn custom_probe(pdev: *mut platform_device) -> c_int {
    static int custom_probe(struct platform_device *pdev)
    {
    struct custom_priv *custom_priv;
    struct simple_util_priv *simple_priv;
    struct device *dev = &pdev.dev;
    int ret;
    custom_priv = devm_kzalloc(dev, sizeof(*custom_priv), GFP_KERNEL);
    if (!custom_priv)
    return -ENOMEM;
    simple_priv		= &custom_priv.simple_priv;
    simple_priv.ops	= &custom_ops; /* customize dai_link ops */
// "audio-graph-card2-custom-sample" is too long
    simple_priv.snd_card.name = "card2-custom";
// use audio-graph-card2 parsing with own custom hooks
    ret = audio_graph2_parse_of(simple_priv, dev, &custom_hooks);
    if (ret < 0)
    return ret;
// customize more if needed
    return 0;
    }
    static const struct of_device_id custom_of_match[] = {
    { .compatible = "audio-graph-card2-custom-sample", },
    {},
    };
    MODULE_DEVICE_TABLE(of, custom_of_match);
    static struct platform_driver custom_card = {
    .driver = {
    .name = "audio-graph-card2-custom-sample",
    .of_match_table = custom_of_match,
    },
    .probe	= custom_probe,
    .remove = simple_util_remove,
    };
    module_platform_driver(custom_card);
    MODULE_ALIAS("platform:asoc-audio-graph-card2-custom-sample");
    MODULE_LICENSE("GPL v2");
    MODULE_DESCRIPTION("ASoC Audio Graph Card2 Custom Sample");
    MODULE_AUTHOR("Kuninori Morimoto <kuninori.morimoto.gx@renesas.com>");
