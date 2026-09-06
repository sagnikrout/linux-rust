//! Automatically rewritten from C to Rust
//! Source: sound/soc/samsung/lowland.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// Lowland audio support
//
// Copyright 2011 Wolfson Microelectronics

    static struct snd_soc_jack lowland_headset;
// Headset jack detection DAPM pins
    static struct snd_soc_jack_pin lowland_headset_pins[] = {
    {
    .pin = "Headphone",
    .mask = SND_JACK_HEADPHONE,
    },
    {
    .pin = "Headset Mic",
    .mask = SND_JACK_MICROPHONE,
    },
    {
    .pin = "Line Out",
    .mask = SND_JACK_LINEOUT,
    },
    };
#[no_mangle]
unsafe extern "C" fn lowland_wm5100_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int lowland_wm5100_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_component *component = snd_soc_rtd_to_codec(rtd, 0).component;
    int ret;
    ret = snd_soc_component_set_sysclk(component, WM5100_CLK_SYSCLK,
    WM5100_CLKSRC_MCLK1, MCLK1_RATE,
    SND_SOC_CLOCK_IN);
    if (ret < 0) {
    pr_err("Failed to set SYSCLK clock source: %d\n", ret);
    return ret;
    }
// Clock OPCLK, used by the other audio components.
    ret = snd_soc_component_set_sysclk(component, WM5100_CLK_OPCLK, 0,
    CLKOUT_RATE, 0);
    if (ret < 0) {
    pr_err("Failed to set OPCLK rate: %d\n", ret);
    return ret;
    }
    ret = snd_soc_card_jack_new_pins(rtd.card, "Headset",
    SND_JACK_LINEOUT | SND_JACK_HEADSET |
    SND_JACK_BTN_0,
    &lowland_headset, lowland_headset_pins,
    ARRAY_SIZE(lowland_headset_pins));
    if (ret)
    return ret;
    wm5100_detect(component, &lowland_headset);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn lowland_wm9081_init(rtd: *mut snd_soc_pcm_runtime) -> c_int {
    static int lowland_wm9081_init(struct snd_soc_pcm_runtime *rtd)
    {
    struct snd_soc_component *component = snd_soc_rtd_to_codec(rtd, 0).component;
    struct snd_soc_dapm_context *dapm = snd_soc_card_to_dapm(rtd.card);
    snd_soc_dapm_disable_pin(dapm, "LINEOUT");
// At any time the WM9081 is active it will have this clock
    return snd_soc_component_set_sysclk(component, WM9081_SYSCLK_MCLK, 0,
    CLKOUT_RATE, 0);
    }
    static const struct snd_soc_pcm_stream sub_params = {
    .formats = SNDRV_PCM_FMTBIT_S32_LE,
    .rate_min = 44100,
    .rate_max = 44100,
    .channels_min = 2,
    .channels_max = 2,
    };
    SND_SOC_DAILINK_DEFS(cpu,
    DAILINK_COMP_ARRAY(COMP_CPU("samsung-i2s.0")),
    DAILINK_COMP_ARRAY(COMP_CODEC("wm5100.1-001a", "wm5100-aif1")),
    DAILINK_COMP_ARRAY(COMP_PLATFORM("samsung-i2s.0")));
    SND_SOC_DAILINK_DEFS(baseband,
    DAILINK_COMP_ARRAY(COMP_CPU("wm5100-aif2")),
    DAILINK_COMP_ARRAY(COMP_CODEC("wm1250-ev1.1-0027", "wm1250-ev1")));
    SND_SOC_DAILINK_DEFS(speaker,
    DAILINK_COMP_ARRAY(COMP_CPU("wm5100-aif3")),
    DAILINK_COMP_ARRAY(COMP_CODEC("wm9081.1-006c", "wm9081-hifi")));
    static struct snd_soc_dai_link lowland_dai[] = {
    {
    .name = "CPU",
    .stream_name = "CPU",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBP_CFP,
    .init = lowland_wm5100_init,
    SND_SOC_DAILINK_REG(cpu),
    },
    {
    .name = "Baseband",
    .stream_name = "Baseband",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    SND_SOC_DAILINK_REG(baseband),
    },
    {
    .name = "Sub Speaker",
    .stream_name = "Sub Speaker",
    .dai_fmt = SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_NB_NF |
    SND_SOC_DAIFMT_CBP_CFP,
    .ignore_suspend = 1,
    .c2c_params = &sub_params,
    .num_c2c_params = 1,
    .init = lowland_wm9081_init,
    SND_SOC_DAILINK_REG(speaker),
    },
    };
    static struct snd_soc_codec_conf lowland_codec_conf[] = {
    {
    .dlc = COMP_CODEC_CONF("wm9081.1-006c"),
    .name_prefix = "Sub",
    },
    };
    static const struct snd_kcontrol_new controls[] = {
    SOC_DAPM_PIN_SWITCH("Main Speaker"),
    SOC_DAPM_PIN_SWITCH("Main DMIC"),
    SOC_DAPM_PIN_SWITCH("Main AMIC"),
    SOC_DAPM_PIN_SWITCH("WM1250 Input"),
    SOC_DAPM_PIN_SWITCH("WM1250 Output"),
    SOC_DAPM_PIN_SWITCH("Headphone"),
    SOC_DAPM_PIN_SWITCH("Line Out"),
    };
    static const struct snd_soc_dapm_widget widgets[] = {
    SND_SOC_DAPM_HP("Headphone", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Headset Mic", core::ptr::null_mut()),
    SND_SOC_DAPM_LINE("Line Out", core::ptr::null_mut()),
    SND_SOC_DAPM_SPK("Main Speaker", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Main AMIC", core::ptr::null_mut()),
    SND_SOC_DAPM_MIC("Main DMIC", core::ptr::null_mut()),
    };
    static const struct snd_soc_dapm_route audio_paths[] = {
    { "Sub IN1", core::ptr::null_mut(), "HPOUT2L" },
    { "Sub IN2", core::ptr::null_mut(), "HPOUT2R" },
    { "Main Speaker", core::ptr::null_mut(), "Sub SPKN" },
    { "Main Speaker", core::ptr::null_mut(), "Sub SPKP" },
    { "Main Speaker", core::ptr::null_mut(), "SPKDAT1" },
    };
    static struct snd_soc_card lowland = {
    .name = "Lowland",
    .owner = THIS_MODULE,
    .dai_link = lowland_dai,
    .num_links = ARRAY_SIZE(lowland_dai),
    .codec_conf = lowland_codec_conf,
    .num_configs = ARRAY_SIZE(lowland_codec_conf),
    .controls = controls,
    .num_controls = ARRAY_SIZE(controls),
    .dapm_widgets = widgets,
    .num_dapm_widgets = ARRAY_SIZE(widgets),
    .dapm_routes = audio_paths,
    .num_dapm_routes = ARRAY_SIZE(audio_paths),
    };
#[no_mangle]
unsafe extern "C" fn lowland_probe(pdev: *mut platform_device) -> c_int {
    static int lowland_probe(struct platform_device *pdev)
    {
    struct snd_soc_card *card = &lowland;
    int ret;
    card.dev = &pdev.dev;
    ret = devm_snd_soc_register_card(&pdev.dev, card);
    if (ret)
    dev_err_probe(&pdev.dev, ret, "snd_soc_register_card() failed\n");
    return ret;
    }
    static struct platform_driver lowland_driver = {
    .driver = {
    .name = "lowland",
    .pm = &snd_soc_pm_ops,
    },
    .probe = lowland_probe,
    };
    module_platform_driver(lowland_driver);
    MODULE_DESCRIPTION("Lowland audio support");
    MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
    MODULE_LICENSE("GPL");
    MODULE_ALIAS("platform:lowland");
