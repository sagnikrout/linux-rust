//! Automatically rewritten from C to Rust
//! Source: sound/soc/atmel/mikroe-proto.c
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
// ASoC driver for PROTO AudioCODEC (with a WM8731)
//
// Author:      Florian Meier, <koalo@koalo.de>
// Copyright 2013
//

#[no_mangle]
unsafe extern "C" fn snd_proto_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int snd_proto_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_card *card = rtd.card;
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
// Set proto sysclk
    int ret = snd_soc_dai_set_sysclk(codec_dai, WM8731_SYSCLK_XTAL,
    XTAL_RATE, SND_SOC_CLOCK_IN);
    if (ret < 0) {
    dev_err(card.dev, "Failed to set WM8731 SYSCLK: %d\n",
    ret);
    return ret;
    }
    return 0;
    }
    static const struct snd_soc_dapm_widget snd_proto_widget[] = {
    SND_SOC_DAPM_MIC("Microphone Jack", core::ptr::null_mut()),
    SND_SOC_DAPM_HP("Headphone Jack", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route snd_proto_route[] = {
// speaker connected to LHPOUT/RHPOUT
    {"Headphone Jack", core::ptr::null_mut(), "LHPOUT"},
    {"Headphone Jack", core::ptr::null_mut(), "RHPOUT"},
// mic is connected to Mic Jack, with WM8731 Mic Bias
    {"MICIN", core::ptr::null_mut(), "Mic Bias"},
    {"Mic Bias", core::ptr::null_mut(), "Microphone Jack"},
    };
// audio machine driver
    static struct snd_soc_card snd_proto = {
    .name		= "snd_mikroe_proto",
    .owner		= THIS_MODULE,
    .dapm_widgets	= snd_proto_widget,
    .num_dapm_widgets = ARRAY_SIZE(snd_proto_widget),
    .dapm_routes	= snd_proto_route,
    .num_dapm_routes = ARRAY_SIZE(snd_proto_route),
    };
#[no_mangle]
unsafe extern "C" fn snd_proto_probe(pdev: *mut platform_device) -> c_int {
    static int snd_proto_probe(struct platform_device *pdev)
    {
    struct snd_soc_dai_link *dai;
    struct snd_soc_dai_link_component *comp;
    struct device_node *np = pdev.dev.of_node;
    struct device_node *codec_np, *cpu_np;
    struct device_node *bitclkmaster = core::ptr::null_mut();
    struct device_node *framemaster = core::ptr::null_mut();
    unsigned int dai_fmt;
    let mut ret: c_int = 0;
    if (!np) {
    dev_err(&pdev.dev, "No device node supplied\n");
    return -EINVAL;
    }
    snd_proto.dev = &pdev.dev;
    ret = snd_soc_of_parse_card_name(&snd_proto, "model");
    if (ret)
    return ret;
    dai = devm_kzalloc(&pdev.dev, sizeof(*dai), GFP_KERNEL);
    if (!dai)
    return -ENOMEM;
// for cpus/codecs/platforms
    comp = devm_kzalloc(&pdev.dev, 3 * sizeof(*comp), GFP_KERNEL);
    if (!comp)
    return -ENOMEM;
    snd_proto.dai_link = dai;
    snd_proto.num_links = 1;
    dai.cpus = &comp[0];
    dai.num_cpus = 1;
    dai.codecs = &comp[1];
    dai.num_codecs = 1;
    dai.platforms = &comp[2];
    dai.num_platforms = 1;
    dai.name = "WM8731";
    dai.stream_name = "WM8731 HiFi";
    dai.codecs.dai_name = "wm8731-hifi";
    dai.init = &snd_proto_init;
    codec_np = of_parse_phandle(np, "audio-codec", 0);
    if (!codec_np) {
    dev_err(&pdev.dev, "audio-codec node missing\n");
    return -EINVAL;
    }
    dai.codecs.of_node = codec_np;
    cpu_np = of_parse_phandle(np, "i2s-controller", 0);
    if (!cpu_np) {
    dev_err(&pdev.dev, "i2s-controller missing\n");
    ret = -EINVAL;
    goto put_codec_node;
    }
    dai.cpus.of_node = cpu_np;
    dai.platforms.of_node = cpu_np;
    dai_fmt = snd_soc_daifmt_parse_format(np, core::ptr::null_mut());
    snd_soc_daifmt_parse_clock_provider_as_phandle(np, core::ptr::null_mut(),
    &bitclkmaster, &framemaster);
    if (bitclkmaster != framemaster) {
    dev_err(&pdev.dev, "Must be the same bitclock and frame master\n");
    ret = -EINVAL;
    goto put_cpu_node;
    }
    if (bitclkmaster) {
    if (codec_np == bitclkmaster)
    dai_fmt |= SND_SOC_DAIFMT_CBP_CFP;
    else
    dai_fmt |= SND_SOC_DAIFMT_CBC_CFC;
    } else {
    dai_fmt |= snd_soc_daifmt_parse_clock_provider_as_flag(np, core::ptr::null_mut());
    }
    dai.dai_fmt = dai_fmt;
    ret = devm_snd_soc_register_card(&pdev.dev, &snd_proto);
    if (ret)
    dev_err_probe(&pdev.dev, ret,
    "snd_soc_register_card() failed\n");
    put_cpu_node:
    of_node_put(bitclkmaster);
    of_node_put(framemaster);
    of_node_put(cpu_np);
    put_codec_node:
    of_node_put(codec_np);
    return ret;
    }
    static const struct of_device_id snd_proto_of_match[] = {
    { .compatible = "mikroe,mikroe-proto", },
    {},
    };
    MODULE_DEVICE_TABLE(of, snd_proto_of_match);
    static struct platform_driver snd_proto_driver = {
    .driver = {
    .name   = "snd-mikroe-proto",
    .of_match_table = snd_proto_of_match,
    },
    .probe	  = snd_proto_probe,
    };
    module_platform_driver(snd_proto_driver);
    MODULE_AUTHOR("Florian Meier");
    MODULE_DESCRIPTION("ASoC Driver for PROTO board (WM8731)");
    MODULE_LICENSE("GPL");
