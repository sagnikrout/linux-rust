//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/sam9g20_wm8731.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// sam9g20_wm8731  --  SoC audio for AT91SAM9G20-based
// ATMEL AT91SAM9G20ek board.
//
// Copyright (C) 2005 SAN People
// Copyright (C) 2008 Atmel
//
// Authors: Sedji Gaouaou <sedji.gaouaou@atmel.com>
//
// Based on ati_b1_wm8731.c by:
// Frank Mandarino <fmandarino@endrelia.com>
// Copyright 2006 Endrelia Technologies Inc.
// Based on corgi.c by:
// Copyright 2005 Wolfson Microelectronics PLC.
// Copyright 2005 Openedhand Ltd.
//

pub const MCLK_RATE: c_int = 12000000;
//
// As shipped the board does not have inputs.  However, it is relatively
// straightforward to modify the board to hook them up so support is left
// in the driver.
//

    static const struct snd_soc_dapm_widget at91sam9g20ek_dapm_widgets[] = {
    SND_SOC_DAPM_MIC("Int Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Ext Spk", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route intercon[] = {
// speaker connected to LHPOUT/RHPOUT
    {"Ext Spk", core::ptr::null_mut(), "LHPOUT"},
    {"Ext Spk", core::ptr::null_mut(), "RHPOUT"},
// mic is connected to Mic Jack, with WM8731 Mic Bias
    {"MICIN", core::ptr::null_mut(), "Mic Bias"},
    {"Mic Bias", core::ptr::null_mut(), "Int Mic"},
    };
//
// Logic for a wm8731 as connected on a at91sam9g20ek board.
//
#[no_mangle]
unsafe extern "C" fn at91sam9g20ek_wm8731_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int at91sam9g20ek_wm8731_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    struct device *dev = rtd.dev;
    int ret;
    dev_dbg(dev, "%s called\n", __func__);
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8731_SYSCLK_MCLK,
    MCLK_RATE, SND_SOC_CLOCK_IN);
    if (ret < 0) {
    dev_err(dev, "Failed to set WM8731 SYSCLK: %d\n", ret);
    return ret;
    }

    snd_soc_dapm_disable_pin(snd_soc_card_to_dapm(rtd.card), "Int Mic");

    return 0;
    }
    SND_SOC_DAILINK_DEFS(pcm,
    DAILINK_COMP_ARRAY(COMP_CPU("at91rm9200_ssc.0")),
    DAILINK_COMP_ARRAY(COMP_CODEC("wm8731.0-001b", "wm8731-hifi")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("at91rm9200_ssc.0")));
    static struct snd_soc_dai_link at91sam9g20ek_dai = {
    .name = "WM8731",
    .stream_name = "WM8731 PCM",
    .init = at91sam9g20ek_wm8731_init,
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBP_CFP,

    .playback_only = true,

    SND_SOC_DAILINK_REG(pcm),
    };
    static struct snd_soc_card snd_soc_at91sam9g20ek = {
    .name = "AT91SAMG20-EK",
    .owner = THIS_MODULE,
    .dai_link = &at91sam9g20ek_dai,
    .num_links = 1,
    .dapm_widgets = at91sam9g20ek_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(at91sam9g20ek_dapm_widgets),
    .dapm_routes = intercon,
    .num_dapm_routes = ARRAY_SIZE(intercon),
    .fully_routed = true,
    };
#[no_mangle]
unsafe extern "C" fn at91sam9g20ek_audio_probe(pdev: *mut platform_device) -> c_int {
    static int at91sam9g20ek_audio_probe(struct platform_device *pdev)
    {
    struct device_node *np = pdev.dev.of_node;
    struct device_node *codec_np, *cpu_np;
    struct snd_soc_card *card = &snd_soc_at91sam9g20ek;
    int ret;
    if (!np) {
    return -ENODEV;
    }
    ret = atmel_ssc_set_audio(0);
    if (ret) {
    dev_err(&pdev.dev, "ssc channel is not valid: %d\n", ret);
    return ret;
    }
    card.dev = &pdev.dev;
// Parse device node info
    ret = snd_soc_of_parse_card_name(card, "atmel,model");
    if (ret)
    goto err;
    ret = snd_soc_of_parse_audio_routing(card,
    "atmel,audio-routing");
    if (ret)
    goto err;
// Parse codec info
    at91sam9g20ek_dai.codecs.name = core::ptr::null_mut();
    codec_np = of_parse_phandle(np, "atmel,audio-codec", 0);
    if (!codec_np) {
    dev_err(&pdev.dev, "codec info missing\n");
    ret = -EINVAL;
    goto err;
    }
    at91sam9g20ek_dai.codecs.of_node = codec_np;
// Parse dai and platform info
    at91sam9g20ek_dai.cpus.dai_name = core::ptr::null_mut();
    at91sam9g20ek_dai.platforms.name = core::ptr::null_mut();
    cpu_np = of_parse_phandle(np, "atmel,ssc-controller", 0);
    if (!cpu_np) {
    dev_err(&pdev.dev, "dai and pcm info missing\n");
    of_node_put(codec_np);
    ret = -EINVAL;
    goto err;
    }
    at91sam9g20ek_dai.cpus.of_node = cpu_np;
    at91sam9g20ek_dai.platforms.of_node = cpu_np;
    of_node_put(codec_np);
    of_node_put(cpu_np);
    ret = snd_soc_register_card(card);
    if (ret) {
    dev_err_probe(&pdev.dev, ret,
    "snd_soc_register_card() failed\n");
    goto err;
    }
    return 0;
    err:
    atmel_ssc_put_audio(0);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91sam9g20ek_audio_remove(pdev: *mut platform_device) {
    static void at91sam9g20ek_audio_remove(struct platform_device *pdev)
    {
    struct snd_soc_card *card = platform_get_drvdata(pdev);
    snd_soc_unregister_card(card);
    atmel_ssc_put_audio(0);
    }

    static const struct of_device_id at91sam9g20ek_wm8731_dt_ids[] = {
    { .compatible = "atmel,at91sam9g20ek-wm8731-audio", },
    { }
    };
    MODULE_DEVICE_TABLE(of, at91sam9g20ek_wm8731_dt_ids);

    static struct platform_driver at91sam9g20ek_audio_driver = {
    .driver = {
    .name	= "at91sam9g20ek-audio",
    .of_match_table = of_match_ptr(at91sam9g20ek_wm8731_dt_ids),
    },
    .probe	= at91sam9g20ek_audio_probe,
    .remove = at91sam9g20ek_audio_remove,
    };
    module_platform_driver(at91sam9g20ek_audio_driver);
// Module information
    MODULE_AUTHOR("Sedji Gaouaou <sedji.gaouaou@atmel.com>");
    MODULE_DESCRIPTION("ALSA SoC AT91SAM9G20EK_WM8731");
    MODULE_ALIAS("platform:at91sam9g20ek-audio");
    MODULE_LICENSE("GPL");
