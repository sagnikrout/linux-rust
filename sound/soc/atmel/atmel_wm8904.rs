//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/atmel_wm8904.c
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
// atmel_wm8904 - Atmel ASoC driver for boards with WM8904 codec.
//
// Copyright (C) 2012 Atmel
//
// Author: Bo Shen <voice.shen@atmel.com>
//

    static const struct snd_soc_dapm_widget atmel_asoc_wm8904_dapm_widgets[] = {
    SND_SOC_DAPM_HP("Headphone Jack", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line In Jack", core::ptr::null_mut()),
    };
    static int atmel_asoc_wm8904_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    int ret;
    ret = snd_soc_dai_set_pll(codec_dai, WM8904_FLL_MCLK, WM8904_FLL_MCLK,
    32768, params_rate(params) * 256);
    if (ret < 0) {
    pr_err("%s - failed to set wm8904 codec PLL.", __func__);
    return ret;
    }
//
// As here wm8904 use FLL output as its system clock
// so calling set_sysclk won't care freq parameter
// then we pass 0
//
    ret = snd_soc_dai_set_sysclk(codec_dai, WM8904_CLK_FLL,
    0, SND_SOC_CLOCK_IN);
    if (ret < 0) {
    pr_err("%s -failed to set wm8904 SYSCLK\n", __func__);
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_ops atmel_asoc_wm8904_ops = {
    .hw_params = atmel_asoc_wm8904_hw_params,
    };
    SND_SOC_DAILINK_DEFS(pcm,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC(core::ptr::null_mut(), "wm8904-hifi")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    static struct snd_soc_dai_link atmel_asoc_wm8904_dailink = {
    .name = "WM8904",
    .stream_name = "WM8904 PCM",
    .dai_fmt = SND_SOC_DAIFMT_I2S
    | SND_SOC_DAIFMT_NB_NF
    | SND_SOC_DAIFMT_CBP_CFP,
    .ops = &atmel_asoc_wm8904_ops,
    SND_SOC_DAILINK_REG(pcm),
    };
    static struct snd_soc_card atmel_asoc_wm8904_card = {
    .name = "atmel_asoc_wm8904",
    .owner = THIS_MODULE,
    .dai_link = &atmel_asoc_wm8904_dailink,
    .num_links = 1,
    .dapm_widgets = atmel_asoc_wm8904_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(atmel_asoc_wm8904_dapm_widgets),
    .fully_routed = true,
    };
#[no_mangle]
unsafe extern "C" fn atmel_asoc_wm8904_dt_init(pdev: *mut platform_device) -> c_int {
    static int atmel_asoc_wm8904_dt_init(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct device_node *np = dev.of_node;
    struct device_node *codec_np, *cpu_np;
    struct snd_soc_card *card = &atmel_asoc_wm8904_card;
    struct snd_soc_dai_link *dailink = &atmel_asoc_wm8904_dailink;
    int ret;
    if (!np) {
    dev_err(dev, "only device tree supported\n");
    return -EINVAL;
    }
    ret = snd_soc_of_parse_card_name(card, "atmel,model");
    if (ret) {
    dev_err(dev, "failed to parse card name\n");
    return ret;
    }
    ret = snd_soc_of_parse_audio_routing(card, "atmel,audio-routing");
    if (ret) {
    dev_err(dev, "failed to parse audio routing\n");
    return ret;
    }
    cpu_np = of_parse_phandle(np, "atmel,ssc-controller", 0);
    if (!cpu_np) {
    dev_err(dev, "failed to get dai and pcm info\n");
    ret = -EINVAL;
    return ret;
    }
    dailink.cpus.of_node = cpu_np;
    dailink.platforms.of_node = cpu_np;
    of_node_put(cpu_np);
    codec_np = of_parse_phandle(np, "atmel,audio-codec", 0);
    if (!codec_np) {
    dev_err(dev, "failed to get codec info\n");
    ret = -EINVAL;
    return ret;
    }
    dailink.codecs.of_node = codec_np;
    of_node_put(codec_np);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn atmel_asoc_wm8904_probe(pdev: *mut platform_device) -> c_int {
    static int atmel_asoc_wm8904_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &atmel_asoc_wm8904_card;
    struct snd_soc_dai_link *dailink = &atmel_asoc_wm8904_dailink;
    int id, ret;
    card.dev = &pdev.dev;
    ret = atmel_asoc_wm8904_dt_init(pdev);
    if (ret) {
    dev_err(&pdev.dev, "failed to init dt info\n");
    return ret;
    }
    id = of_alias_get_id((struct device_node *)dailink.cpus.of_node, "ssc");
    ret = atmel_ssc_set_audio(id);
    if (ret != 0) {
    dev_err(&pdev.dev, "failed to set SSC %d for audio\n", id);
    return ret;
    }
    ret = snd_soc_register_card(card);
    if (ret) {
    dev_err(&pdev.dev, "snd_soc_register_card failed\n");
    goto err_set_audio;
    }
    return 0;
    err_set_audio:
    atmel_ssc_put_audio(id);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn atmel_asoc_wm8904_remove(pdev: *mut platform_device) {
    static void atmel_asoc_wm8904_remove(struct platform_device *pdev)
    {
    struct snd_soc_card *card = platform_get_drvdata(pdev);
    struct snd_soc_dai_link *dailink = &atmel_asoc_wm8904_dailink;
    int id;
    id = of_alias_get_id((struct device_node *)dailink.cpus.of_node, "ssc");
    snd_soc_unregister_card(card);
    atmel_ssc_put_audio(id);
    }

    static const struct of_device_id atmel_asoc_wm8904_dt_ids[] = {
    { .compatible = "atmel,asoc-wm8904", },
    { }
    };
    MODULE_DEVICE_TABLE(of, atmel_asoc_wm8904_dt_ids);

    static struct platform_driver atmel_asoc_wm8904_driver = {
    .driver = {
    .name = "atmel-wm8904-audio",
    .of_match_table = of_match_ptr(atmel_asoc_wm8904_dt_ids),
    .pm		= &snd_soc_pm_ops,
    },
    .probe = atmel_asoc_wm8904_probe,
    .remove = atmel_asoc_wm8904_remove,
    };
    module_platform_driver(atmel_asoc_wm8904_driver);
// Module information
    MODULE_AUTHOR("Bo Shen <voice.shen@atmel.com>");
    MODULE_DESCRIPTION("ALSA SoC machine driver for Atmel EK with WM8904 codec");
    MODULE_LICENSE("GPL");
