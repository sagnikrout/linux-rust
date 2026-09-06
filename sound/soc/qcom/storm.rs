//! Automatically rewritten from C to Rust
//! Source: sound/soc/qcom/storm.c
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
//
// Copyright (c) 2010-2011,2013-2015 The Linux Foundation. All rights reserved.
//
// storm.c -- ALSA SoC machine driver for QTi ipq806x-based Storm board
//

pub const STORM_SYSCLK_MULT: c_int = 4;
    static int storm_ops_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    struct snd_soc_pcm_runtime *soc_runtime = snd_soc_substream_to_rtd(substream);
    struct snd_soc_card *card = soc_runtime.card;
    let mut format: snd_pcm_format_t = params_format(params);
    let mut rate: c_uint = params_rate(params);
    unsigned int sysclk_freq;
    int bitwidth, ret;
    bitwidth = snd_pcm_format_width(format);
    if (bitwidth < 0) {
    dev_err(card.dev, "invalid bit width given: %d\n", bitwidth);
    return bitwidth;
    }
//
// as the CPU DAI is the I2S bus master and no system clock is needed by
// the MAX98357a DAC, simply set the system clock to be a constant
// multiple of the bit clock for the clock divider
//
    sysclk_freq = rate * bitwidth * 2 * STORM_SYSCLK_MULT;
    ret = snd_soc_dai_set_sysclk(snd_soc_rtd_to_cpu(soc_runtime, 0), 0, sysclk_freq, 0);
    if (ret) {
    dev_err(card.dev, "error setting sysclk to %u: %d\n",
    sysclk_freq, ret);
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_ops storm_soc_ops = {
    .hw_params	= storm_ops_hw_params,
    };
    SND_SOC_DAILINK_DEFS(hifi,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC(core::ptr::null_mut(), "HiFi")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    static struct snd_soc_dai_link storm_dai_link = {
    .name		= "Primary",
    .stream_name	= "Primary",
    .ops		= &storm_soc_ops,
    SND_SOC_DAILINK_REG(hifi),
    };
#[no_mangle]
unsafe extern "C" fn storm_parse_of(dev: *mut device) -> c_int {
    static int storm_parse_of(struct device *dev)
    {
    struct snd_soc_dai_link *dai_link = &storm_dai_link;
    struct device_node *np = dev.of_node;
    dai_link.cpus.of_node = of_parse_phandle(np, "cpu", 0);
    if (!dai_link.cpus.of_node) {
    dev_err(dev, "error getting cpu phandle\n");
    return -EINVAL;
    }
    dai_link.platforms.of_node = dai_link.cpus.of_node;
    dai_link.codecs.of_node = of_parse_phandle(np, "codec", 0);
    if (!dai_link.codecs.of_node) {
    dev_err(dev, "error getting codec phandle\n");
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn storm_platform_probe(pdev: *mut platform_device) -> c_int {
    static int storm_platform_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card;
    int ret;
    card = devm_kzalloc(&pdev.dev, sizeof(*card), GFP_KERNEL);
    if (!card)
    return -ENOMEM;
    card.dev = &pdev.dev;
    card.owner = THIS_MODULE;
    ret = snd_soc_of_parse_card_name(card, "qcom,model");
    if (ret) {
    dev_err(&pdev.dev, "error parsing card name: %d\n", ret);
    return ret;
    }
    card.dai_link	= &storm_dai_link;
    card.num_links	= 1;
    ret = storm_parse_of(&pdev.dev);
    if (ret) {
    dev_err(&pdev.dev, "error resolving dai links: %d\n", ret);
    return ret;
    }
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    dev_err(&pdev.dev, "error registering soundcard: %d\n", ret);
    return ret;
    }

    static const struct of_device_id storm_device_id[]  = {
    { .compatible = "google,storm-audio" },
    {},
    };
    MODULE_DEVICE_TABLE(of, storm_device_id);

    static struct platform_driver storm_platform_driver = {
    .driver = {
    .name = "storm-audio",
    .of_match_table =
    of_match_ptr(storm_device_id),
    },
    .probe = storm_platform_probe,
    };
    module_platform_driver(storm_platform_driver);
    MODULE_DESCRIPTION("QTi IPQ806x-based Storm Machine Driver");
    MODULE_LICENSE("GPL");
