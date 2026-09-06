//! Automatically rewritten from C to Rust
//! Source: sound/soc/amd/acp-rt5645.c
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


// SPDX-License-Identifier: MIT
//
// Machine driver for AMD ACP Audio engine using Realtek RT5645 codec
//
// Copyright 2017 Advanced Micro Devices, Inc.
//
// This file is modified from rt288 machine driver
//

pub const CZ_PLAT_CLK: c_int = 24000000;
    static struct snd_soc_jack cz_jack;
    static struct snd_soc_jack_pin cz_jack_pins[] = {
    {
    .pin = "Headphones",
    .mask = SND_JACK_HEADPHONE,
    },
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    };
    static int cz_aif1_hw_params(struct snd_pcm_substream *substream,
    struct snd_pcm_hw_params *params)
    {
    let mut ret: c_int = 0;
    struct snd_soc_pcm_runtime *rtd = snd_soc_substream_to_rtd(substream);
    struct snd_soc_dai *codec_dai = snd_soc_rtd_to_codec(rtd, 0);
    ret = snd_soc_dai_set_pll(codec_dai, 0, RT5645_PLL1_S_MCLK,
    CZ_PLAT_CLK, params_rate(params) * 512);
    if (ret < 0) {
    dev_err(rtd.dev, "can't set codec pll: %d\n", ret);
    return ret;
    }
    ret = snd_soc_dai_set_sysclk(codec_dai, RT5645_SCLK_S_PLL1,
    params_rate(params) * 512, SND_SOC_CLOCK_OUT);
    if (ret < 0) {
    dev_err(rtd.dev, "can't set codec sysclk: %d\n", ret);
    return ret;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn cz_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int cz_init(struct snd_soc_pcm_runtime *rtd)
    {
    int ret;
    struct snd_soc_card *card;
    struct snd_soc_component *codec;
    codec = snd_soc_rtd_to_codec(rtd, 0).component;
    card = rtd.card;
    ret = snd_soc_card_jack_new_pins(card, "Headset Jack",
    SND_JACK_HEADPHONE | SND_JACK_MICROPHONE |
    SND_JACK_BTN_0 | SND_JACK_BTN_1 |
    SND_JACK_BTN_2 | SND_JACK_BTN_3,
    &cz_jack,
    cz_jack_pins,
    ARRAY_SIZE(cz_jack_pins));
    if (ret) {
    dev_err(card.dev, "HP jack creation failed %d\n", ret);
    return ret;
    }
    rt5645_set_jack_detect(codec, &cz_jack, &cz_jack, &cz_jack);
    return 0;
    }
    static const struct snd_soc_ops cz_aif1_ops = {
    .hw_params = cz_aif1_hw_params,
    };
    SND_SOC_DAILINK_DEF(designware1,
    DAILINK_COMP_ARRAY(COMP_CPU("designware-i2s.1")));
    SND_SOC_DAILINK_DEF(designware2,
    DAILINK_COMP_ARRAY(COMP_CPU("designware-i2s.2")));
    SND_SOC_DAILINK_DEF(codec,
    DAILINK_COMP_ARRAY(COMP_CODEC("i2c-10EC5650:00", "rt5645-aif1")));
    SND_SOC_DAILINK_DEF(platform,
    DAILINK_COMP_ARRAY(COMP_PLATFORM("acp_audio_dma.0")));
    static struct snd_soc_dai_link cz_dai_rt5650[] = {
    {
    .name = "amd-rt5645-play",
    .stream_name = "RT5645_AIF1",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
    | SND_SOC_DAIFMT_CBP_CFP,
    .init = cz_init,
    .ops = &cz_aif1_ops,
    SND_SOC_DAILINK_REG(designware1, codec, platform),
    },
    {
    .name = "amd-rt5645-cap",
    .stream_name = "RT5645_AIF1",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF
    | SND_SOC_DAIFMT_CBP_CFP,
    .ops = &cz_aif1_ops,
    SND_SOC_DAILINK_REG(designware2, codec, platform),
    },
    };
    static const struct snd_soc_dapm_widget cz_widgets[] = {
    SND_SOC_DAPM_HP("Headphones", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Speakers", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Int Mic", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route cz_audio_route[] = {
    {"Headphones", core::ptr::null_mut(), "HPOL"},
    {"Headphones", core::ptr::null_mut(), "HPOR"},
    {"RECMIXL", core::ptr::null_mut(), "Headset Mic"},
    {"RECMIXR", core::ptr::null_mut(), "Headset Mic"},
    {"Speakers", core::ptr::null_mut(), "SPOL"},
    {"Speakers", core::ptr::null_mut(), "SPOR"},
    {"DMIC L2", core::ptr::null_mut(), "Int Mic"},
    {"DMIC R2", core::ptr::null_mut(), "Int Mic"},
    };
    static const struct snd_kcontrol_new cz_mc_controls[] = {
    SOC_DAPM_PIN_SWITCH("Headphones"),
    SOC_DAPM_PIN_SWITCH("Speakers"),
    SOC_DAPM_PIN_SWITCH("Headset Mic"),
    SOC_DAPM_PIN_SWITCH("Int Mic"),
    };
    static struct snd_soc_card cz_card = {
    .name = "acprt5650",
    .owner = THIS_MODULE,
    .dai_link = cz_dai_rt5650,
    .num_links = ARRAY_SIZE(cz_dai_rt5650),
    .dapm_widgets = cz_widgets,
    .num_dapm_widgets = ARRAY_SIZE(cz_widgets),
    .dapm_routes = cz_audio_route,
    .num_dapm_routes = ARRAY_SIZE(cz_audio_route),
    .controls = cz_mc_controls,
    .num_controls = ARRAY_SIZE(cz_mc_controls),
    };
#[no_mangle]
unsafe extern "C" fn cz_probe(pdev: *mut platform_device) -> c_int {
    static int cz_probe(struct platform_device *pdev)
    {
    int ret;
    struct snd_soc_card *card;
    card = &cz_card;
    cz_card.dev = &pdev.dev;
    platform_set_drvdata(pdev, card);
    ret = devm_snd_soc_register_card(&pdev.dev, &cz_card);
    if (ret) {
    dev_err(&pdev.dev,
    "devm_snd_soc_register_card(%s) failed: %d\n",
    cz_card.name, ret);
    return ret;
    }
    return 0;
    }

    static const struct acpi_device_id cz_audio_acpi_match[] = {
    { "AMDI1002", 0 },
    {},
    };
    MODULE_DEVICE_TABLE(acpi, cz_audio_acpi_match);

    static struct platform_driver cz_pcm_driver = {
    .driver = {
    .name = "cz-rt5645",
    .acpi_match_table = ACPI_PTR(cz_audio_acpi_match),
    .pm = &snd_soc_pm_ops,
    },
    .probe = cz_probe,
    };
    module_platform_driver(cz_pcm_driver);
    MODULE_AUTHOR("akshu.agrawal@amd.com");
    MODULE_DESCRIPTION("cz-rt5645 audio support");
    MODULE_LICENSE("GPL v2");
