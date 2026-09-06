//! Automatically rewritten from C to Rust
//! Source: sound/soc/rockchip/rockchip_rt5645.c
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
// Rockchip machine ASoC driver for boards using a RT5645/RT5650 CODEC.
//
// Copyright (c) 2015, ROCKCHIP CORPORATION.  All rights reserved.
//

    static struct snd_soc_jack headset_jack;
    static struct snd_soc_jack_pin headset_jack_pins[] = {
    {
    .pin = "Headphones",
    .mask = SND_JACK_HEADPHONE,
    },
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    };
    static const struct snd_soc_dapm_widget rk_dapm_widgets[] = {
    SND_SOC_DAPM_HP("Headphones", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Speakers", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Int Mic", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route rk_audio_map[] = {
// Input Lines
    {"DMIC L2", core::ptr::null_mut(), "Int Mic"},
    {"DMIC R2", core::ptr::null_mut(), "Int Mic"},
    {"RECMIXL", core::ptr::null_mut(), "Headset Mic"},
    {"RECMIXR", core::ptr::null_mut(), "Headset Mic"},
// Output Lines
    {"Headphones", core::ptr::null_mut(), "HPOR"},
    {"Headphones", core::ptr::null_mut(), "HPOL"},
    {"Speakers", core::ptr::null_mut(), "SPOL"},
    {"Speakers", core::ptr::null_mut(), "SPOR"},
    };
    static const struct snd_kcontrol_new rk_mc_controls[] = {
    SOC_DAPM_PIN_SWITCH("Headphones"),
    SOC_DAPM_PIN_SWITCH("Speakers"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("Int Mic"),
    };
    static int rk_aif1_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    let mut ret: c_int = 0;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    int mclk;
    switch (params_rate(params)) {
    case 8000:
    case 16000:
    case 24000:
    case 32000:
    case 48000:
    case 64000:
    case 96000:
    mclk = 12288000;
    break;
    case 11025:
    case 22050:
    case 44100:
    case 88200:
    mclk = 11289600;
    break;
    default:
    return -EINVAL;
    }
    ret = snd_soc_dai_set_sysclk(cpu_dai, 0, mclk,
    SND_SOC_CLOCK_OUT);
    if (ret < 0) {
    dev_err(codec_dai.dev, "Can't set codec clock %d\n", ret);
    return ret;
    }
    ret = snd_soc_dai_set_sysclk(codec_dai, 0, mclk,
    SND_SOC_CLOCK_IN);
    if (ret < 0) {
    dev_err(codec_dai.dev, "Can't set codec clock %d\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn rk_init(runtime: *mut snd_soc_pcm_runtime) -> c_int {
    static int rk_init(struct snd_soc_pcm_runtime *runtime)
    {
    struct snd_soc_card *card = runtime.card;
    int ret;
// Enable Headset and 4 Buttons Jack detection
    ret = snd_soc_card_jack_new_pins(card, "Headset Jack",
    SND_JACK_HEADPHONE | SND_JACK_MICROPHONE |
    SND_JACK_BTN_0 | SND_JACK_BTN_1 |
    SND_JACK_BTN_2 | SND_JACK_BTN_3,
    &headset_jack,
    headset_jack_pins,
    ARRAY_SIZE(headset_jack_pins));
    if (ret) {
    dev_err(card.dev, "New Headset Jack failed! (%d)\n", ret);
    return ret;
    }
    return rt5645_set_jack_detect(snd_soc_rtd_to_codec(runtime, 0).component,
    &headset_jack,
    &headset_jack,
    &headset_jack);
    }
    static const struct snd_soc_ops rk_aif1_ops = {
    .hw_params = rk_aif1_hw_params,
    };
    SND_SOC_DAILINK_DEFS(pcm,
    DAILINK_COMP_ARRAY(COMP_EMPTY()),
    DAILINK_COMP_ARRAY(COMP_CODEC(core::ptr::null_mut(), "rt5645-aif1")),
    DAILINK_COMP_ARRAY(COMP_EMPTY()));
    static struct snd_soc_dai_link rk_dailink = {
    .name = "rt5645",
    .stream_name = "rt5645 PCM",
    .init = rk_init,
    .ops = &rk_aif1_ops,
// set rt5645 as slave
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBC_CFC,
    SND_SOC_DAILINK_REG(pcm),
    };
    static struct snd_soc_card snd_soc_card_rk = {
    .name = "I2S-RT5650",
    .owner = THIS_MODULE,
    .dai_link = &rk_dailink,
    .num_links = 1,
    .dapm_widgets = rk_dapm_widgets,
    .num_dapm_widgets = ARRAY_SIZE(rk_dapm_widgets),
    .dapm_routes = rk_audio_map,
    .num_dapm_routes = ARRAY_SIZE(rk_audio_map),
    .controls = rk_mc_controls,
    .num_controls = ARRAY_SIZE(rk_mc_controls),
    };
#[no_mangle]
unsafe extern "C" fn snd_rk_mc_probe(pdev: *mut platform_device) -> c_int {
    static int snd_rk_mc_probe(struct platform_device *pdev)
    {
    let mut ret: c_int = 0;
    struct snd_soc_card *card = &snd_soc_card_rk;
    struct device_node *np = pdev.dev.of_node;
// register the soc card
    card.dev = &pdev.dev;
    rk_dailink.codecs.of_node = of_parse_phandle(np,
    "rockchip,audio-codec", 0);
    if (!rk_dailink.codecs.of_node) {
    dev_err(&pdev.dev,
    "Property 'rockchip,audio-codec' missing or invalid\n");
    return -EINVAL;
    }
    rk_dailink.cpus.of_node = of_parse_phandle(np,
    "rockchip,i2s-controller", 0);
    if (!rk_dailink.cpus.of_node) {
    dev_err(&pdev.dev,
    "Property 'rockchip,i2s-controller' missing or invalid\n");
    ret = -EINVAL;
    goto put_codec_of_node;
    }
    rk_dailink.platforms.of_node = rk_dailink.cpus.of_node;
    ret = snd_soc_of_parse_card_name(card, "rockchip,model");
    if (ret)
    goto put_cpu_of_node;
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret) {
    dev_err(&pdev.dev,
    "Soc register card failed %d\n", ret);
    goto put_cpu_of_node;
    }
    return ret;
    put_cpu_of_node:
    of_node_put(rk_dailink.cpus.of_node);
    rk_dailink.cpus.of_node = core::ptr::null_mut();
    put_codec_of_node:
    of_node_put(rk_dailink.codecs.of_node);
    rk_dailink.codecs.of_node = core::ptr::null_mut();
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn snd_rk_mc_remove(pdev: *mut platform_device) {
    static void snd_rk_mc_remove(struct platform_device *pdev)
    {
    of_node_put(rk_dailink.cpus.of_node);
    rk_dailink.cpus.of_node = core::ptr::null_mut();
    of_node_put(rk_dailink.codecs.of_node);
    rk_dailink.codecs.of_node = core::ptr::null_mut();
    }
    static const struct of_device_id rockchip_rt5645_of_match[] = {
    { .compatible = "rockchip,rockchip-audio-rt5645", },
    {},
    };
    MODULE_DEVICE_TABLE(of, rockchip_rt5645_of_match);
    static struct platform_driver snd_rk_mc_driver = {
    .probe = snd_rk_mc_probe,
    .remove = snd_rk_mc_remove,
    .driver = {
    .name = DRV_NAME,
    .pm = &snd_soc_pm_ops,
    .of_match_table = rockchip_rt5645_of_match,
    },
    };
    module_platform_driver(snd_rk_mc_driver);
    MODULE_AUTHOR("Xing Zheng <zhengxing@rock-chips.com>");
    MODULE_DESCRIPTION("Rockchip rt5645 machine ASoC driver");
    MODULE_LICENSE("GPL v2");
    MODULE_ALIAS("platform:" DRV_NAME);
