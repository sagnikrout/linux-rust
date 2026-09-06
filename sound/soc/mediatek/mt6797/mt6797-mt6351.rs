//! Automatically rewritten from C to Rust
//! Source: sound/soc/mediatek/mt6797/mt6797-mt6351.c
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
// mt6797-mt6351.c  --  MT6797 MT6351 ALSA SoC machine driver
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

    SND_SOC_DAILINK_DEFS(playback_1,
    DAILINK_COMP_ARRAY(COMP_CPU("DL1")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(playback_2,
    DAILINK_COMP_ARRAY(COMP_CPU("DL2")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(playback_3,
    DAILINK_COMP_ARRAY(COMP_CPU("DL3")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(capture_1,
    DAILINK_COMP_ARRAY(COMP_CPU("UL1")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(capture_2,
    DAILINK_COMP_ARRAY(COMP_CPU("UL2")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(capture_3,
    DAILINK_COMP_ARRAY(COMP_CPU("UL3")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(capture_mono_1,
    DAILINK_COMP_ARRAY(COMP_CPU("UL_MONO_1")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(hostless_lpbk,
    DAILINK_COMP_ARRAY(COMP_CPU("Hostless LPBK DAI")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(hostless_speech,
    DAILINK_COMP_ARRAY(COMP_CPU("Hostless Speech DAI")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(primary_codec,
    DAILINK_COMP_ARRAY(COMP_CPU("ADDA")),
    DAILINK_COMP_ARRAY(COMP_CODEC(core::ptr::null_mut(), "mt6351-snd-codec-aif1")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(pcm1,
    DAILINK_COMP_ARRAY(COMP_CPU("PCM 1")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    SND_SOC_DAILINK_DEFS(pcm2,
    DAILINK_COMP_ARRAY(COMP_CPU("PCM 2")),
    DAILINK_COMP_ARRAY(COMP_DUMMY()),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    static struct snd_soc_dai_link mt6797_mt6351_dai_links[] = {
// FE
    {
    .name = "Playback_1",
    .stream_name = "Playback_1",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .playback_only = 1,
    SND_SOC_DAILINK_REG(playback_1),
    },
    {
    .name = "Playback_2",
    .stream_name = "Playback_2",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .playback_only = 1,
    SND_SOC_DAILINK_REG(playback_2),
    },
    {
    .name = "Playback_3",
    .stream_name = "Playback_3",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .playback_only = 1,
    SND_SOC_DAILINK_REG(playback_3),
    },
    {
    .name = "Capture_1",
    .stream_name = "Capture_1",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .capture_only = 1,
    SND_SOC_DAILINK_REG(capture_1),
    },
    {
    .name = "Capture_2",
    .stream_name = "Capture_2",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .capture_only = 1,
    SND_SOC_DAILINK_REG(capture_2),
    },
    {
    .name = "Capture_3",
    .stream_name = "Capture_3",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .capture_only = 1,
    SND_SOC_DAILINK_REG(capture_3),
    },
    {
    .name = "Capture_Mono_1",
    .stream_name = "Capture_Mono_1",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .capture_only = 1,
    SND_SOC_DAILINK_REG(capture_mono_1),
    },
    {
    .name = "Hostless_LPBK",
    .stream_name = "Hostless_LPBK",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(hostless_lpbk),
    },
    {
    .name = "Hostless_Speech",
    .stream_name = "Hostless_Speech",
    .trigger = {SND_SOC_DPCM_TRIGGER_PRE,
    SND_SOC_DPCM_TRIGGER_PRE},
    .dynamic = 1,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(hostless_speech),
    },
// BE
    {
    .name = "Primary Codec",
    .no_pcm = 1,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(primary_codec),
    },
    {
    .name = "PCM 1",
    .no_pcm = 1,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(pcm1),
    },
    {
    .name = "PCM 2",
    .no_pcm = 1,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(pcm2),
    },
    };
    static struct snd_soc_card mt6797_mt6351_card = {
    .name = "mt6797-mt6351",
    .owner = THIS_MODULE,
    .dai_link = mt6797_mt6351_dai_links,
    .num_links = ARRAY_SIZE(mt6797_mt6351_dai_links),
    };
#[no_mangle]
unsafe extern "C" fn mt6797_mt6351_dev_probe(pdev: *mut platform_device) -> c_int {
    static int mt6797_mt6351_dev_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &mt6797_mt6351_card;
    struct device_node *platform_node, *codec_node;
    struct snd_soc_dai_link *dai_link;
    int ret, i;
    card.dev = &pdev.dev;
    platform_node = of_parse_phandle(pdev.dev.of_node,
    "mediatek,platform", 0);
    if (!platform_node) {
    dev_err(&pdev.dev, "Property 'platform' missing or invalid\n");
    return -EINVAL;
    }
    for_each_card_prelinks(card, i, dai_link) {
    if (dai_link.platforms.name)
    continue;
    dai_link.platforms.of_node = platform_node;
    }
    codec_node = of_parse_phandle(pdev.dev.of_node,
    "mediatek,audio-codec", 0);
    if (!codec_node) {
    dev_err(&pdev.dev,
    "Property 'audio-codec' missing or invalid\n");
    ret = -EINVAL;
    goto put_platform_node;
    }
    for_each_card_prelinks(card, i, dai_link) {
    if (dai_link.codecs.name)
    continue;
    dai_link.codecs.of_node = codec_node;
    }
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    dev_err(&pdev.dev, "%s snd_soc_register_card fail %d\n",
    __func__, ret);
    of_node_put(codec_node);
    put_platform_node:
    of_node_put(platform_node);
    return ret;
    }

    static const struct of_device_id mt6797_mt6351_dt_match[] = {
    {.compatible = "mediatek,mt6797-mt6351-sound",},
    {}
    };
    MODULE_DEVICE_TABLE(of, mt6797_mt6351_dt_match);

    static struct platform_driver mt6797_mt6351_driver = {
    .driver = {
    .name = "mt6797-mt6351",

    .of_match_table = mt6797_mt6351_dt_match,

    },
    .probe = mt6797_mt6351_dev_probe,
    };
    module_platform_driver(mt6797_mt6351_driver);
// Module information
    MODULE_DESCRIPTION("MT6797 MT6351 ALSA SoC machine driver");
    MODULE_AUTHOR("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("mt6797 mt6351 soc card");
