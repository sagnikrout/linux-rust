//! Automatically rewritten from C to Rust
//! Source: sound/soc/codecs/nau8315.c
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
// nau8315.c  --  NAU8315 ALSA SoC Audio Amplifier Driver
//
// Copyright 2020 Nuvoton Technology Crop.
//
// Author: David Lin <ctlin0@nuvoton.com>
//
// Based on MAX98357A.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nau8315_priv {
    pub enable: *mut gpio_desc,
    pub enpin_switch: c_int,
}

    static int nau8315_daiops_trigger(struct snd_pcm_substream *substream,
    int cmd, struct snd_soc_dai *dai)
    {
    struct snd_soc_component *component = dai.component;
    struct nau8315_priv *nau8315 =
    snd_soc_component_get_drvdata(component);
    if (!nau8315.enable)
    return 0;
    switch (cmd) {
    case SNDRV_PCM_TRIGGER_START:
    case SNDRV_PCM_TRIGGER_RESUME:
    case SNDRV_PCM_TRIGGER_PAUSE_RELEASE:
    if (nau8315.enpin_switch) {
    gpiod_set_value(nau8315.enable, 1);
    dev_dbg(component.dev, "set enable to 1");
    }
    break;
    case SNDRV_PCM_TRIGGER_STOP:
    case SNDRV_PCM_TRIGGER_SUSPEND:
    case SNDRV_PCM_TRIGGER_PAUSE_PUSH:
    gpiod_set_value(nau8315.enable, 0);
    dev_dbg(component.dev, "set enable to 0");
    break;
    }
    return 0;
    }
    static int nau8315_enpin_event(struct snd_soc_dapm_widget *w,
    struct snd_kcontrol *kcontrol, int event)
    {
    struct snd_soc_component *component =
    snd_soc_dapm_to_component(w.dapm);
    struct nau8315_priv *nau8315 =
    snd_soc_component_get_drvdata(component);
    if (event & SND_SOC_DAPM_PRE_PMU)
    nau8315.enpin_switch = 1;
#[no_mangle]
pub unsafe extern "C" fn if(SND_SOC_DAPM_POST_PMD: event &) -> else {
    else if (event & SND_SOC_DAPM_POST_PMD)
    nau8315.enpin_switch = 0;
    return 0;
    }
    static const struct snd_soc_dapm_widget nau8315_dapm_widgets[] = {
    SND_SOC_DAPM_OUTPUT("Speaker"),
    SND_SOC_DAPM_OUT_DRV_E("EN_Pin", SND_SOC_NOPM, 0, 0, core::ptr::null_mut(), 0,
    nau8315_enpin_event,
    SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    };
    static const struct snd_soc_dapm_route nau8315_dapm_routes[] = {
    {"EN_Pin", core::ptr::null_mut(), "HiFi Playback"},
    {"Speaker", core::ptr::null_mut(), "EN_Pin"},
    };
    static const struct snd_soc_component_driver nau8315_component_driver = {
    .dapm_widgets		= nau8315_dapm_widgets,
    .num_dapm_widgets	= ARRAY_SIZE(nau8315_dapm_widgets),
    .dapm_routes		= nau8315_dapm_routes,
    .num_dapm_routes	= ARRAY_SIZE(nau8315_dapm_routes),
    .idle_bias_on		= 1,
    .use_pmdown_time	= 1,
    .endianness		= 1,
    };
    static const struct snd_soc_dai_ops nau8315_dai_ops = {
    .trigger	= nau8315_daiops_trigger,
    };

    static struct snd_soc_dai_driver nau8315_dai_driver = {
    .name = "nau8315-hifi",
    .playback = {
    .stream_name	= "HiFi Playback",
    .formats	= NAU8315_FORMATS,
    .rates		= NAU8315_RATES,
    .channels_min	= 1,
    .channels_max	= 2,
    },
    .ops    = &nau8315_dai_ops,
    };
#[no_mangle]
unsafe extern "C" fn nau8315_platform_probe(pdev: *mut platform_device) -> c_int {
    static int nau8315_platform_probe(struct platform_device *pdev)
    {
    struct nau8315_priv *nau8315;
    nau8315 = devm_kzalloc(&pdev.dev, sizeof(*nau8315), GFP_KERNEL);
    if (!nau8315)
    return -ENOMEM;
    nau8315.enable = devm_gpiod_get_optional(&pdev.dev,
    "enable", GPIOD_OUT_LOW);
    if (IS_ERR(nau8315.enable))
    return PTR_ERR(nau8315.enable);
    dev_set_drvdata(&pdev.dev, nau8315);
    return devm_snd_soc_register_component(&pdev.dev,
    &nau8315_component_driver,
    &nau8315_dai_driver, 1);
    }

    static const struct of_device_id nau8315_device_id[] = {
    { .compatible = "nuvoton,nau8315" },
    { .compatible = "nuvoton,nau8318" },
    {}
    };
    MODULE_DEVICE_TABLE(of, nau8315_device_id);

    static const struct acpi_device_id nau8315_acpi_match[] = {
    { "NVTN2010", 0 },
    { "NVTN2012", 0},
    {},
    };
    MODULE_DEVICE_TABLE(acpi, nau8315_acpi_match);

    static struct platform_driver nau8315_platform_driver = {
    .driver = {
    .name = "nau8315",
    .of_match_table = of_match_ptr(nau8315_device_id),
    .acpi_match_table = ACPI_PTR(nau8315_acpi_match),
    },
    .probe	= nau8315_platform_probe,
    };
    module_platform_driver(nau8315_platform_driver);
    MODULE_DESCRIPTION("ASoC NAU8315 Mono Class-D Amplifier Driver");
    MODULE_AUTHOR("David Lin <ctlin0@nuvoton.com>");
    MODULE_LICENSE("GPL v2");
